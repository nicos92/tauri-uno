# Plan: Módulo Presupuesto

## Objetivo

Permitir que desde la misma pantalla donde se arma la venta, el usuario pueda
guardar el carrito actual como un presupuesto, reutilizando la lógica de
interfaz existente pero aislando la persistencia de datos. Guardar un
presupuesto **no** descuenta stock.

## Decisiones confirmadas

- **Alcance**: crear presupuesto desde la pantalla de Nueva Venta (botón
  "Guardar Presupuesto") + backend/estructura completa (tablas, entidades,
  repositorio, comandos de lectura) para uso futuro (listado / conversión a
  venta).
- **Día cerrado**: se permite guardar un presupuesto aunque el día esté cerrado
  (no toca stock ni ventas).
- **Fecha de vencimiento**: campo opcional en la UI (input `type="date"`); si
  está vacío se guarda `NULL`.
- **Estado inicial**: `pendiente`. Valores posibles: `pendiente`, `aprobado`,
  `vencido`, `convertido` (con `CHECK` en la tabla).
- **`cliente_id` opcional**: a diferencia de `ventas`, el presupuesto puede
  quedar sin cliente (`NULL`).
- **Sin decremento de stock**: el alta inserta encabezado + detalle en una
  transacción atómica pero no ejecuta `UPDATE stock`.
- **Permiso existente**: `generar_presupuesto` ya está sincronizado en los 3
  lugares (Rust `PermissionCode`, seed de DB, TS `PERMISSIONS`) y existe el
  helper `canGenerarPresupuesto()`. No se agregan permisos nuevos.

## 1. Base de datos

`src-tauri/src/infrastructure/database/mod.rs` — agregar al `apply_schema`
(clonando `ventas`/`venta_detalle`):

```sql
CREATE TABLE IF NOT EXISTS presupuestos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    fecha TEXT NOT NULL,
    total REAL NOT NULL,
    descuento REAL NOT NULL DEFAULT 0,
    estado TEXT NOT NULL DEFAULT 'pendiente'
        CHECK (estado IN ('pendiente','aprobado','vencido','convertido')),
    fecha_vencimiento TEXT,
    observacion TEXT,
    cliente_id INTEGER REFERENCES clientes(id),
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS detalle_presupuestos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    id_presupuesto INTEGER NOT NULL,
    id_articulo INTEGER NOT NULL,
    cantidad REAL NOT NULL,
    costo_unitario REAL NOT NULL,
    precio_unitario REAL NOT NULL,
    subtotal REAL NOT NULL,
    FOREIGN KEY (id_presupuesto) REFERENCES presupuestos(id) ON DELETE CASCADE,
    FOREIGN KEY (id_articulo) REFERENCES articulos(id)
);

CREATE INDEX IF NOT EXISTS idx_detalle_presupuestos_id_presupuesto ON detalle_presupuestos(id_presupuesto);
CREATE INDEX IF NOT EXISTS idx_presupuestos_estado ON presupuestos(estado);
```

Además, agregar a `reset_test_db()`:

```sql
DROP TABLE IF EXISTS detalle_presupuestos;
DROP TABLE IF EXISTS presupuestos;
```

(En ese orden, respetando las FK.)

## 2. Entidades Rust

Nuevo archivo `src-tauri/src/domain/entities/presupuesto.rs` (espejo de
`venta.rs`):

- `enum PresupuestoEstado { Pendiente, Aprobado, Vencido, Convertido }` con
  `as_str()` → `"pendiente"`, `"aprobado"`, `"vencido"`, `"convertido"`.
- `Presupuesto`:
  `id`, `user_id`, `fecha`, `total`, `descuento`, `estado`,
  `fecha_vencimiento: Option<String>`, `observacion: Option<String>`,
  `cliente_id: Option<i64>`, `created_at`.
- `PresupuestoDetalle`:
  `id`, `id_presupuesto`, `id_articulo`, `cantidad`, `costo_unitario`,
  `precio_unitario`, `subtotal`.
