# Plan Fase 2 — Impresión (PDF): solo el contenido deseado + botón en detalle de venta

## Objetivo

Resolver los dos puntos de impresión del documento:

1. **El PDF imprime toda la pantalla** en lugar de solo los detalles de la venta
   y la lista de artículos.
2. **Falta un botón "Generar PDF"** en el modal de detalle de venta
   (`VentasPage.vue`).

Cambios **solo de frontend**. No hay librería PDF en el proyecto (ni en
`package.json` ni en `Cargo.toml`): la impresión se hace con `window.print()` +
CSS `@media print`, y así se mantiene.

## Causa raíz del bug 1

En `NuevaVentaPage.vue` el bloque `@media print` (L728-772) está dentro de un
`<style scoped>`. Vue reescribe `body *` a `body *[data-v-<hash>]`, que solo
esconde el subtree **del propio componente**. El layout (`MainLayout.vue`:
sidebar/header), `#app` y el `router-view` quedan visibles al imprimir.

Problema secundario: `visibility: hidden` conserva el espacio ocupado, por lo
que aparecerían huecos en blanco.

## Estrategia de solución

1. **Mover las reglas `@media print` a CSS global** (el `<style>` sin `scoped`
   de `App.vue` ya existe en L10-49).
2. **Renderizar la zona imprimible con `Teleport to="body"`** para que quede
   como hijo directo de `<body>`. Esto permite el patrón de `display:none` (sin
   páginas en blanco) y evita el scoping.
3. **Reutilizar el patrón en ambas pantallas** (presupuesto del carrito y
   ticket de venta).

### CSS print global (en `App.vue`)

```css
@media print {
  body > * {
    display: none !important;
  }

  body > .print-area {
    display: block !important;
  }

  .print-area * {
    display: revert !important;
  }
}
```

Razones:

- `body > * { display: none }` oculta `#app` completo (todo el layout) y las
  notificaciones (que también se renderizan a nivel de `body` vía Teleport).
- `body > .print-area { display: block }` vuelve a mostrar únicamente la zona
  imprimible, que por Teleport es hija directa de `body`.
- `.print-area * { display: revert }` restaura el display natural de tablas,
  `p`, `tr`, `td`, etc. Funciona en Chromium/WebView2 (motor de Tauri).

### Zona imprimible con Teleport

```vue
<Teleport to="body">
  <div class="print-area" id="print-area">
    <!-- contenido del presupuesto / ticket -->
  </div>
</Teleport>
```

Notas:

- Si se usa `v-if` para el contenido, el `#print-area` debe existir en el DOM en
  el momento del `window.print()`. En `NuevaVentaPage` se renderiza siempre (el
  guard `generarPdf` ya corta si el carrito está vacío). En `VentasPage` se
  renderiza cuando `selectedVenta` no es null.
- Cada página define su propio `#print-area`; solo está montada la ruta activa,
  así que no hay conflictos de id.
- Quitar el bloque `@media print` scoped actual de `NuevaVentaPage` (L728-772) y
  el `.print-area { display: none }` scoped (L724-726) pasa a manejarse con el
  CSS global (en pantalla, el área se oculta con `display: none` por defecto y
  solo se muestra en print).

## Bug 1 — Arreglo en `NuevaVentaPage.vue`

1. `generarPdf()` (L193-196): sin cambios (ya llama `window.print()` con guard).
2. Envolver el bloque `.print-area` (L399-430) en `<Teleport to="body">`.
3. Eliminar de `<style scoped>`:
   - `.print-area { display: none; }` (L724-726).
   - Todo el bloque `@media print` (L728-772).
4. Mantener el botón "Generar PDF" (L223-232) con `canGenerarPresupuesto()`.

## Bug 2 — Botón PDF en detalle de venta (`VentasPage.vue`)

El modal de detalle (L147-212) ya tiene todos los datos en `selectedVenta`
(`VentaWithDetalle`): `id`, `username`, `fecha`, `items[]` (con `cod_articulo`,
`articulo`, `cantidad`, `precio_unitario`, `subtotal`), `subtotal`, `descuento`,
`total`, `observacion`, `anulada`.

Cambios:

1. **Script**:
   - Importar `usePermissions` (ya importado) y agregar `canGenerarPresupuesto`
     al destructuring (L12).
   - Agregar función:

     ```ts
     function generarPdfDetalle() {
       if (!selectedVenta.value) return;
       window.print();
     }
     ```

2. **Template** — en `modal-actions` (L203-210), agregar el botón:

   ```vue
   <button
       v-if="canGenerarPresupuesto()"
       @click="generarPdfDetalle"
       class="btn-secondary"
   >
       Generar PDF
   </button>
   ```

3. **Template** — agregar la zona imprimible con Teleport (fuera del modal):

   ```vue
   <Teleport to="body">
       <div v-if="selectedVenta" class="print-area" id="print-area">
           <h1>Venta N° {{ selectedVenta.id }}</h1>
           <p>Fecha: {{ new Date(selectedVenta.fecha).toLocaleString() }}</p>
           <p>Usuario: {{ selectedVenta.username }}</p>
           <p v-if="selectedVenta.observacion">
               Observación: {{ selectedVenta.observacion }}
           </p>
           <table>
               <thead>
                   <tr>
                       <th>Código</th>
                       <th>Artículo</th>
                       <th>Cantidad</th>
                       <th>Precio</th>
                       <th>Subtotal</th>
                   </tr>
               </thead>
               <tbody>
                   <tr v-for="item in selectedVenta.items" :key="item.id">
                       <td>{{ item.cod_articulo }}</td>
                       <td>{{ item.articulo }}</td>
                       <td>{{ item.cantidad }}</td>
                       <td>{{ formatMoney(item.precio_unitario) }}</td>
                       <td>{{ formatMoney(item.subtotal) }}</td>
                   </tr>
               </tbody>
           </table>
           <p class="print-line">Subtotal: {{ formatMoney(selectedVenta.subtotal) }}</p>
           <p v-if="selectedVenta.descuento > 0" class="print-line">
               Descuento ({{ selectedVenta.descuento }}%):
               −{{ formatMoney((selectedVenta.subtotal * selectedVenta.descuento) / 100) }}
           </p>
           <p class="print-total">Total: {{ formatMoney(selectedVenta.total) }}</p>
           <p class="print-obs">
               Estado: {{ selectedVenta.anulada ? "Anulada" : "Activa" }}
           </p>
       </div>
   </Teleport>
   ```

   El `#print-area` queda como hijo directo de `body` (Teleport) y el modal
   (overlay oscuro incluido) queda dentro de `#app`, que en print se oculta con
   `display: none`.

## Archivos tocados (resumen)

| Archivo | Cambio |
|---|---|
| `src/App.vue` | Agregar reglas globales `@media print` |
| `src/presentation/pages/NuevaVentaPage.vue` | Teleport del `.print-area`; quitar CSS print scoped |
| `src/presentation/pages/VentasPage.vue` | Botón "Generar PDF" + `.print-area` con Teleport |

## Verificación

- `pnpm build`.
- Prueba manual:
  - **Nueva Venta**: armar carrito, "Generar PDF" → el diálogo de impresión
    muestra solo el presupuesto (fecha, items, subtotal, descuento, total,
    observación), sin sidebar ni resto de la app. Sin páginas en blanco.
  - **Ventas**: abrir detalle → botón "Generar PDF" → imprime solo el ticket.
  - Repetir en tema dark (el contenido impreso debe verse legible).
