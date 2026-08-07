# Plan: Sistema de Auditoría de Acciones de Usuarios

## Objetivo

Registrar en base de datos cada acción de mutación (nuevo, modificar, eliminar) que
un usuario realiza en la aplicación, guardando: `user_id`, `username` (snapshot),
`screen` (pantalla), `action` (nuevo / modificar / consultar / eliminar), `detail`
(detalle opcional) y `created_at` (fecha y hora ISO 8601 UTC).

## Decisiones confirmadas

- **Backend centralizado**: cada comando Tauri registra la auditoría tras operar con
  éxito. El usuario se obtiene de `user_id` (ya recibido por todos los comandos) y la
  pantalla/acción se mapea por comando. Inmune a manipulación del frontend.
- **Solo CRUD**: se registran `nuevo` / `modificar` / `eliminar`. `consultar` queda
  disponible como tipo en el enum pero no se registra cada carga de pantalla.
- **Campos complementarios**: columna `detail` (entidad/id afectados) y snapshot de
  `username` para que el log siga legible si el usuario se elimina.
- **Nuevo permiso `ver_auditoria`** (sync en 3 lugares: Rust `PermissionCode`,
  TypeScript `PERMISSIONS` y lista seed de la DB) + nueva pantalla "Auditoría".

## Arquitectura

Sigue la Clean Architecture ya usada en el proyecto (domain / application /
infrastructure / api), con una entidad, un trait de repositorio, una implementación
SQLite, un servicio y comandos Tauri.

## Backend (Rust)

### 1. Esquema DB — `src-tauri/src/infrastructure/database/mod.rs`

```sql
CREATE TABLE IF NOT EXISTS audit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    username TEXT NOT NULL,          -- snapshot: legible aunque se borre el usuario
    screen TEXT NOT NULL,            -- Usuarios, Proveedores, Categorias, ...
    action TEXT NOT NULL,            -- nuevo / modificar / consultar / eliminar
    detail TEXT,
    created_at TEXT NOT NULL         -- ISO 8601 (UTC)
);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_screen_action ON audit_logs(screen, action);
```

- Sin FK a `users`: los logs se conservan aunque el usuario se elimine.
- Agregar `"ver_auditoria"` a la lista `PERMISSIONS` de seed (se inserta con
  `INSERT OR IGNORE`, no rompe DBs existentes).

### 2. Entidad — `src-tauri/src/domain/entities/audit_log.rs` (nuevo)

- `AuditAction` enum con `as_str()`:
  - `Create` → `"nuevo"`
  - `Update` → `"modificar"`
  - `Read` → `"consultar"`
  - `Delete` → `"eliminar"`
- `AuditScreen` enum con `as_str()`: `Usuarios`, `Proveedores`, `Categorias`,
  `SubCategorias`, `Articulos`, `Stock`, `Permisos`, `Auditoria`.
- `AuditLog` struct `#[derive(Serialize)]`: `id`, `user_id`, `username`, `screen`,
  `action`, `detail: Option<String>`, `created_at: DateTime<Utc>`.
- Registrar en `src-tauri/src/domain/entities/mod.rs`.

### 3. Repositorio

- Trait `AuditLogRepository` — `src-tauri/src/domain/repositories/audit_log_repository.rs`:
  - `create(&self, &AuditLog)`
  - `find_with_filters(...)` con filtros opcionales: `user_id`, `screen`, `action`,
    rango `from`/`to`, paginación `limit`/`offset`.
- Implementación `SqliteAuditLogRepository` —
  `src-tauri/src/infrastructure/repositories/SqliteAuditLogRepository.rs`
  (mismo patrón que `SqliteCategoriaRepository`, usa `DB` global).
- Registrar ambos en sus `mod.rs` (`domain/repositories/mod.rs` e
  `infrastructure/repositories/mod.rs`).

### 4. Servicio + helper central — `src-tauri/src/application/services/audit_log_service.rs`