- `PresupuestoDetalleConArticulo`:
  `id`, `id_articulo`, `cod_articulo`, `articulo`, `cantidad`,
  `costo_unitario`, `precio_unitario`, `subtotal`.
- `PresupuestoWithDetalle`:
  `id`, `user_id`, `username`, `fecha`, `subtotal`, `descuento`, `total`,
  `estado`, `fecha_vencimiento`, `observacion`, `cliente_id`,
  `cliente_nombre`, `cliente_apellido`, `created_at`, `items`.
- Constructores `::new(...)` siguiendo `venta.rs`.

Registrar en `src-tauri/src/domain/entities/mod.rs` (`pub mod presupuesto;` +
re-exports).

## 3. Trait repositorio

Nuevo archivo `src-tauri/src/domain/repositories/presupuesto_repository.rs`:

```rust
#[cfg_attr(test, mockall::automock)]
pub trait PresupuestoRepository: Send + Sync {
    fn create(
        &self,
        presupuesto: &Presupuesto,
        detalles: &[PresupuestoDetalle],
    ) -> Result<PresupuestoWithDetalle, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<PresupuestoWithDetalle>, AppError>;
    fn find_page(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Page<PresupuestoWithDetalle>, AppError>;
}
```

Registrar en `src-tauri/src/domain/repositories/mod.rs`.

## 4. Servicio de aplicación

Nuevo archivo `src-tauri/src/application/services/presupuesto_service.rs`
(patrón `VentaService`):

- `PresupuestoService { repository: Arc<dyn PresupuestoRepository>, cliente_repository: Arc<dyn ClienteRepository> }`
  con `with_repositories(...)` para tests.
- `create(user_id, detalles, descuento, observacion, fecha_vencimiento, cliente_id)`:
  - Valida `descuento` en `0..=100` → `AppError::DescuentoInvalido`.
  - Si `cliente_id` es `Some(id)`, valida que exista → `AppError::ClienteNotFound`.
    Si es `None`, persiste `NULL`.
  - Construye `Presupuesto` con `estado: Pendiente` y
    `fecha: chrono::Utc::now().to_rfc3339()`.
- `get_by_id` / `get_page`.
- Tests unitarios con `mockall` (`MockPresupuestoRepository`,
  `MockClienteRepository`): descuento inválido, cliente inexistente, cliente
  opcional.

Registrar en `src-tauri/src/application/services/mod.rs`.

## 5. Repositorio SQLite

Nuevo archivo `src-tauri/src/infrastructure/repositories/presupuesto_repository.rs`
(patrón `SqliteVentaRepository`):

- `create`: `DB.lock()` + transacción. Por detalle lee `stock`
  (`costo`, `ganancia`), calcula `precio_unitario` (si el del carrito es > 0 lo
  usa, si no `costo * (1 + ganancia/100)`), `subtotal` y `total`. Inserta
  encabezado (estado `'pendiente'`, `fecha_vencimiento` nullable) y detalles.
  **NO ejecuta `UPDATE stock`**. Commit + carga completa.
- `find_by_id` / `find_page` con JOINs a `users` y `clientes`, cargando `items`
  en bulk (como ventas).
- Tests de integración con `TEST_LOCK` + `reset_test_db()`:
  - persiste encabezado y detalles;
  - **el stock NO se modifica** (cantidades iguales antes y después);
  - `cliente_id` NULL permitido;
  - con un `cierre` del día actual, `create` **sí** funciona (día cerrado no
    bloquea).

Registrar en `src-tauri/src/infrastructure/repositories/mod.rs`.

## 6. Auditoría

`src-tauri/src/domain/entities/audit_log.rs`: agregar variante `Presupuestos` a
`AuditScreen` → `as_str()` => `"Presupuestos"`.

## 7. Comandos Tauri

Nuevo archivo `src-tauri/src/api/commands/presupuesto_commands.rs`:

