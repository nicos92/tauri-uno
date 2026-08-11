# Plan Fase 6 — Historial de ventas por cliente

## Objetivo

Mostrar el historial de ventas de cada cliente desde la página **Gestión de
Clientes**, aprovechando lo ya construido en la iteración de la relación
venta-cliente: la FK `ventas.cliente_id`, el comando backend
`get_ventas_por_cliente(user_id, cliente_id)` y el método
`ventasStore.getVentasPorCliente(clienteId)`.

No requiere cambios en backend ni en base de datos.

## Requerimientos específicos

### RF-1 — Acción "Ver ventas" por cliente

- En la tabla de `ClientesPage.vue` se agrega un botón (ícono) en la columna de
  acciones: "Ver ventas", visible para cualquier usuario con el permiso
  `ver_ventas` (se reutiliza `canViewVentas` del composable `usePermissions`).
- El botón está deshabilitado u oculto si el cliente es el por defecto
  ("Consumidor Final") y no tiene ventas asociadas (opcional; el listado
  simplemente puede salir vacío).

### RF-2 — Modal de historial

- Al hacer clic se abre un modal con el encabezado "Historial de ventas de
  {nombre cliente}".
- Se cargan las ventas con `ventasStore.getVentasPorCliente(cliente.id)`.
- Se muestran en una tabla con columnas: N°, Fecha, Tipo de venta, Subtotal,
  Descuento, Total y Estado (Activa/Anulada).
- Estados de UI: `loading` mientras carga y mensaje "Sin ventas" cuando el
  listado está vacío.
- Acciones dentro del historial:
  - **Generar PDF**: imprime el ticket de la venta (mismo `#print-area` de
    `VentasPage.vue`, con Teleport a `body`).
  - **Cerrar**: cierra el modal.

### RF-3 — Reuso del detalle

- El historial reutiliza los estilos de tabla, estados (`status-activa`/
  `status-anulada`) y `formatMoney` ya presentes en `VentasPage.vue`. Si es
  viable, se extrae el bloque de tabla + print-area a un componente reutilizable
  (`VentaTicket`/`VentaHistorial`); si no, se duplica el bloque con estilos
  scoped (patrón actual del proyecto).

## Decisiones de diseño

- El cliente por defecto ("Consumidor Final") puede tener ventas; no se
  restringe el historial para este caso (el seeder de cliente por defecto existe
  siempre).
- No se reimplementa `get_ventas_por_cliente` en el frontend: se usa el store ya
  existente.
- El permiso se evalúa con `canViewVentas()` (existe en `usePermissions.ts`).
  Verificar su nombre exacto al implementar.

## Impacto técnico

### Backend Rust

- Sin cambios: `get_ventas_por_cliente` ya existe en
  `venta_commands.rs`, `VentaService` y `SqliteVentaRepository`.

### Frontend Vue

- `src/presentation/pages/ClientesPage.vue`:
  - Importar `useVentasStore` y `VentaWithDetalle`.
  - Agregar columna de acciones con botón "Ver ventas".
  - Agregar modal de historial (estado `historialCliente`, `historialVentas`,
    `loadingHistorial`).
  - Agregar `#print-area` con Teleport para el ticket seleccionado.
- `src/presentation/stores/index.ts`: ya expone `getVentasPorCliente` (sin
  cambios).
- `src/presentation/composables/usePermissions.ts`: verificar que exista
  `canViewVentas` (agregar si falta).

## Verificación

- `pnpm build` (`vue-tsc --noEmit` + vite).
- `cd src-tauri && cargo test` (no debería tocar nada, pero se corre por
  regresión).
- Prueba manual: crear 2-3 ventas con distintos clientes (incluida una anulada),
  abrir Clientes → "Ver ventas" → listado correcto, totales y estado, PDF del
  ticket, cierre del modal.
