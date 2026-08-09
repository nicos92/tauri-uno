# refactor confirm por modal

cuando se pide confirmar eliminar un registro mediante el boton de eliminar este llama a una uncion que utliza
la funcion nativa de java script `confirm`

## tarea

cambiar la funcion nativa de javascript `confirm` por un modal de confirmacion.

crear un elemento modal para que pueda ser reutilizado para toda la aplicacion.

---

## plan de implementación

Alcance acordado: **solo ConfirmDialog** (sin refactorizar los modales create/edit existentes).

### Arquitectura

Se replica el patrón ya existente de `useToasts.ts`:

- Estado module-level (singleton) en un composable `useConfirm()`.
- Un único componente global `<ConfirmDialog />` montado en `App.vue` (junto a `<Toasts />`).
- Las páginas solo importan el composable y llaman `await confirm({...})`, que devuelve `Promise<boolean>`.

### Archivos nuevos

1. **`src/presentation/composables/useConfirm.ts`**
   - `interface ConfirmOptions { title?, message, confirmText?, cancelText?, variant?: "danger" | "primary" }`
   - `active = ref<PendingConfirm | null>(null)` donde `PendingConfirm = { options, resolve }`.
   - `confirm(options): Promise<boolean>` — arma los defaults
     (`title: "Confirmar"`, `confirmText: "Confirmar"`, `cancelText: "Cancelar"`, `variant: "danger"`),
     guarda la promise pendiente y devuelve el promise.
     - **Guard anti doble-clic**: si ya hay una confirmación activa, las llamadas
       concurrentes resuelven `false` inmediatamente (evita modales apilados y doble borrado).
   - `resolveActive(value)` — cierra la confirmación activa resolviendo la promise
     (todas las rutas de cierre del modal pasan por acá).

2. **`src/presentation/components/ConfirmDialog.vue`**
   - `Teleport to="body"` + `z-index: 1500` (encima de los modales de página `1000`, debajo de toasts `2000`).
   - Overlay con `@click.self` → cancelar (patrón ya usado en el proyecto).
   - `@keydown.esc` → cancelar.
   - Autofocus del botón de confirmar al abrir (`watch` + `nextTick` + `.focus()`).
   - `onBeforeUnmount` → `resolveActive(false)` para no dejar promises colgadas.
   - Botón de confirmar rojo cuando `variant === "danger"`.

### Archivos modificados

3. **`src/App.vue`** — importar y montar `<ConfirmDialog />` junto a `<Toasts />`.

4. **9 páginas / 10 call sites** — importar `useConfirm`, destructure `const { confirm } = useConfirm();`
   (sombrea el global) y agregar `await` + objeto de options:

   | Página | Línea | Cambio |
   |--------|-------|--------|
   | `ArticulosPage.vue` | 152 | `if (await confirm({ message: "¿Está seguro de eliminar este artículo?" }))` |
   | `CategoriasPage.vue` | 61 | igual |
   | `SubCategoriasPage.vue` | 85 | igual |
   | `ProveedoresPage.vue` | 92 | igual |
   | `StockPage.vue` | 136 | igual |
   | `TiposVentaPage.vue` | 67 | igual |
   | `UsersPage.vue` | 79 | igual |
   | `CierresPage.vue` | 44, 60 | `if (!(await confirm({ message: \`...\`, confirmText: "Cerrar día" / "Reabrir" }))) return;` |
   | `VentasPage.vue` | 48 | `if (!(await confirm({ message: "...", confirmText: "Anular venta" }))) return;` |

### Problemas identificados y soluciones

1. **Sincrónico → asincrónico**: `confirm` nativo bloquea; el modal es async.
   Todos los handlers ya son `async`, solo falta el `await` en cada call site.
2. **Doble clic / race condition**: guard en el composable que resuelve `false`
   en llamadas concurrentes mientras hay una confirmación activa.
3. **Promise colgada por unmount**: resolver siempre en todas las rutas de cierre
   + fallback `onBeforeUnmount` en el componente.
4. **Z-index / stacking**: `Teleport to="body"` + `z-index: 1500`.
5. **Focus / accesibilidad**: autofocus del botón de confirmar, cierre con Escape
   y con clic en overlay.
6. **Mensajes dinámicos**: `message` es string; CierresPage ya interpola la fecha
   con template literal antes de llamar.
7. **Colisión con `confirm` global**: el destructure de `useConfirm()` sombrea el
   global (válido en TS strict) y deja el diff casi 1:1.

### Verificación

```bash
pnpm build   # corre vue-tsc --noEmit (strict) + vite build
```
