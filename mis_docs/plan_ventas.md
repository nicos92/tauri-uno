# Plan: Módulo de Ventas + Permiso "Vender sin stock"

## Objetivo

Registrar ventas de artículos que existen en la tabla `stock`, descontando las
cantidades vendidas, con opción de imprimir un presupuesto del carrito en PDF y
anular ventas restaurando el stock. Todo queda registrado en auditoría.

## Decisiones confirmadas

- **Sin clientes**: la venta registra únicamente el usuario que la genera, la
  fecha, el detalle de artículos y el total.
- **Solo artículos con stock**: el selector de la venta y la validación del backend
  restringen los items a artículos que tienen fila en la tabla `stock`.
- **Nuevo permiso `vender_sin_stock`**: si el usuario lo tiene, se permite vender
  aunque la cantidad supere el stock disponible (el stock puede quedar negativo).
  Sin el permiso, se valida `stock.cantidad >= item.cantidad`.
- **Presupuesto en PDF**: botón en el carrito (mientras se arma la venta) que
  imprime la lista actual mediante HTML + diálogo de impresión del sistema
  (`window.print()`), habilitado por el permiso `generar_presupuesto`. No se
  persiste.
- **Anular venta**: restaura las cantidades al stock, marca la venta como
  `anulada` y registra la acción en auditoría. No se elimina.
- **Snapshot de precios**: se guardan `costo_unitario` y `precio_unitario` en el
  detalle al momento de vender, para que cambios posteriores de costo/ganancia no
  alteren ventas históricas.

## Permisos nuevos (sync en 3 lugares + UI)

| Rust `PermissionCode` | TS `PERMISSIONS` | String |
|---|---|---|
| `ViewVentas` | `VIEW_VENTAS` | `ver_ventas` |
| `CreateVenta` | `CREATE_VENTA` | `crear_venta` |
| `AnularVenta` | `ANULAR_VENTA` | `anular_venta` |
| `VenderSinStock` | `VENDER_SIN_STOCK` | `vender_sin_stock` |
| `GenerarPresupuesto` | `GENERAR_PRESUPUESTO` | `generar_presupuesto` |

- Rust: `src-tauri/src/domain/entities/permission_code.rs` (enum, `as_str()`, `all()`).
- TS: `src/domain/entities/permissions.ts`.
- Seed: lista `PERMISSIONS` en `src-tauri/src/infrastructure/database/mod.rs`
  (`INSERT OR IGNORE`, el admin los recibe automáticamente).
- `src/presentation/composables/usePermissions.ts`: `canViewVentas`,
  `canCreateVenta`, `canAnularVenta`, `canVenderSinStock`, `canGenerarPresupuesto`.

## Arquitectura

Sigue la Clean Architecture del proyecto (domain / application / infrastructure /
api), con entidad, trait de repositorio, implementación SQLite, servicio y comandos
Tauri. Archivos Rust nuevos en `snake_case`.

## Backend (Rust)

### 1. Esquema DB — `src-tauri/src/infrastructure/database/mod.rs`

```sql
CREATE TABLE IF NOT EXISTS ventas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id),
    fecha TEXT NOT NULL,
    total REAL NOT NULL,
    anulada INTEGER NOT NULL DEFAULT 0,
    observacion TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS venta_detalle (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    id_venta INTEGER NOT NULL REFERENCES ventas(id) ON DELETE CASCADE,
    id_articulo INTEGER NOT NULL REFERENCES articulos(id),
    cantidad REAL NOT NULL,
    costo_unitario REAL NOT NULL,
    precio_unitario REAL NOT NULL,
    subtotal REAL NOT NULL
);
```

- Tablas nuevas → se crean en DBs existentes sin migraciones.
- Agregar los 5 permisos al array `PERMISSIONS` del seed.

### 2. Entidad — `src-tauri/src/domain/entities/venta.rs` (nuevo)

- `Venta { id, user_id, fecha, total, anulada, observacion, created_at }`.
- `VentaDetalle { id, id_venta, id_articulo, cantidad, costo_unitario,
  precio_unitario, subtotal }`.
