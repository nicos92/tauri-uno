# AGENTS.md — Guía para Agentes de Código

## Stack y setup
- Tauri 2 + Vue 3 + TypeScript strict + Vite + Pinia + Vue Router. Gestor: `pnpm`.
- **No hay ESLint/Prettier ni tests** (ni Rust ni TS). La verificación real es:
  - `pnpm build` (corre `vue-tsc --noEmit && vite build` — es el typecheck)
  - `cd src-tauri && cargo check` (y `cargo clippy`)
- Comandos: `pnpm dev` (Vite, puerto fijo 1420, strictPort), `pnpm tauri dev` / `pnpm tauri build`.

## Arquitectura (clean architecture espejada)
- Backend `src-tauri/src/`: `domain` (entidades + traits de repositorio), `application/services`, `infrastructure/repositories` (impls `Sqlite*`), `infrastructure/database`, `api/commands` (comandos `#[tauri::command]`).
- Frontend `src/`: `domain` (types + `PERMISSIONS`), `application/usecases`, `infrastructure/api`, `presentation` (layouts, pages, router, stores, composables).
- Registrar comandos y estados en `src-tauri/src/lib.rs` (`.manage()` + `tauri::generate_handler!`).

## Gotchas críticos
1. **DB global**: `infrastructure::database::DB` es un `static Lazy<Mutex<Connection>>` (rusqlite no es `Sync`); todos los repos lo bloquean. El esquema se crea en el primer arranque en el directorio de datos de `ProjectDirs` (`app.db`), sin migraciones. Se siembran 27 permisos y el usuario `admin` / `admin123` con todos los permisos.
2. **Sincronizar permisos en 3 lugares** (strings en español snake_case, ej. `ver_usuarios`): Rust `PermissionCode::as_str()` (`domain/entities/permission_code.rs`), TS `PERMISSIONS` (`src/domain/entities/permissions.ts`), y la lista seed (`infrastructure/database/mod.rs`).
3. **Chequeo de permisos backend**: todo comando recibe `user_id: i64` como primer argumento. Los comandos de usuarios/permisos (`api/commands/mod.rs`) usan `AppState` + `UserService::has_permission`; los de dominio (articulo/categoria/...) duplican un `check_permission` propio que consulta la DB directo. Respetar el patrón al agregar comandos.
4. **Auth en frontend**: usuario y permisos se persisten en `localStorage` (`currentUser`, `userPermissions`). Los repos leen `getCurrentUserId()` de localStorage y lo pasan como `userId` a cada `invoke`. El guard del router llama `authStore.loadFromStorage()`.
5. **Repos no uniformes**: solo `UserApiRepository` implementa `IUserRepository` y pasa por usecases; Articulo/Categoria/Proveedor/Stock/SubCategoria son clases directas invocadas desde los stores. `infrastructure/api/index.ts` solo re-exporta `userRepository` (los demás se importan por ruta completa).
6. **Nombres de archivo Rust PascalCase** (`Categoria_commands.rs`, `Categoria_service.rs`, `Categoria_repository.rs`) mientras `mod.rs` declara módulos en minúscula — solo compila en filesystems case-insensitive (Windows/macOS); en Linux falla. No renombrar sin avisar.
7. **Código generado**: `src-tauri/gen/**` (proyecto Android) no se edita a mano.

## Convenciones
- Texto de UI y commits en español; identificadores de código en inglés.
- TS: comillas dobles, strict mode (`noUnusedLocals`/`noUnusedParameters`), sin `any`, recordar `.value` en refs.
- Rust: indentación 4 espacios, snake_case, manejar `Result` con `?`; errores vía `AppError` (se serializa a string en las respuestas de invoke).
