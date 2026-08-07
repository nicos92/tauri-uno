# Plan Módulo de Ventas parte dos

## Objetivo

Mejorar el módulo de ventas: reemplazar el modal de registro por una pantalla completa tipo punto de venta, agregar búsqueda de producto por código y descuento en porcentaje (persistido en DB), y permitir encadenar varias ventas seguidas sin salir de la pantalla.

## Requerimientos específicos

### RF-1 — Pantalla de registro (reemplaza el modal)

- La acción "Nueva Venta" navega a una pantalla completa en la ruta `/ventas/nueva` (ruta protegida con el permiso `crear_venta`), en lugar de abrir un modal.
- La pantalla tiene 4 secciones en orden vertical:
  1. Cabecera con observación y acciones.
  2. Resumen de totales (subtotal, descuento, total).
  3. Filtro de búsqueda de artículo en stock.
  4. Lista (carrito).
- "Cancelar" vuelve a `/ventas` descartando el carrito sin registrar.

### RF-2 — Búsqueda por código

- Campo de texto que filtra sobre `cod_articulo` y `articulo` de los artículos que tienen stock.
- Al presionar Enter con coincidencia exacta de código, el artículo se agrega directo al carrito y el campo de búsqueda se limpia y mantiene el foco (flujo rápido tipo POS).
- Con coincidencias parciales se muestra una lista clicable (máx. 20 resultados) con código, nombre, stock disponible y precio de venta; hacer clic agrega el artículo.
- Los artículos ya presentes en el carrito se excluyen de los resultados.

### RF-3 — Descuento en porcentaje

- Campo numérico `descuento` (0–100) en la sección de totales.
- `subtotal = Σ (cantidad × precio)` por línea.
- `total = subtotal × (1 − descuento / 100)`, redondeado a 2 decimales.
- Se muestra el monto descontado en pesos además del porcentaje.
- Se persiste en `ventas.descuento`. El backend valida que esté entre 0 y 100 (error `descuento_invalido`) y recalcula el total; no confía en el total enviado por el cliente.
- El subtotal no se guarda en la cabecera: se deriva de la suma de los subtotales de los items.

### RF-4 — Permanecer tras registrar

- Al registrar la venta correctamente: toast "Venta N° X registrada", se limpian carrito, descuento, observación y búsqueda, se permanece en la misma pantalla y se re-enfoca el campo de búsqueda.
- Botón "Vaciar" para limpiar el carrito sin registrar.

### RF-5 — Carrito

- Columnas: código, artículo, cantidad (editable, mínimo 0.01), precio (editable), subtotal, quitar.
- Si la cantidad supera el stock disponible y el usuario NO tiene el permiso `vender_sin_stock`, se muestra un aviso visual en la línea (el backend valida igualmente).
- Los resultados de búsqueda muestran stock disponible y precio de venta.

### RF-6 — Presupuesto PDF

- El botón "Generar PDF" (visible solo con permiso `generar_presupuesto`) imprime un presupuesto con fecha, lista de artículos, subtotal, descuento (%), total y observación.

### RF-7 — Listado y detalle

- La tabla del listado de ventas muestra el descuento y el total.
- El modal de detalle muestra subtotal, descuento y total.
- Anular venta: sin cambios (restaura stock).

## Decisiones de diseño

- Descuento único en % sobre el total de la venta (no por línea ni monto fijo).
- El backend es autoritativo para el cálculo del total (suma subtotales de items y aplica descuento).
- La pantalla de nueva venta es una ruta separada (`/ventas/nueva`).
- Dinero en `f64`/`REAL` (consistente con el resto del proyecto); redondeo a 2 decimales en el total.

## Impacto técnico

### Base de datos

- Nueva columna `descuento REAL NOT NULL DEFAULT 0` en `ventas` (agregada al `CREATE TABLE` y mediante `ALTER TABLE` protegido — consultando `PRAGMA table_info` — para DBs existentes de la v1, ya que no hay sistema de migraciones).

### Backend Rust

- `Venta`: campo `descuento`; `Venta::new` recibe descuento.
- `VentaWithDetalle`: campos `descuento` y `subtotal` (derivado de los items).
- `AppError::DescuentoInvalido` (código `descuento_invalido`).
- `VentaService::create` recibe y valida `descuento`.
- `SqliteVentaRepository::create` aplica el descuento al total; SELECTs devuelven `descuento` y `subtotal`.
- `CreateVentaRequest.descuento: Option<f64>` (None → 0.0).
- Sin cambios en `lib.rs`.

### Frontend Vue

- `types.ts`: `descuento` en `Venta` y `VentaWithDetalle` (+ `subtotal`); `descuento` opcional en `CreateVentaRequest`.
- Ruta `/ventas/nueva` con `NuevaVentaPage.vue`.
- `VentasPage.vue`: elimina el modal de creación; listado y detalle muestran subtotal/descuento/total.
- Helper compartido `formatMoney` en `src/presentation/utils/format.ts`.

## Verificación

- `cd src-tauri && cargo check` y `cargo clippy`.
- `pnpm build` (`vue-tsc --noEmit` + vite).
- Prueba manual: crear venta con descuento (persistencia y descuento de stock), detalle, anular, PDF.