- `VentaDetalleConArticulo` = detalle + `cod_articulo`, `articulo`.
- `VentaWithDetalle { venta, username, items: Vec<VentaDetalleConArticulo> }`.
- Registrar en `src-tauri/src/domain/entities/mod.rs`.

### 3. Repositorio

- Trait `VentaRepository` — `src-tauri/src/domain/repositories/venta_repository.rs`:
  - `create(&self, venta: &Venta, detalles: &[VentaDetalle], allow_negative_stock: bool) -> Result<VentaWithDetalle, AppError>`
  - `find_by_id(&self, id: i64) -> Result<Option<VentaWithDetalle>, AppError>`
  - `find_all(&self) -> Result<Vec<VentaWithDetalle>, AppError>`
  - `anular(&self, id: i64) -> Result<(), AppError>`
- Implementación `SqliteVentaRepository` —
  `src-tauri/src/infrastructure/repositories/venta_repository.rs`.
  Cada método **bloquea el `Mutex<Connection>` global una sola vez** y corre la
  operación completa en una transacción (`BEGIN`/`COMMIT`), porque una venta =
  insertar cabecera + items + decremento de stock (todo atómico).
- Registrar ambos en sus `mod.rs`.

**Lógica de `create` (dentro de la transacción):**
1. Por cada item: leer la fila de `stock` de `id_articulo`. Si no existe →
   `ArticuloWithoutStock`.
2. Si `!allow_negative_stock` y `stock.cantidad < item.cantidad` →
   `InsufficientStock`.
3. Snapshot: `costo_unitario = stock.costo`,
   `precio_unitario = stock.costo * (1 + stock.ganancia / 100)` (se permite
   sobreescribir `precio_unitario` desde el request, default al calculado).
4. `INSERT ventas` (fecha = now, `anulada = 0`, total = suma de subtotales),
   obtener `id`, `INSERT` cada `venta_detalle`.
5. `UPDATE stock SET cantidad = cantidad - item.cantidad WHERE id_articulo`.
6. `COMMIT`. Si falla algo → `ROLLBACK` y propagar error.

**Lógica de `anular` (dentro de la transacción):**
1. Leer la venta. Si no existe → `VentaNotFound`. Si `anulada = 1` →
   `VentaAlreadyAnulada`.
2. Por cada detalle: `UPDATE stock SET cantidad = cantidad + detalle.cantidad
   WHERE id_articulo`. Si la fila de stock no existe → `ArticuloWithoutStock`
   (no se recrea; el operador debe crear stock antes de anular).
3. `UPDATE ventas SET anulada = 1`.
4. `COMMIT`.

### 4. Servicio — `src-tauri/src/application/services/venta_service.rs`

- `VentaService` con `create(venta, detalles, allow_negative_stock)`, `get_all`,
  `get_by_id`, `anular(id)`. Delega en el repositorio.
- Registrar en `src-tauri/src/application/services/mod.rs`.

### 5. Errores nuevos — `src-tauri/src/infrastructure/error.rs`

| Variante | `code()` | `user_message()` |
|---|---|---|
| `VentaNotFound` | `venta_not_found` | "La venta no existe." |
| `VentaAlreadyAnulada` | `venta_already_anulada` | "La venta ya fue anulada." |
| `InsufficientStock` | `insufficient_stock` | "Stock insuficiente para uno de los artículos." |
| `ArticuloWithoutStock` | `articulo_without_stock` | "Uno de los artículos no tiene stock registrado." |

### 6. Audit — `src-tauri/src/domain/entities/audit_log.rs`

- Agregar `AuditScreen::Ventas` → `as_str()` `"Ventas"`.

### 7. Permiso — `src-tauri/src/domain/entities/permission_code.rs`

- Agregar las 5 variantes nuevas a `as_str()` y `all()`.

### 8. Commands — `src-tauri/src/api/commands/venta_commands.rs` (nuevo)

- `VentaAppState { venta_service: Mutex<VentaService> }`.
- `CreateVentaRequest { items: Vec<CreateVentaDetalleRequest>, observacion }` con
  `CreateVentaDetalleRequest { id_articulo, cantidad, precio_unitario? }`.
