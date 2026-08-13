# Plan: Gestión de Presupuestos (parte 2)

## Objetivo

Crear el módulo dedicado a la Gestión de Presupuestos: listado con filtros,
detalle, anulación (soft-delete con estado `anulado`) y conversión a venta
cargando los ítems del presupuesto en el carrito de "Nueva Venta".

## Decisiones confirmadas

- **Anulación = soft-delete**: se agrega el estado `anulado` al `CHECK` de la
  tabla `presupuestos`. Requiere una migración idempotente que reconstruye la
  tabla si ya existe (conserva datos y el FK de `detalle_presupuestos`).
- **Permiso único**: todo el módulo (ver, anular, convertir) usa el permiso
  existente `generar_presupuesto`. No se agregan permisos nuevos (no se toca
  la sincronización Rust/TS/seed).
- **Conversión vía carrito**: "Convertir a venta" navega a Nueva Venta con
  `?presupuesto_id=N`, precargando el carrito. Al registrar la venta con éxito
  se marca el presupuesto como `convertido`. No hay transacción de conversión
  directa en backend.
- **Regla de negocio**: un presupuesto en estado `convertido` o `anulado` es
  terminal; no puede volver a cambiar de estado ni convertirse en venta.
- **Stock insuficiente**: al cargar el carrito se advierte con el warning
  existente (cantidad vs. stock disponible); la venta respeta el permiso
  `vender_sin_stock` como cualquier venta normal.

## 1. Base de datos (migración)

`src-tauri/src/infrastructure/database/mod.rs`

1. En `apply_schema`, actualizar el `CHECK` de `CREATE TABLE IF NOT EXISTS
   presupuestos` para incluir `'anulado'`:

   ```sql
   estado TEXT NOT NULL DEFAULT 'pendiente'
       CHECK (estado IN ('pendiente','aprobado','vencido','convertido','anulado')),
   ```

2. Nueva función `migrate_presupuestos_estado(conn) -> Result<(), rusqlite::Error>`
   llamada al final de `apply_schema` (después de `execute_batch` y los índices):

   - Leer `SELECT sql FROM sqlite_master WHERE type='table' AND name='presupuestos'`.
   - Si el SQL ya contiene `'anulado'` → no hacer nada (idempotente, cubre DB
     nuevas y tests).
   - Si no: reconstruir con `PRAGMA foreign_keys = OFF`:
     1. `CREATE TABLE presupuestos_new (...)` con el mismo schema + `'anulado'`.
     2. `INSERT INTO presupuestos_new SELECT id, user_id, fecha, total,
        descuento, estado, fecha_vencimiento, observacion, cliente_id,
        created_at FROM presupuestos;`
     3. `DROP TABLE presupuestos;`
     4. `ALTER TABLE presupuestos_new RENAME TO presupuestos;`
     5. Recrear `idx_detalle_presupuestos_id_presupuesto` e
        `idx_presupuestos_estado`.
     6. `PRAGMA foreign_keys = ON;`

   Con FK apagado durante el `DROP`, `detalle_presupuestos` no pierde filas
   (los ids se preservan y el FK referencia la tabla por nombre).

## 2. Entidad

`src-tauri/src/domain/entities/presupuesto.rs`

- Agregar variante `Anulado` a `PresupuestoEstado` con
  `as_str() => "anulado"`.
- `PresupuestoWithDetalle.estado` ya es `String`; no requiere cambio.

## 3. Errores

`src-tauri/src/infrastructure/error.rs`

- Nuevo variante `PresupuestoEstadoInvalido`:
  - `#[error("Presupuesto estado inválido")]`
  - `code()` → `"presupuesto_estado_invalido"`
  - `user_message()` → "El presupuesto no está en un estado que permita esa
    operación."
- Agregar asserts en los tests existentes (`code_returns_expected_strings`,
  `user_message_returns_spanish_messages`).

## 4. Repositorio (trait)

`src-tauri/src/domain/repositories/presupuesto_repository.rs`

