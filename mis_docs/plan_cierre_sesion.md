# Plan: Cerrar sesión al cerrar la aplicación

## Problema

Al cerrar la app con la cruz de la ventana o `Alt + F4` con una sesión iniciada, al
reabrir la app la sesión sigue activa. Por seguridad debe pedir login nuevamente.

## Causa raíz

La sesión se persiste en `localStorage` (`currentUser` y `userPermissions`) y el guard
del router (`src/presentation/router/index.ts`) restaura el estado autenticado con
`authStore.loadFromStorage()`. `localStorage` sobrevive al cierre del webview, así que
la sesión persiste entre ejecuciones.

## Decisión confirmada

- **`sessionStorage` en vez de `localStorage`** para las claves de sesión.
- `sessionStorage` está atado a la vida del webview: al cerrar la app (X, `Alt+F4`,
  crash o cierre forzado) se destruye y al reabrir no hay sesión.
- Sin handlers de eventos frágiles (`beforeunload`, `ExitRequested`, etc.).
- **No aplica al tema**: `themeStore` sigue en `localStorage` (el tema sí debe persistir).

## Archivos a modificar

### 1. `src/presentation/stores/index.ts` (store de auth)

- `login()`: `localStorage.setItem("currentUser" / "userPermissions")` →
  `sessionStorage.setItem(...)`
- `logout()`: `localStorage.removeItem(...)` → `sessionStorage.removeItem(...)`
- `loadFromStorage()`: `localStorage.getItem(...)` → `sessionStorage.getItem(...)`

### 2. Repositorios — `getCurrentUserId()` lee `localStorage` →

`sessionStorage`

- `src/infrastructure/api/userRepository.ts:12`
- `src/infrastructure/api/articuloRepository.ts:6`
- `src/infrastructure/api/CategoriaRepository.ts:6`
- `src/infrastructure/api/subCategoriaRepository.ts:6`
- `src/infrastructure/api/proveedorRepository.ts:6`
- `src/infrastructure/api/stockRepository.ts:6`
- `src/infrastructure/api/auditRepository.ts:6`

### 3. Se deja intacto

- `src/presentation/stores/themeStore.ts` (persistencia de tema en `localStorage`).

## Comportamiento esperado

- Login guarda sesión en `sessionStorage`.
- Cerrar la app (X / `Alt+F4` / forzado) destruye el `sessionStorage` del webview.
- Al reabrir, `loadFromStorage()` no encuentra datos → guard redirige a `/login`.

## Verificación

- `pnpm build` (vue-tsc + vite build)
- Manual: login → cerrar app → reabrir → debe pedir login.
