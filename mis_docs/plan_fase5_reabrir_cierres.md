# Plan Fase 5 — Bugs de cierre de venta

## Objetivo

Resolver los 5 bugs documentados en `mis_docs/bugs_cierre_venta.md`:

1. Los días que no tienen ventas no se tienen que cerrar.
2. Se tiene que poder reabrir una fecha, con el permiso necesario.
3. Si el día se reabre se tiene que eliminar de la base de datos.
4. Si el día está cerrado no se tienen que poder registrar nuevas ventas.
5. Mostrar cartel en el módulo de ventas con el mensaje "día cerrado no se
   pueden ingresar más ventas".

**Dependencia**: Fase 4 (módulo de cierres del día ya implementado).

## Decisiones de diseño (confirmadas)

- **Permiso nuevo**: `reabrir_cierre` (granular e independiente de
  `crear_cierre`). Sync en 3 lugares + `usePermissions`.
- **Fechas**: se puede cerrar hoy y días pasados; **se rechaza cerrar fechas
  futuras**.
- **Anulación en día cerrado**: se **bloquea**. Si el día está cerrado no se
  pueden anular ventas de ese día (el snapshot del cierre queda consistente).
  Reabrir es el único camino para modificar.
- **Atomicidad**: `crear_cierre` corre duplicado + snapshot + INSERT en una
  sola transacción bajo un único `DB.lock()`. El bloqueo de ventas/anulación en
  día cerrado se valida dentro de la transacción de `venta_repository`.
- **Zona horaria**: el día es local. `ventas.fecha` está en UTC RFC3339; el
  check de día cerrado calcula la fecha local del momento de la venta y de la
  venta a anular con el mismo criterio que el cierre.
- **Cascade**: `cierre_tipos.id_cierre` ya tiene `ON DELETE CASCADE`; al
  reabrir (eliminar) el cierre, los tipos se limpian solos.
- **Cartel**: un vendedor no tiene `ver_cierres`. Se agrega un comando
  dedicado `is_dia_cerrado` (gateado en `crear_venta`) que devuelve si hoy
  (local, calculado en backend) está cerrado. El check autoritativo de
  `create_venta` igual protege el backend.

## Problemáticas detectadas y cómo se resuelven

| Problemática | Solución |
|---|---|
| `crear_cierre` libera el lock entre snapshot e INSERT (`drop(conn)`) → cierre desactualizado | Todo en una transacción + un solo lock |
| TOCTOU: venta insertada mientras se cierra el día | Check "día cerrado" dentro de la transacción de `create`/`anular` |
| DST: `Local.from_local_datetime(...).single().unwrap()` puede panic | `earliest()`/`latest()` con fallback |
| Anular venta de día cerrado desincroniza el snapshot | Bloquear anulación (check por la fecha local de la venta) |
| Fechas futuras | Rechazo con `CierreFechaFutura` |
| Día sin ventas | Rechazo con `CierreSinVentas` |
| Cartel para usuarios sin `ver_cierres` | Comando `is_dia_cerrado` gateado en `crear_venta` |
| Cierres viejos con ceros ya creados | No se migran (fuera de alcance) |

---

## Backend (Rust)

### 1. Permiso — `src-tauri/src/domain/entities/permission_code.rs`

- Variante `ReopenCierre` en la enum (después de `CreateCierre`).
- `as_str()` → `"reabrir_cierre"`.
- Agregar a `all()`.

### 2. Seed — `src-tauri/src/infrastructure/database/mod.rs`

```rust
// Cierres del día
"ver_cierres",
"crear_cierre",
"reabrir_cierre",
```

### 3. Errores — `src-tauri/src/infrastructure/error.rs`

| Variante | `code()` | `user_message()` |
|---|---|---|
| `CierreSinVentas` | `cierre_sin_ventas` | "El día seleccionado no tiene ventas para cerrar." |
| `CierreFechaFutura` | `cierre_fecha_futura` | "No se puede cerrar una fecha futura." |
| `DiaCerrado` | `dia_cerrado` | "Día cerrado, no se pueden ingresar más ventas." |

Agregar a las tres secciones (enum, `code()`, `user_message()`).

### 4. Trait — `src-tauri/src/domain/repositories/cierre_repository.rs`

```rust
pub trait CierreRepository: Send + Sync {
    fn find_by_fecha(&self, fecha: &str) -> Result<Option<CierreWithTipos>, AppError>;
    fn find_all(&self) -> Result<Vec<CierreWithTipos>, AppError>;
    fn delete_by_fecha(&self, fecha: &str) -> Result<(), AppError>;
}
```

Se quita `create` (la creación pasa a orquestarse en el servicio sobre una
transacción única; el INSERT se hace vía helper concreto del repo).

### 5. Implementación — `src-tauri/src/infrastructure/repositories/cierre_repository.rs`

- `delete_by_fecha`: transacción, `DELETE FROM cierres WHERE fecha = ?1`; si 0
  filas → `CierreNotFound`; commit (cascade limpia `cierre_tipos`).