```rust
pub struct PresupuestoFilter {
    pub estado: Option<PresupuestoEstado>,
    pub fecha_desde: Option<String>,
    pub fecha_hasta: Option<String>,
    pub query: Option<String>,
}
```

- Cambiar `find_page` a `find_page(&self, filter: &PresupuestoFilter, limit, offset)`.
- Agregar `update_estado(&self, id: i64, estado: PresupuestoEstado) -> Result<(), AppError>`.

## 5. Repositorio (SQLite)

`src-tauri/src/infrastructure/repositories/presupuesto_repository.rs`

- `find_page` con WHERE dinámico (parametrizado, combinando con `AND`):
  - `estado` → `p.estado = ?`
  - `fecha_desde` → `p.fecha >= ?`
  - `fecha_hasta` → `p.fecha <= ?`
  - `query` → `LIKE` sobre `p.id`, `c.nombre`, `c.apellido`, `u.username`,
    `a.articulo`, `a.cod_articulo` (usar subquery en `detalle_presupuestos`).
  - Aplicar los mismos filtros al `SELECT COUNT(*)`.
- `update_estado`:
  - Leer `estado` actual de `presupuestos WHERE id = ?` (no existe → `PresupuestoNotFound`).
  - Si actual es `convertido` o `anulado` → `AppError::PresupuestoEstadoInvalido`.
  - Si no → `UPDATE presupuestos SET estado = ? WHERE id = ?`.
- Tests nuevos: filtros por estado/rango/query; `update_estado` pendiente→anulado
  OK, convertido→anulado error, id inexistente error.

## 6. Servicio

`src-tauri/src/application/services/presupuesto_service.rs`

- `get_page(filter: &PresupuestoFilter, limit, offset)` → delega en repo.
- `cambiar_estado(id, estado)` → matriz de transición (delegando al repo la
  guarda de terminales):
  - `convertido` / `anulado` → cualquier estado: error (terminal).
  - Desde `pendiente`/`aprobado`/`vencido`: permitir pasar a cualquier estado
    no terminal (`pendiente`, `aprobado`, `vencido`, `anulado`, `convertido`).
- Tests unitarios con `MockPresupuestoRepository` (trait con `automock`).

## 7. Commands (Tauri)

`src-tauri/src/api/commands/presupuesto_commands.rs`

- Extender `GetPresupuestosRequest`:

  ```rust
  pub struct GetPresupuestosRequest {
      pub limit: Option<i64>,
      pub offset: Option<i64>,
      pub estado: Option<PresupuestoEstado>,
      pub fecha_desde: Option<String>,
      pub fecha_hasta: Option<String>,
      pub query: Option<String>,
  }
  ```

  `get_all_presupuestos` construye `PresupuestoFilter` y llama `get_page(filter, limit, offset)`.

- Nuevo comando `cambiar_estado_presupuesto`:

  ```rust
  pub struct CambiarEstadoPresupuestoRequest {
      pub id: i64,
      pub estado: PresupuestoEstado,
  }
  ```

  Con `check_permission(user_id, PermissionCode::GenerarPresupuesto)` y
  `log_audit(user_id, AuditScreen::Presupuestos, AuditAction::Update, ...)`.

`src-tauri/src/api/commands/mod.rs` y `src-tauri/src/lib.rs`:

- Re-exportar `cambiar_estado_presupuesto` y registrarlo en
  `tauri::generate_handler!`.

## 8. Tipos (frontend)

`src/domain/entities/types.ts`

- `PresupuestoEstado` += `"anulado"`.
- `PresupuestoFilter { estado?: PresupuestoEstado; fecha_desde?: string; fecha_hasta?: string; query?: string }`.
- `GetPresupuestosRequest { limit: number; offset: number; filter?: PresupuestoFilter }`.
- `CambiarEstadoPresupuestoRequest { id: number; estado: PresupuestoEstado }`.

## 9. Repositorio API (frontend)

`src/infrastructure/api/presupuestoRepository.ts`

