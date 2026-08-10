# Plan: Rendimiento del login e inicialización de la base de datos

## Objetivo

Eliminar los congelamientos de la interfaz (hilo principal colgado) que se producen
al abrir la aplicación y al iniciar sesión, y separar de forma clara las tareas de
cada momento (apertura de la app, login y carga posterior al login).

## Diagnóstico

| Problema | Causa raíz | Impacto |
|---|---|---|
| Ventana congelada al abrir (1.ª vez) | `src-tauri/src/lib.rs:24` fuerza el `Lazy<Mutex<Connection>>` de la DB de forma síncrona en el hilo principal antes de crear la ventana. El seed calcula **7 hashes bcrypt a cost 12** (admin + 6 usuarios demo) ≈ 1,5–2 s. | Ventana en blanco al arrancar |
| UI se congela al hacer login | `login` es un comando síncrono → Tauri v2 lo ejecuta en el **main thread**. `bcrypt::verify` a cost 12 ≈ 250 ms por intento. | Freeze al clickear "Entrar" |
| UI se congela tras el login | `HomePage.vue:36` dispara 7 `fetch*` en `Promise.all`; cada uno es un comando síncrono en el main thread y todos serializan sobre el `Mutex<Connection>` global. | Freeze hasta terminar los 7 |
| Percepción "se crea la DB al loguear" | La creación ocurre en el arranque pero antes de mostrar la ventana; el usuario la asocia al primer uso. | — |

## Decisiones confirmadas

- **`#[tauri::command(async)]` en todos los comandos**: cambio mecánico que ejecuta el
  cuerpo síncrono en el runtime async de Tauri, fuera del hilo principal. Los cuerpos
  no tienen `.await` internos, por lo que los `State<AppState>` y `Mutex` actuales
  siguen compilando (el guard no cruza await points).
- **Nuevo comando `ensure_db_ready`**: inicializa la DB en background
  (`spawn_blocking`) y resuelve cuando está lista. El frontend lo espera en la pantalla
  de login.
- **Quitar el init forzado del hilo principal**: eliminar `let _ = &DB;` de `lib.rs`.
- **Bajar bcrypt a cost 10**: los hashes **nuevos** (seed, alta de usuario, cambio de
  contraseña) usan cost 10 (~20–30 ms). Los hashes existentes conservan su costo
  embebido en `verify` (no se requiere migración).
- Se mantiene el `Mutex<Connection>` global de SQLite (fuera de alcance de este plan);
  el cambio async ya elimina el freeze.

## Cambios backend (Rust)

### 1. Comandos async — `src-tauri/src/api/commands/*.rs`

Cambiar `#[tauri::command]` → `#[tauri::command(async)]` en:

- `mod.rs`: `login`, `create_user`, `get_all_users`, `update_user`, `change_password`,
  `delete_user`, `add_permission_to_user`, `remove_permission_from_user`,
  `get_user_permissions`, `get_all_permissions`, `create_permission`.
- `articulo_commands.rs`, `categoria_commands.rs`, `proveedor_commands.rs`,
  `stock_commands.rs`, `sub_categoria_commands.rs`, `tipo_venta_commands.rs`,
  `venta_commands.rs`, `cierre_commands.rs`, `audit_log_commands.rs`.

### 2. Comando `ensure_db_ready` — `src-tauri/src/api/commands/mod.rs`

```rust
#[tauri::command]
async fn ensure_db_ready() -> Result<(), AppError> {
    tauri::async_runtime::spawn_blocking(|| {
        let _ = &*crate::infrastructure::database::DB;
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))
}
```

- Fuerza la inicialización del `Lazy` en el blocking pool (tablas + seeds) y resuelve
  cuando la DB está lista.
- Registrarlo en `generate_handler!` en `lib.rs`.

### 3. Quitar init forzado — `src-tauri/src/lib.rs`

Eliminar `let _ = &infrastructure::database::DB;` (línea 24). La DB se inicializa en
background vía `ensure_db_ready` o perezosamente en el primer comando (ya async).

### 4. bcrypt cost 10

Definir `const BCRYPT_COST: u32 = 10;` y reemplazar `bcrypt::DEFAULT_COST`:

- `src-tauri/src/application/services/user_service.rs` → `create_user` y
  `change_password`.
- `src-tauri/src/infrastructure/database/mod.rs` → `seed_admin_user` y
  `seed_demo_data`.

## Cambios frontend (división de tareas)

### 5. Repositorio — `src/infrastructure/api/userRepository.ts`

Agregar:

```ts
async ensureDbReady(): Promise<void> {
  return await invoke<void>("ensure_db_ready");
}
```

### 6. Login — `src/presentation/pages/LoginPage.vue`

En `onMounted` llamar `ensureDbReady()`; estado `dbReady` que deshabilita el botón y
muestra "Inicializando base de datos..." hasta que resuelva.

## Nueva división de tareas

- **Abrir app**: ventana inmediata + render de login (o sesión restaurada) al
  instante; `ensure_db_ready` inicializa la DB en background. Sin bloqueos.
- **Iniciar sesión**: solo autenticación (verify + permisos), async, sin congelar.
- **Después del login**: navegar a Home al instante; sus fetches corren async (sin
  freeze). Cada página mantiene su fetch propio en `onMounted`.

## Verificación

1. `cd src-tauri && cargo check`
2. `cd src-tauri && cargo clippy`
3. `pnpm build` (typecheck `vue-tsc --noEmit` + build)
4. Prueba manual:
   - Borrar `app.db` → la app abre al instante, login muestra "Inicializando...",
     luego login fluido.
   - Con DB existente → login inmediato sin congelamiento.