- `create_venta(user_id, request, state)`:
  - `check_permission(CreateVenta)`.
  - `allow_negative_stock = check_permission(VenderSinStock).is_ok()`.
  - `log_audit(Ventas, Create, "Venta (id N)")` **después** de que el repo libere
    el Mutex (evita deadlock del Mutex no reentrante).
- `get_all_ventas(user_id, state)` → `check_permission(ViewVentas)`.
- `get_venta_by_id(user_id, id, state)` → `check_permission(ViewVentas)`.
- `anular_venta(user_id, id, state)` → `check_permission(AnularVenta)` +
  `log_audit(Ventas, Update, "Venta (id N) anulada")`.
- Registrar en `src-tauri/src/api/commands/mod.rs`.

### 9. Registro — `src-tauri/src/lib.rs`

- `.manage(VentaAppState::new())` + comandos en `tauri::generate_handler!` +
  imports.

## Frontend (Vue)

- `src/domain/entities/permissions.ts`: agregar las 5 claves.
- `src/domain/entities/types.ts`: interfaces `Venta`, `VentaDetalle`,
  `VentaDetalleConArticulo`, `VentaWithDetalle`, `CreateVentaDetalleRequest`,
  `CreateVentaRequest`.
- `src/infrastructure/api/ventaRepository.ts` (nuevo): `VentasApiRepository` con
  `getAllVentas`, `getVentaById`, `createVenta`, `anularVenta` (patrón
  `stockRepository`, pasando `userId`).
- `src/presentation/stores/index.ts`: `useVentasStore` con `ventas`, `loading`,
  `error`, `fetchVentas`, `getVentaById`, `createVenta`, `anularVenta`; tras crear
  o anular, refrescar `useStockStore` para reflejar las cantidades.
- `src/presentation/composables/usePermissions.ts`: nuevas funciones `can*`.
- `src/presentation/pages/VentasPage.vue` (nuevo):
  - Tabla de ventas: N°, fecha, usuario, items, total, estado (activa/anulada),
    acciones (ver, anular si `canAnularVenta` y no anulada).
  - Modal "Nueva Venta" (`canCreateVenta`): carrito con búsqueda/select de
    artículos de la tabla de stock (reusa `stockStore` + `articulosStore`), línea
    con cantidad y precio editable (default precio de venta), subtotal y total.
  - Botón **"Generar PDF"** (`canGenerarPresupuesto`): renderiza el carrito en una
    sección imprimible oculta y llama `window.print()` con `@media print`.
  - Modal detalle con los items de una venta.
  - Anular con confirmación → restaura stock y refresca la lista.
- `src/presentation/router/index.ts`: ruta `ventas` con
  `meta: { permission: PERMISSIONS.VIEW_VENTAS }`.
- `src/presentation/layouts/MainLayout.vue`: item de menú "Ventas" con
  `permission: "ver_ventas"` + ícono nuevo `src/public/svg/ventas.svg`.
- `src/presentation/pages/AuditoriaPage.vue`: agregar `"Ventas"` al array `screens`.

## Verificación

- `cd src-tauri && cargo check` (+ `cargo clippy` si aplica).
- `pnpm build` (vue-tsc + vite build).
- Prueba manual:
  - Crear venta sin permiso `vender_sin_stock` con cantidad > stock → error
    "Stock insuficiente...".
  - Con permiso → la venta se crea y el stock queda negativo.
  - Anular → el stock vuelve al valor original y la venta figura anulada.
  - Artículo sin fila de stock → no se puede vender.
  - Generar PDF desde el carrito → diálogo de impresión.

## Notas

- **Deadlock**: `log_audit` se invoca siempre fuera de la transacción (el
  `Mutex<Connection>` global no es reentrante).
- **Dinero en `f64`**: consistente con el proyecto (stock usa `REAL`); no se
  cambia a centavos en este alcance.
- **Anular con stock borrado**: no se recrea la fila; se devuelve
  `ArticuloWithoutStock` para que el operador regularice el stock primero.
- Archivos Rust nuevos en `snake_case`; no se renombran los existentes.