- `getAllPresupuestos(filters)` → enviar `{ limit, offset, ...filtros }`.
- `cambiarEstadoPresupuesto(id, estado)` → `invoke("cambiar_estado_presupuesto", ...)`.
- `getPresupuestoById` ya existe.

## 10. Store

`src/presentation/stores/index.ts` (sección `usePresupuestosStore`, ~línea 1136)

- Estado: `presupuestos`, `loading`, `error`, `total`, `limit`, `offset`.
- `fetchPresupuestos(filters?)` → página con filtros (actualiza estado).
- `getPresupuestoById(id)` → devuelve `PresupuestoWithDetalle | null`.
- `cambiarEstado(id, estado)` → llama al repo y recarga la lista.

## 11. Página Presupuestos

`src/presentation/pages/PresupuestosPage.vue` (nuevo, patrón `VentasPage.vue`)

- Header "Gestión de Presupuestos".
- Filtros: select de estado (todos, pendiente, aprobado, vencido, convertido,
  anulado), inputs `type="date"` desde/hasta, input de término (N°, cliente,
  usuario, artículo).
- Tabla: N°, fecha, vencimiento, cliente, usuario, items, subtotal, desc., total,
  estado (badge), acciones.
- Modal de detalle (artículos, subtotal/descuento/total, cliente, vencimiento,
  observación).
- Paginación (mismo patrón que `VentasPage.vue`).
- Acciones:
  - **Convertir a venta**: confirm → `router.push({ name: "nueva-venta", query: { presupuesto_id: id } })`. Oculto/deshabilitado si `convertido`/`anulado`.
  - **Anular**: confirm → `cambiarEstado(id, "anulado")` con toast de éxito/error.
- Reglas de negocio en UI: estado terminal → sin acciones de conversión/anulación.

## 12. Router y sidebar

- `src/presentation/router/index.ts`: ruta `presupuestos` →
  `PresupuestosPage.vue` con `meta: { permission: PERMISSIONS.GENERAR_PRESUPUESTO }`.
- `src/presentation/layouts/MainLayout.vue`: ítem "Presupuestos" (después de
  "Ventas"), `permission: "generar_presupuesto"`, icono reusando `card.svg`
  (no hay `presupuesto.svg` en `public/svg`).

## 13. Nueva Venta — precarga desde presupuesto

`src/presentation/pages/NuevaVentaPage.vue`

- Leer `route.query.presupuesto_id` (con `useRoute`).
- En `onMounted`, si hay id y `canGenerarPresupuesto()`:
  - `getPresupuestoById(id)`.
  - Si estado es `convertido`/`anulado` → toast de error y no cargar.
  - Si no: llenar `cart` desde `items` (id_articulo, cantidad,
    precio_unitario), `descuento`, `observacion`, y cliente si
    `cliente_id` está presente (si no, Consumidor Final).
- En `handleCreate`, tras éxito de `createVenta`:
  - Si vino de un presupuesto → `cambiarEstado(id, "convertido")`,
    `router.replace` limpiando el query, toast "Presupuesto N° X convertido a
    venta N° Y".
  - El warning de stock insuficiente ya existe en el carrito.

## 14. Verificación

```bash
cd src-tauri && cargo test --lib      # migración + filtros + transiciones + existentes
cd src-tauri && cargo clippy --lib --tests
pnpm build                            # vue-tsc + vite build
```

## 15. Documentación

`AGENTS.md`:

- Modelo `presupuestos`: agregar `'anulado'` al CHECK y nota de la migración.
- Tabla de comandos: agregar `cambiar_estado_presupuesto`.
- Agregar `PresupuestosPage` a la lista de páginas y aclarar que la conversión
  es vía carrito (marcar `convertido` tras registrar la venta).

## Notas / riesgos

- Si `cambiarEstado(convertido)` fallara tras una venta exitosa, el presupuesto
  quedaría `pendiente` con la venta ya creada (inconsistencia menor; se avisa
  con toast). El botón de conversión seguiría habilitado.
- La migración solo se ejecuta si el `CHECK` no contiene `anulado`; es segura de
  re-ejecutar y no toca datos.
