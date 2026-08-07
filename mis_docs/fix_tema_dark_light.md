# Fix: Aplicación del tema oscuro/claro al recargar (F5)

Fecha: 2026-08-06

## Problema

Al pulsar F5 (recargar la app), si el tema estaba en modo oscuro, la app vuelve al modo claro. Solo al entrar a la pantalla de Configuración (Settings) la app recupera el modo oscuro.

## Análisis / Causa raíz

- El tema se aplica mediante el atributo `data-theme` en `<html>` (CSS vars definidas en `src/App.vue`, bloques `html[data-theme="light"]` y `html[data-theme="dark"]`). Sin el atributo, se usa la paleta clara por defecto.
- El store `useThemeStore` (`src/presentation/stores/themeStore.ts`) es el único encargado de aplicar `data-theme`, pero **solo se instancia cuando `SettingsPage.vue` lo importa** (`src/presentation/pages/SettingsPage.vue`).
- El store cargaba la preferencia guardada dentro de `onMounted` (`loadFromStorage()`). `onMounted` en un setup store se ancla al componente activo al crearse el store.
- Consecuencia en F5:
  1. La app arranca en Home/Login; el `themeStore` nunca se instancia → `data-theme` no se setea → tema claro.
  2. Al navegar a Configuración, el store se crea, `onMounted` dispara `loadFromStorage()` → lee `"dark"` de `localStorage` y aplica el tema oscuro.

Coincide exactamente con el comportamiento reportado.

- Bonus (flash de tema): aun creando el store al arrancar, el `watchEffect` del store aplica `"system"` antes de que `onMounted` lea `localStorage`, causando un parpadeo claro→oscuro.

## Solución

Enfoque profesional en 2 capas (anti-FOUC):

### 1. `index.html` — script inline en `<head>` (antes del primer render)

Aplicar el tema de forma **síncrona y antes de que cargue el bundle de JS**:

```html
<script>
  (function () {
    try {
      var stored = localStorage.getItem("app-theme");
      var mode =
        stored === "light" || stored === "dark" || stored === "system"
          ? stored
          : "system";
      var dark =
        mode === "dark" ||
        (mode === "system" &&
          window.matchMedia("(prefers-color-scheme: dark)").matches);
      document.documentElement.dataset.theme = dark ? "dark" : "light";
    } catch (e) {
      document.documentElement.dataset.theme = "light";
    }
  })();
</script>
```

Esto garantiza que F5 respete el modo guardado desde el primer píxel.

### 2. `src/presentation/stores/themeStore.ts` — inicialización síncrona

- `mode` se inicializa leyendo `localStorage` en el momento de crear el store (sin depender de `onMounted`), con `try/catch`.
- El listener de `window.matchMedia("(prefers-color-scheme: dark)")` se registra al crear el store (Pinia es singleton, no duplica listeners).
- Se mantienen `watchEffect`, `applyThemeClass`, `setMode` y `loadFromStorage` (para compatibilidad).

Con esto el store es correcto sin importar dónde/cuándo se instancie.

### 3. `src/main.ts` — instanciar el store en el bootstrap

```ts
import { useThemeStore } from "./presentation/stores/themeStore";
// ...
app.use(pinia);
app.use(router);

useThemeStore();
```

Instanciar el store de forma eager al arrancar activa desde el inicio el `watchEffect` y el listener de `prefers-color-scheme` (necesario para el modo `"system"`).

### 4. Sin cambios

- `src/presentation/pages/SettingsPage.vue` (el `v-model` con `computed` ya funciona).

## Verificación

```bash
pnpm build   # vue-tsc --noEmit + vite build
```

Prueba manual: fijar modo oscuro en Configuración → F5 → debe arrancar en oscuro sin parpadeo.
