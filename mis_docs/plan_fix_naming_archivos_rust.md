# Plan: Renombrar archivos Rust a snake_case (fix de rust-analyzer en VS Code)

## Objetivo

Eliminar el error de VS Code (rust-analyzer) `unresolved module: can't find module file`
en `src-tauri/src/infrastructure/repositories/mod.rs` (y módulos hermanos), renombrando
los archivos que usan `PascalCase` en su nombre a `snake_case` para que coincidan con las
declaraciones de módulo de cada `mod.rs`.

## Causa raíz

Los `mod.rs` declaran módulos en minúscula (`pub mod categoria_repository;`) pero los
archivos físicos se llaman `Categoria_repository.rs` (con `C` mayúscula). En Windows el
filesystem es case-insensitive, por lo que `cargo check` resuelve el módulo igualmente.
rust-analyzer, en cambio, resuelve los archivos de módulo con sensibilidad a mayúsculas
y no encuentra `categoria_repository.rs`, marcando el error. Además, esto rompe la
compilación en filesystems case-sensitive (Linux/macOS). Es el punto 12 de AGENTS.md.

## Archivos afectados

| Archivo actual | Destino | Módulo declarado |
|---|---|---|
| `src-tauri/src/api/commands/Categoria_commands.rs` | `categoria_commands.rs` | `categoria_commands` |
| `src-tauri/src/application/services/Categoria_service.rs` | `categoria_service.rs` | `categoria_service` |
| `src-tauri/src/domain/repositories/Categoria_repository.rs` | `categoria_repository.rs` | `categoria_repository` |
| `src-tauri/src/infrastructure/repositories/Categoria_repository.rs` | `categoria_repository.rs` | `categoria_repository` |

## Pasos

1. **Renombrar con git en dos pasos** (solo cambio de mayúsculas y `core.ignorecase=true`
   en git: un rename directo no quedaría registrado). Para cada archivo:
   - `git mv <origen> <nombre_temporal>` (agrega un sufijo `_tmp.rs`)
   - `git mv <nombre_temporal> <destino_snake_case>`

2. **Sin cambios de código**: los `use` y rutas de módulo ya referencian los nombres en
   minúscula (verificado con grep; no hay referencias a `Categoria_*` como módulo).

3. **Verificar**:
   - `cd src-tauri && cargo check`
   - `cd src-tauri && cargo clippy`

4. **Actualizar AGENTS.md**: reemplazar el punto 12 (los archivos quedan en snake_case y
   ya compilan en filesystems case-sensitive).

## Verificación

- `cargo check` y `cargo clippy` sin errores.
- El error `unresolved module` de VS Code desaparece (se confirma al recargar
  rust-analyzer).
- `git status` muestra los 4 renames registrados.

## Notas

- No se toca `src-tauri/gen/**` ni ningún contenido de los archivos Rust.
- Los structs/`use` en PascalCase (`SqliteCategoriaRepository`, `CategoriaService`, etc.)
  son nombres de items y no se modifican.
