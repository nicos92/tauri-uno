# Estadísticas en el inicio

Se requiere la vista de estadísticas en la pantalla de inicio.

## Requerimientos

### Artículos
- Cantidad total de artículos.
- Cantidad de artículos que tienen stock asignado (artículos distintos con al menos un registro en `stock`).

### Usuarios
- Cantidad total de usuarios.
- Cantidad de usuarios activos e inactivos.

### Proveedores
- Cantidad total de proveedores.

### Ventas
- Cantidad de ventas realizadas en el día (excluyendo anuladas, consistente con el cierre del día).

## Decisión de vista

Las 4 tarjetas actuales (Artículos, Usuarios, Categorías, Sub Categorías) se reorganizan en un grid de **6 tarjetas**:

| # | Tarjeta | Valor principal | Detalle |
|---|---------|-----------------|---------|
| 1 | Artículos | total artículos | "X con stock" |
| 2 | Usuarios | total usuarios | "X activos · Y inactivos" |
| 3 | Proveedores | total proveedores | — |
| 4 | Ventas | ventas del día (no anuladas) | "Hoy · $ total" |
| 5 | Categorías | total categorías | — |
| 6 | Sub Categorías | total sub categorías | — |

Se mantienen las tarjetas de detalle "Stock Bajo" y "Categorías y Sub Categorías".

## Plan de implementación (pulido)

### Enfoque
100 % frontend en `src/presentation/pages/HomePage.vue`. Sin cambios en Rust. Se reutilizan los stores existentes y se agregan `useProveedoresStore` y `useVentasStore`. El conteo de ventas del día usa `Date` local (correcto con la zona horaria, mismo display que `VentasPage.vue`).

### Pasos

1. **Stores y datos**
   - Importar `useProveedoresStore` y `useVentasStore` desde `../stores`.
   - Agregar al `Promise.all` de `onMounted`: `proveedoresStore.fetchProveedores()` y `ventasStore.fetchVentas()`. Cada fetch ya maneja sus errores internamente, por lo que el `Promise.all` no falla si falta permiso.

2. **Computeds nuevos**
   - `totalProveedores`: longitud de `proveedoresStore.proveedores`.
   - `articulosConStock`: tamaño del `Set` de `id_articulo` distintos en `stockStore.stocks`.
   - `usuariosActivos` / `usuariosInactivos`: filtros por `user.active`.
   - `ventasDelDia`: ventas con `!anulada` y `fecha` dentro del día local (inicio del día → inicio del día siguiente, parseando con `new Date(v.fecha)`).
   - `totalVentasDelDia`: suma de `total` de `ventasDelDia`.

3. **Reorganizar el grid a 6 tarjetas** en el orden: Artículos, Usuarios, Proveedores, Ventas, Categorías, Sub Categorías.
   - **Artículos**: valor total + sub-estadística "X con stock".
   - **Usuarios**: valor total + sub-estadística "X activos · Y inactivos".
   - **Proveedores** (nueva): ícono de camión, clase de color `.teal`.
   - **Ventas** (nueva): ícono de dinero, clase `.red`, sub-estadística "Hoy · $ total".
   - **Categorías** y **Sub Categorías**: se conservan al final.

4. **Polish / detalles**
   - Estilo `.stat-sub` (texto pequeño bajo la etiqueta) y clases de color nuevas para los íconos.
   - Respetar permisos: si el usuario no tiene el permiso de vista correspondiente (`ver_articulos`, `ver_usuarios`, `ver_proveedor`, `ver_ventas`, `ver_categorias`, `ver_sub_categorias` vía `authStore.hasPermission`), mostrar "—" en la tarjeta en vez de "0" engañoso.
   - Fix de bug preexistente: `articulosBajoStock` filtra `cantidad < 100` pero la tarjeta dice "menos de 10 unidades" → corregir a `< 10`.

### Fuera de alcance
- Cambios en `src-tauri/**` (sin comandos nuevos, sin registro en `lib.rs`, sin permisos nuevos).
- Cambios en stores / repos / entidades (solo lectura desde HomePage).

### Verificación
- `pnpm build` (corre `vue-tsc --noEmit` + `vite build`). No hay lint ni tests.
- Chequeo manual: iniciar sesión como `admin`, ver 6 tarjetas con valores correctos; anular una venta del día y verificar que el conteo baja.
