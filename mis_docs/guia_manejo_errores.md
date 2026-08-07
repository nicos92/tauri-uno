# Guía: Implementación de Manejo de Errores

Plan para implementar el manejo de errores en el proyecto, con foco en el caso de
eliminación de filas con referencias a otras tablas (error de FOREIGN KEY).

## Diagnóstico

El error clásico de FOREIGN KEY nunca ocurre hoy porque `PRAGMA foreign_keys`
no está habilitado en `init_database()` (`src-tauri/src/infrastructure/database/mod.rs`).
Las FKs están declaradas en el esquema pero SQLite no las aplica: al eliminar se
dejan filas huérfanas en silencio.

Además:

- **Backend**: todo error SQLite cae en `AppError::Database("Database error: ...")`
  genérico; `AppError` se serializa como string plano (`error.rs`), sin código, por
  lo que el frontend no puede distinguir un error FK de otro.
- **Cobertura incompleta**: `CategoriaService` y `SubCategoriaService` tienen
  pre-checks (`has_sub_categorias`, `has_articulos`), pero `ProveedorService::delete`
  no verifica si hay artículos referenciándolo. No hay guard para eliminar a sí
  mismo ni al usuario `admin`.
- **Frontend**: sin sistema de toasts; un error de borrado reemplaza la tabla
  completa con un div rojo, y las páginas ignoran el booleano que devuelve el store.

## Decisiones tomadas

- Contrato de errores: **estructurado `{ code, message }`**.
- Estrategia FK: **habilitar `PRAGMA foreign_keys = ON` + pre-checks**.
- UX de errores: **toast ligero propio** (sin librería).
- Brechas adicionales: **incluir** guards de usuario y permisos.

---

## Fase 1 — Backend: errores estructurados y enforcement de FKs

### 1.1 Habilitar FKs en SQLite

Archivo: `src-tauri/src/infrastructure/database/mod.rs`

Agregar `PRAGMA foreign_keys = ON;` al inicio del `execute_batch` de
`init_database()`. Es un solo `Connection` global, así que la pragma queda
activa para toda la sesión.

Nota: las tablas existentes no se migran; los huérfanos creados antes no se
validan retroactivamente, pero los deletes con pre-check y el mapping de
constraints lo cubren.

### 1.2 Error mapping por código SQLite

Archivo: `src-tauri/src/infrastructure/error.rs`

Reemplazar el `#[from] rusqlite::Error` por un `impl From<rusqlite::Error> for AppError`
manual que matchee `Error::SqliteFailure(ffi::Error { extended_code, .. }, msg)`:

| Código extendido | Significado | Variante AppError |
|---|---|---|
| `787` | `SQLITE_CONSTRAINT_FOREIGNKEY` | `AppError::ForeignKeyConstraint` |
| `2067` | `SQLITE_CONSTRAINT_UNIQUE` | `AppError::DuplicateValue` |
| resto | cualquier otro error SQLite | `AppError::Database(...)` (sin perder `msg`) |

Variantes nuevas de `AppError`: `ForeignKeyConstraint`, `ProveedorHasArticulos`,
`CannotDeleteSelf`, `CannotDeleteAdmin`.

### 1.3 Pre-check proveedor → artículos

Espejo del patrón existente en categoría/subcategoría.

- `src-tauri/src/domain/repositories/Proveedor_repository.rs`:
  agregar `fn has_articulos(&self, id: i64) -> Result<bool, AppError>` al trait.
- `src-tauri/src/infrastructure/repositories/proveedor_repository.rs`:
  implementarlo con `SELECT EXISTS(...)`.
- `src-tauri/src/application/services/proveedor_service.rs`:
  en `delete`, tras `ProveedorNotFound`, agregar el check `has_articulos` →
  `ProveedorHasArticulos` (mismo patrón que `categoria_service.rs` y
  `sub_categoria_service.rs`).

### 1.4 Serialización estructurada

Archivo: `src-tauri/src/infrastructure/error.rs`