- Helper concreto `insert(&Transaction, cierre, tipos) -> Result<i64, AppError>`:
  INSERT cabecera + tipos, devuelve `last_insert_rowid()`.
- Helper concreto `load_by_id(&Connection, id) -> Result<Option<CierreWithTipos>, AppError>`:
  combina `load_cierre` + `load_tipos`.

### 6. Servicio — `src-tauri/src/application/services/cierre_service.rs`

`crear_cierre(fecha)` (refactor atómico):

1. Validar formato `%Y-%m-%d`.
2. Rechazar fecha futura (`day > Local::now().date_naive()` → `CierreFechaFutura`).
3. Un único `DB.lock()` + `conn.transaction()`:
   - Check duplicado → `CierreYaExiste`.
   - Snapshot (consultas actuales de ventas y costos).
   - Si `ventas_by_tipo` está vacío → `CierreSinVentas`.
   - INSERT cabecera + tipos vía `repository.insert(&tx, ...)`.
   - `tx.commit()`.
   - Recargar con `load_by_id(&conn, id)`.
4. `local_to_utc` con `earliest()`/`latest()` (fallback DST).

Nuevos métodos:

- `reabrir_cierre(fecha)` → `repository.delete_by_fecha(fecha)`.
- `is_dia_cerrado()` → `find_by_fecha(Local::now() fecha local)`.

### 7. Ventas — `src-tauri/src/infrastructure/repositories/venta_repository.rs`

- `create`: dentro de la transacción, calcular fecha local de hoy
  (`Local::now()`) y `SELECT COUNT(*) FROM cierres WHERE fecha = ?1`; si > 0 →
  `Err(AppError::DiaCerrado)` antes de insertar.
- `anular`: leer `(anulada, fecha)` de la venta; convertir la `fecha` UTC a
  fecha local (`parse_from_rfc3339` + `with_timezone(&Local)`); mismo check →
  `Err(AppError::DiaCerrado)` antes de restaurar stock.
- Helper `utc_to_local_date`.

### 8. Commands — `src-tauri/src/api/commands/cierre_commands.rs`

- `reabrir_cierre(user_id, request, state)` → `check_permission(ReopenCierre)`
  → `service.reabrir_cierre` → `log_audit(Cierres, Delete, "Cierre del día
  {fecha} reabierto")`.
- `is_dia_cerrado(user_id, state) -> Result<bool, AppError>` →
  `check_permission(CreateVenta)` → `service.is_dia_cerrado()`.

### 9. Registro — `src-tauri/src/lib.rs`

Importar y registrar `reabrir_cierre` e `is_dia_cerrado` en
`tauri::generate_handler!`.

---

## Frontend (Vue)

### 1. Permisos — `src/domain/entities/permissions.ts`

```ts
REABRIR_CIERRE: "reabrir_cierre",
```

### 2. Composable — `src/presentation/composables/usePermissions.ts`

`canReabrirCierre()` + devolverla.

### 3. Repositorios

- `src/infrastructure/api/cierreRepository.ts`: `reabrirCierre(fecha)`.
- `src/infrastructure/api/ventaRepository.ts`: `isDiaCerrado()` (invoke
  `is_dia_cerrado`).

### 4. Stores — `src/presentation/stores/index.ts`

- `useCierresStore.reabrirCierre(fecha)`: llama al repo, quita el cierre del
  listado local.
- `useVentasStore`: `diaCerrado` (ref) + `checkDiaCerrado()` para el cartel.

### 5. Página Cierres — `src/presentation/pages/CierresPage.vue`

- Columna "Acciones" con botón "Reabrir" (gated `canReabrirCierre()`),
  confirmación y toast.

### 6. Módulo de ventas — cartel

- `src/presentation/pages/NuevaVentaPage.vue`: `checkDiaCerrado()` en
  `onMounted`; banner "Día cerrado, no se pueden ingresar más ventas." y
  deshabilitar el botón de confirmar.
- `src/presentation/pages/VentasPage.vue`: `checkDiaCerrado()` en `onMounted` y
  banner en el encabezado.

---

## Verificación

- `cd src-tauri && cargo check && cargo clippy`.
- `pnpm build`.
- Prueba manual:
  - Cerrar un día con ventas → OK con totales.
  - Cerrar un día sin ventas → "El día seleccionado no tiene ventas para
    cerrar."
  - Cerrar una fecha futura → "No se puede cerrar una fecha futura."
  - Cerrar hoy y luego intentar vender → bloqueado + cartel en el módulo de
    ventas.
  - Anular una venta de un día cerrado → bloqueado.
  - Reabrir (permiso `reabrir_cierre`) → el cierre se elimina de la BD y se
    puede vender/anular de nuevo; al volver a cerrar se recalcula todo.
  - Un usuario sin `reabrir_cierre` no ve el botón y el comando devuelve
    `PermissionDenied`.