- `PresupuestoAppState { presupuesto_service: Mutex<PresupuestoService> }` +
  `Default` (patrón `VentaAppState`).
- `CreatePresupuestoDetalleRequest { id_articulo: i64, cantidad: f64, precio_unitario: Option<f64> }`.
- `CreatePresupuestoRequest { items: Vec<CreatePresupuestoDetalleRequest>, descuento: Option<f64>, observacion: Option<String>, fecha_vencimiento: Option<String>, cliente_id: Option<i64> }`.
- `crear_presupuesto(user_id, request, state)`:
  - `check_permission(user_id, PermissionCode::GenerarPresupuesto)?`
  - llama al servicio y registra auditoría:
    `log_audit(user_id, AuditScreen::Presupuestos, AuditAction::Create, ...)`.
- `get_presupuesto_by_id` y `get_all_presupuestos` (paginado, mismo patrón de
  `get_all_ventas`) para dejar la estructura lista para listado/conversión.

Registrar módulo y re-exports en `src-tauri/src/api/commands/mod.rs`.

## 8. Registro en `src-tauri/src/lib.rs`

- `.manage(PresupuestoAppState::new())`.
- Agregar `crear_presupuesto`, `get_presupuesto_by_id`, `get_all_presupuestos`
  a `tauri::generate_handler!`.

## 9. Frontend TypeScript

- `src/domain/entities/types.ts`: interfaces `Presupuesto`,
  `PresupuestoDetalleConArticulo`, `PresupuestoWithDetalle`,
  `CreatePresupuestoDetalleRequest`, `CreatePresupuestoRequest`.
- `src/infrastructure/api/presupuestoRepository.ts` (nuevo, estilo
  `ventaRepository.ts`): clase `PresupuestoApiRepository` con
  `crearPresupuesto(request)` (y `getPresupuestoById` opcional para futuro),
  leyendo `getCurrentUserId()` de `sessionStorage`.
- `src/presentation/stores/index.ts`: store `usePresupuestosStore` con
  `crearPresupuesto(request): Promise<PresupuestoWithDetalle | null>`
  (errores con `toErrorMessage`).

## 10. UI — `src/presentation/pages/NuevaVentaPage.vue`

- Nuevo `ref fechaVencimiento` (input `type="date"`, opcional) en la sección
  header.
- Botón **"Guardar Presupuesto"** junto a "Registrar Venta":
  - visible si `canGenerarPresupuesto()`;
  - habilitado si `cart.length > 0` y `descuentoValido` (sin bloquear por
    `diaCerrado`);
  - `handleGuardarPresupuesto()` mapea `cart` → `CreatePresupuestoRequest`
    (mismo mapeo que `handleCreate` + `fecha_vencimiento`), invoca
    `presupuestosStore.crearPresupuesto`, muestra toast de éxito con el N° y
    **mantiene el carrito** (el usuario puede seguir editando o registrar la
    venta). Limpia solo el campo vencimiento.

## 11. Verificación

- `cd src-tauri && cargo test` (unitarios + integración con DB en memoria).
- `cd src-tauri && cargo clippy --lib --tests`.
- `pnpm build` (corre `vue-tsc --noEmit` + `vite build`).

## 12. Documentación

Actualizar `AGENTS.md`:

- árbol de arquitectura (backend/frontend);
- modelo de datos (`presupuestos`, `detalle_presupuestos`);
- tabla de comandos (`crear_presupuesto`, `get_presupuesto_by_id`,
  `get_all_presupuestos`).

## Notas Rust (best practices)

- Propagar errores con `?` sobre `AppError`/`thiserror` (ya existente).
- Préstamos `&Presupuesto` / `&[PresupuestoDetalle]` en el trait, sin clones
  innecesarios.
- Nombres de tests descriptivos: `create_no_decrements_stock`,
  `create_allowed_when_day_closed`, `create_with_optional_cliente`.