- Mantener `Display` (técnico, para logs) y agregar:
  - `code(&self) -> &'static str` (snake_case, ej. `foreign_key_constraint`,
    `categoria_has_sub_categorias`, `permission_denied`, `database_error`,
    `internal_error`).
  - `user_message(&self) -> String` (español amigable, ej. "No se puede eliminar
    porque otros registros hacen referencia a este elemento").
- Cambiar el `serde::Serialize` a struct `{ "code": String, "message": String }`.
  Los comandos Rust no cambian: `Err(AppError)` propaga igual.
- Catálogo de mensajes amigables en el mismo archivo para todas las variantes.

### 1.5 Guards de usuario

Archivos: `src-tauri/src/application/services/user_service.rs` y
`src-tauri/src/api/commands/mod.rs`

- `delete_user` recibe el actor: `service.delete_user(user_id, id)`.
- En el servicio:
  - `find_by_id(id)` → si no existe, `UserNotFound`.
  - si `user_id == id` → `CannotDeleteSelf`.
  - si el target es `admin` → `CannotDeleteAdmin`.
  - luego `repo.delete(id)`.

---

## Fase 2 — Frontend: contrato de errores y toasts

### 2.1 Tipo compartido

Archivo nuevo: `src/domain/entities/appError.ts`

```typescript
export interface ApiError {
  code: string;
  message: string;
}
```

### 2.2 Helper de normalización

Archivo nuevo: `src/infrastructure/api/errorHandler.ts`

`toErrorMessage(e: unknown): string`: soporta objeto `ApiError` (`e.message`),
string legacy, `Error`, y fallback genérico.

### 2.3 Stores

Archivo: `src/presentation/stores/index.ts`

Reemplazar los ~32 `error.value = e as string` por `error.value = toErrorMessage(e)`.

### 2.4 Sistema de toasts propio (sin librería)

- `src/presentation/composables/useToasts.ts` (nuevo): estado reactivo a nivel
  módulo, expone `error(msg)` / `success(msg)`, auto-dismiss ~4s.
- `src/presentation/components/Toasts.vue` (nuevo): lista de toasts con CSS puro
  usando las vars de tema. (El directorio `components` no existe, se crea.)
- `src/App.vue`: montar `<Toasts />`.

### 2.5 Páginas — fix del delete y de la tabla oculta

Archivos: 6 páginas (Categorias, SubCategorias, Articulos, Proveedores, Stock, Users)

- En `handleDelete`: capturar el booleano; si falla →
  `useToasts().error(store.error)`.
- En el template: cambiar la cadena `v-else-if="store.error"` (que oculta la
  tabla, ej. `CategoriasPage.vue:81-83`) por un banner
  `<div v-if="store.error" class="error-banner">` **arriba** de la tabla; la tabla
  se muestra siempre que no haya loading.
- Modales de crear/editar: sin cambios estructurales (ya muestran error inline),
  solo se benefician de la normalización del mensaje.

### 2.6 Permisos

Archivo: `src/presentation/pages/UsersPage.vue`

`addPermission`/`removePermission` ya retornan `boolean` en el store; aplicar
toast al resultado. (No requiere cambio de store.)

---

## Fase 3 — Verificación

- `cd src-tauri && cargo check` (+ `cargo test` si aplica).
- `pnpm build` (corre `vue-tsc --noEmit` + `vite build`).
- Prueba manual:
  - Categoría con subcategorías → delete → toast "No se puede eliminar...".
  - Proveedor con artículos → idem.
  - Borrar `admin` → bloqueado.
  - Error de fetch → banner sin ocultar la tabla.

---

## Impacto de archivos

| Archivo | Cambio |
|---|---|
| `src-tauri/src/infrastructure/database/mod.rs` | PRAGMA foreign_keys |
| `src-tauri/src/infrastructure/error.rs` | Variantes, mapping From, code/message, Serialize estructurado |
| `src-tauri/src/domain/repositories/Proveedor_repository.rs` | trait `has_articulos` |
| `src-tauri/src/infrastructure/repositories/proveedor_repository.rs` | impl `has_articulos` |
| `src-tauri/src/application/services/proveedor_service.rs` | pre-check delete |
| `src-tauri/src/application/services/user_service.rs` | guards delete |
| `src-tauri/src/api/commands/mod.rs` | pasar actor id a `delete_user` |
| `src/domain/entities/appError.ts` | tipo nuevo |
| `src/infrastructure/api/errorHandler.ts` | helper nuevo |
| `src/presentation/composables/useToasts.ts` | nuevo |
| `src/presentation/components/Toasts.vue` | nuevo |
| `src/App.vue` | montar `<Toasts />` |
| `src/presentation/stores/index.ts` | normalizar mensajes |
| `src/presentation/pages/*` (6) | delete con toast + banner de error |

## Riesgos y consideraciones

- `DeleteStock` no tiene pre-check y su FK a `articulos` no tiene
  `ON DELETE CASCADE`; con FK activado y el cascade manual de
  `articulo_repository.rs` (borra stock primero) sigue correcto, pero el mapping
  genérico `ForeignKeyConstraint` cubre cualquier path no previsto.
- El contrato cambia de string a objeto: verificar que todos los consumidores de
  `store.error` siguen mostrando texto (se resuelve con 2.2 y 2.3).