```rust
pub fn log_audit(
    user_id: i64,
    screen: AuditScreen,
    action: AuditAction,
    detail: Option<&str>,
) -> Result<(), AppError>
```

- Función `pub` libre invocable desde cualquier comando sin estado.
- Resuelve el `username` (SELECT en `users`) para el snapshot, arma el `AuditLog` con
  `created_at = Utc::now()` y lo persiste.
- `AuditLogService` con `log(...)` y `get_logs(filters)`.
- Registrar en `src-tauri/src/application/services/mod.rs`.

### 5. Commands — `src-tauri/src/api/commands/audit_log_commands.rs` (nuevo)

- `AuditLogAppState { audit_service: Mutex<AuditLogService> }`.
- `GetAuditLogsRequest { user_id?, screen?, action?, from?, to?, limit?, offset? }`.
- `get_audit_logs(user_id, request, state) -> Result<Vec<AuditLog>, AppError>` con
  `check_permission(user_id, PermissionCode::ViewAuditoria)` (mismo patrón duplicado
  de los demás commands de dominio).
- Registrar en `src-tauri/src/api/commands/mod.rs` y `src-tauri/src/lib.rs`
  (`.manage(AuditLogAppState::new())` + handler).

### 6. Permiso — `src-tauri/src/domain/entities/permission_code.rs`

- Agregar `ViewAuditoria` → `"ver_auditoria"` en `as_str()` y en `all()`.

### 7. Instrumentar los comandos CRUD existentes (1 línea c/u tras el éxito)

```
create/update/delete_user              → screen Usuarios
add/remove_permission_to_user          → screen Permisos, action modificar
create_permission                      → screen Permisos, action nuevo
create/update/delete_proveedor         → screen Proveedores
create/update/delete_categoria         → screen Categorias
create/update/delete_sub_categoria     → screen SubCategorias
create/update/delete_articulo          → screen Articulos
create/update/delete_stock             → screen Stock
```

- `detail` = entidad afectada, ej: `Some("Categoría: Almacén (id 3)")`.
- Solo se registra si la operación fue exitosa (no en `PermissionDenied` ni fallos).
- Sin riesgo de deadlock: `log_audit` se invoca tras finalizar la operación, cuando el
  `Mutex<Connection>` global ya está liberado.

## Frontend (Vue)

- `src/domain/entities/permissions.ts`: agregar `VIEW_AUDITORIA: "ver_auditoria"`.
- `src/domain/entities/types.ts`: interfaces `AuditLog` y `AuditLogFilters`.
- `src/infrastructure/api/auditRepository.ts` (nuevo): `AuditApiRepository`
  con `getAuditLogs(filters)` → `invoke("get_audit_logs", { userId, request })`
  (mismo patrón que `userRepository`).
- `src/presentation/stores/index.ts`: store `useAuditStore` con `logs`, `loading`,
  `error` y `fetchLogs(filters)`.
- `src/presentation/composables/usePermissions.ts`: `canViewAuditoria()`.
- `src/presentation/pages/AuditoriaPage.vue` (nuevo): tabla con usuario, pantalla,
  acción, detalle y fecha/hora; filtros (usuario, pantalla, acción, rango de fechas)
  y paginación.
- `src/presentation/router/index.ts`: ruta `auditoria` con
  `meta: { permission: PERMISSIONS.VIEW_AUDITORIA }`.
- `src/presentation/layouts/MainLayout.vue`: item de menú "Auditoría" con
  `permission: "ver_auditoria"`.

## Verificación

- `cd src-tauri && cargo check`
- `pnpm build` (vue-tsc + vite build)

## Notas

- Archivos Rust nuevos respetando el naming del repo (contenido snake_case, módulos
  declarados en `mod.rs`); no se toca `src-tauri/gen/**`.
- Fecha/hora en ISO 8601 UTC, consistente con `users.created_at`.
