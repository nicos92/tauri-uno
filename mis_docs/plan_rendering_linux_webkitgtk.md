# Plan — Fix de renderizado en Linux (WebKitGTK)

## Objetivo

Corregir las diferencias visuales de la app en Linux respecto a Windows, donde:

1. **Los botones no muestran el texto dentro** (se clipea o queda invisible).
2. **Los combo box (`<select>`) se ven con colores cambiados** (fondo, flecha y
   opciones usan la paleta del tema GTK del sistema, no la del CSS).

## Causa raíz

Tauri no usa el mismo motor de renderizado en todos los OS:

- **Windows** → WebView2 (Chromium/Blink): renderiza los controles HTML como
  "cajas CSS"; `background`, `color`, `border` se aplican literalmente.
- **Linux** → **WebKitGTK**: pinta `<button>` y `<select>` con los widgets GTK
  del tema del sistema (Adwaita/libadwaita). El CSS no manda del todo:
  - Un `<button>` con `background` + `color` custom puede dibujar el fondo
    nativo GTK *encima* del nodo de texto, o el tema fuerza el color del texto
    y queda invisible/clipeado.
  - Un `<select>` pinta flechita, fondo y colores de opciones desde el tema
    GTK, ignorando `background`/`color` del CSS.

Agravantes en este repo:

- No existe un *reset* de controles de formulario: sin `appearance: none` ni
  `font: inherit`, cada control usa fuente y estilo nativo del GTK.
- La fuente del body (`-apple-system, "Segoe UI", BlinkMacSystemFont`) **no
  existe en Linux**; cae a `Arial` con métricas distintas, lo que también
  afecta al alto de línea y al clipeo del texto en botones.
- No hay `color-scheme` sincronizado con el tema de la app, así que WebKitGTK
  elige la paleta (clara/oscura) según el sistema, no según la app.

## Cambios

### 1. Reset global de controles — `src/App.vue` (style global, ~L12-29)

Agregar junto al reset existente (`* { margin/padding/box-sizing }`):

```css
button,
input,
select,
textarea {
  font: inherit;
  color: inherit;
}

button,
select {
  appearance: none;
  -webkit-appearance: none;
  background: none;
  border: none;
}
```

> Nota: no es necesario por archivo; el style de `App.vue` es global.

### 2. Flecha manual en los `<select>`

Al quitar `appearance: none`, el `<select>` pierde su flechita nativa. Agregar
un chevron SVG como `background-image` en las clases que estilan selects:

- `.form-group select` — `ArticulosPage.vue:527`, `SubCategoriasPage.vue:390`,
  `StockPage.vue:513`
- `.setting-select` — `SettingsPage.vue:113`
- `.tipo-select` — `NuevaVentaPage.vue:768`
- `.filter-input` (también usada en selects) — `AuditoriaPage.vue`

CSS base para el chevron:

```css
select {
  appearance: none;
  -webkit-appearance: none;
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'><path d='M1 1l5 5 5-5' fill='none' stroke='%236b7280' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'/></svg>");
  background-repeat: no-repeat;
  background-position: right 0.75rem center;
  padding-right: 2.25rem;
}
```

Estas propiedades se pueden agregar en el reset global de `App.vue` para cubrir
todos los selects, o por clase (más control sobre el color del stroke con
`var(--color-text-muted)`).

### 3. Botones — evitar clipeo por métrica de fuente

Agregar `line-height` explícito o `display: inline-flex` centrado a las clases
de botones para descartar clipeo por la caída a `Arial`:

```css
.btn-primary,
.btn-secondary,
.btn-danger {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  line-height: 1.2;
}
```

Afecta: `ArticulosPage.vue:423`, `ProveedoresPage.vue:298`,
`CategoriasPage.vue:212`, `SubCategoriasPage.vue:287`, `StockPage.vue:408`,
`UsersPage.vue:378`, `SettingsPage.vue:121`, `TiposVentaPage.vue:236`,
`VentasPage.vue:336`, `CierresPage.vue:210`, `AuditoriaPage.vue:210`,
`NuevaVentaPage.vue:674`, `PermissionsPage.vue:85` y `button` de
`LoginPage.vue:140`.

> Si se prefiere un cambio único, aplicar sobre `button` en el reset global de
> `App.vue`, pero con cuidado de no romper `.btn-icon` (debe seguir inline).

### 4. Sincronizar `color-scheme` con el tema de la app

En `src/App.vue` (o en `themeStore.ts` al setear `data-theme`), declarar el
`color-scheme` por tema:

```css
html[data-theme="light"] { color-scheme: light; }
html[data-theme="dark"]  { color-scheme: dark; }
```

Con esto WebKitGTK pinta fondo y opciones del dropdown con la paleta correcta
según el tema activo de la app, no según el del sistema.

## Verificación

- `pnpm build` (vue-tsc + vite).
- `cd src-tauri && cargo check` (no debería tocar Rust, solo para confirmar que
  nada se rompe).
- Prueba manual con `pnpm tauri dev` en Linux:
  - Los botones de cada página muestran el texto centrado y completo.
  - Los `<select>` (Articulos, Stock, SubCategorias, Settings, NuevaVenta,
    Auditoria) muestran la flechita y respetan la paleta del tema claro/oscuro.
  - Probar cambiar de tema claro → oscuro en Settings y verificar que selects y
    botones siguen legibles.

## Notas / riesgos

- `-webkit-appearance` no afecta a Windows/WebView2 (ya lo ignoraba), así que
  el cambio no debe alterar el render actual en Windows; probar igual.
- Los `<option>` no se estilizan con CSS en WebKitGTK; su color dependerá del
  `color-scheme`. Si se quiere más control, habría que migrar los selects a un
  dropdown custom (componente propio) — fuera del alcance de este plan.
- La flecha del select usa `--color-text-muted` (`#6b7280` claro / `#94a3b8`
  oscuro); si se hardcodea en el SVG hay que duplicar el `background-image`
  por tema o usar `currentColor` cuando el navegador lo soporte.
