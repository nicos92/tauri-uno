# Cambio de contraseña

## Tarea

1. Permitir que todos los usuarios cambien su **propia** contraseña.
2. Crear un permiso para poder cambiar la contraseña de **cualquier** usuario
   (lo puede hacer `admin` o quien tenga ese permiso).
3. El módulo de usuarios solo permite editar datos (nombre, activo), no la
   contraseña.

## Decisiones acordadas

- **Cambio propio**: sección **Seguridad** en `SettingsPage`, accesible para
  todos sin permiso extra.
- **Cambio a otro usuario**: botón candado en `UsersPage`, visible solo con el
  permiso `cambiar_contrasena_usuario`. Abre un modal con nueva contraseña
  (no pide la actual).
- **Sin política de complejidad**: consistente con `create_user`. Solo guard
  mínimo anti-vacío en el backend.
- **Post cambio propio**: `logout()` + redirect a `/login` con banner
  "Contraseña cambiada, vuelva a ingresar".

## Arquitectura

### Backend (Rust) — un solo comando con dos rutas

- Nuevo permiso `ChangeUserPassword` → `"cambiar_contrasena_usuario"`,
  sincronizado en 3 lugares (AGENTS.md #7):
  1. `src-tauri/src/domain/entities/permission_code.rs` (enum + `as_str()` + `all()`)
  2. `src-tauri/src/infrastructure/database/mod.rs` (seed `PERMISSIONS`)
  3. `src/domain/entities/permissions.ts` (`PERMISSIONS` TS)
- `UserService::change_password(actor_id, target_user_id, current_password: Option<String>, new_password)`:
  - `target == actor` → exige `current_password` válida (`bcrypt::verify`). Sin permiso.
  - `target != actor` → no usa `current_password`; la validación de permiso la hace el command layer.
  - Hashea la nueva contraseña, carga el usuario con `find_by_id` y persiste con `update()`
    (sin cambios en repositorios; `update()` ya refresca `modified_at`).
  - Guard: `new_password` vacía → `AppError::EmptyPassword`.
- Comando `change_password` en `api/commands/mod.rs`:
  - `ChangePasswordRequest { target_user_id, current_password: Option<String>, new_password }`.
  - `actor != target` → `check_permission(ChangeUserPassword)`.
  - `log_audit(Usuarios, Update, "Contraseña actualizada (usuario id N)")` — nunca la contraseña.
- Registrar `change_password` en `lib.rs` (`generate_handler`).
- `error.rs`: variante `EmptyPassword` + `code()` + `user_message()`.

### Frontend (Vue/TS)

- `src/domain/entities/permissions.ts`: `CHANGE_USER_PASSWORD`.
- `src/domain/entities/types.ts`: `ChangePasswordRequest`.
- `src/domain/interfaces/userRepository.ts` + `src/infrastructure/api/userRepository.ts`:
  `changePassword(request)` → `invoke("change_password", { userId, request })`.
- Nuevo `ChangePasswordUseCase` (export en `usecases/index.ts`).
- `src/presentation/stores/index.ts`:
  - `auth.changeOwnPassword(current, nueva)` → `target_user_id = user.id`, `current_password = current`.
  - `users.changePassword(userId, nueva)` → `current_password = null`.
- `src/presentation/composables/usePermissions.ts`: `canChangeUserPassword()`.
- `SettingsPage.vue`: modal (actual, nueva, repetir) con botones `:disabled` anti
  doble-clic; éxito → toast + `authStore.logout()` + `router.push({ name: "login", query: { passwordChanged: "1" } })`.
- `LoginPage.vue`: lee `route.query.passwordChanged` y muestra banner.
- `UsersPage.vue`: botón candado (`v-if="canChangeUserPassword()"`) + modal
  (nueva, repetir).

## Problemáticas y soluciones

1. **Permiso sincronizado en 4 puntos** (enum `as_str`/`all`, seed DB, TS
   `PERMISSIONS`, `usePermissions`): si falta uno, rompe. Checklist obligatoria;
   actualizar AGENTS.md #7 y la tabla de API.
2. **DB existente sin el permiso**: `seed_permissions` + `seed_admin_user`
   corren en cada arranque con `INSERT OR IGNORE`, y el loop que asigna todos
   los permisos a `admin` está fuera del `if !exists` (mod.rs:325-334). El
   permiso nuevo y su asignación a admin se crean solos en el próximo arranque.
3. **Escalada de privilegios**: el permiso se exige solo cuando `actor != target`;
   para `target == self` se exige la contraseña actual. El hash nunca se expone
   (`password` tiene `#[serde(skip_serializing)]`).
4. **Auditoría sin datos sensibles**: el detalle solo referencia `usuario (id N)`.
5. **`Option<String>` / `null` entre frontend y Rust**: el caso admin envía
   `current_password: null` → `None`. Campos anidados en snake_case (mismo
   patrón que `AddPermissionRequest`).
6. **Doble submit**: botones `:disabled` mientras `isLoading`.
7. **Confirmación de nueva contraseña**: validación de igualdad en cliente.
8. **Sesión tras cambio propio**: `logout()` limpia `sessionStorage`; mensaje por
   query param y banner en LoginPage.
9. **Admin no aparece en la lista** (`find_all` excluye `admin`): el admin cambia
   su propia clave en Configuración; un encargado con el permiso no puede
   resetear al admin desde la UI. Decisión aceptada y documentada.
10. **Repos no uniformes**: seguir el patrón de `UserApiRepository` + usecases +
    stores (no el de clases directas de dominio).

## Verificación

```bash
pnpm build                     # vue-tsc --noEmit (strict) + vite build
cd src-tauri && cargo check
cd src-tauri && cargo clippy
```
