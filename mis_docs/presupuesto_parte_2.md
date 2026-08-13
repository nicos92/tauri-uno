# presupuesto parte 2

**Rol:** Actúa como un Arquitecto de Software y Desarrollador Full Stack Senior en Rust, Vue.js y SQLite.

**Stack Tecnológico:**

* **Framework App:** Tauri 2.0
* **Backend:** Rust (Comandos Tauri, Manejo de estado/DB)
* **Frontend:** Vue.js (Composition API, Pinia, TypeScript)
* **Base de Datos:** SQLite

**Contexto del Proyecto:**

Ya cuento con la funcionalidad para registrar presupuestos y sus detalles (`presupuestos` y `detalle_presupuestos`) directamente desde la pantalla de nueva venta en mi aplicación Tauri 2.0.

**Objetivo:**

Crear un **módulo dedicado a la Gestión de Presupuestos** en la interfaz que permita listar, consultar el detalle, anular/eliminar y convertir un presupuesto guardado en una venta formal.

**Requerimientos Funcionales y Técnicos:**

1. **Frontend (Vista y Gestión en Vue.js):**

* **Listado General:** Tabla/vista para visualizar todos los presupuestos con filtros básicos (por estado: *pendiente, convertido, anulado*, rango de fechas, cliente o término de búsqueda).
* **Modal/Vista de Detalle:** Al seleccionar un presupuesto, mostrar el desglose de sus artículos, totales y datos del cliente.
* **Acciones de Gestión:**
* **Eliminar / Anular:** Opción para dar de baja un presupuesto cambiando su estado a *anulado* (o borrado físico de la base de datos si corresponde).
* **Convertir a Venta:** Botón para cargar directamente los ítems del presupuesto al carrito de la pantalla de "Nueva Venta" o ejecutar la conversión directa a una entrada en `ventas` y `detalle_ventas`.

1. **Backend en Rust (Comandos Tauri):**

* `obtener_presupuestos`: Retornar la lista paginada o filtrada de presupuestos.
* `obtener_detalle_presupuesto`: Devolver el encabezado y la lista de artículos de un presupuesto específico por su ID.
* `cambiar_estado_presupuesto`: Comando para actualizar el estado del presupuesto (ej. a *anulado* o *vencido*).
* `convertir_presupuesto_a_venta`: Lógica o transacción en Rust que:

1. Verifique disponibilidad de stock actual para los artículos del presupuesto.
2. Inserte los registros correspondientes en `ventas` y `detalle_ventas`.
3. Descuente el stock de los productos involucrados.
4. Actualice el estado del presupuesto a *convertido*.

5. **Manejo de Reglas de Negocio:**

* Un presupuesto en estado *convertido* o *anulado* no debería poder volver a convertirse en venta para evitar duplicaciones de stock o facturación.
* Notificar al usuario desde Vue.js en caso de falta de stock al intentar convertir un presupuesto antiguo.

**Entregables Esperados:**

1. **Comandos en Rust (`#[tauri::command]`):** Implementación de las funciones de consulta, actualización de estado y la transacción atómica de conversión a venta en SQLite.
2. **Componente Vue.js:** Estructura del componente principal del módulo (`PresupuestosView.vue`) con la tabla de listado, filtros y modales de acción.
3. **Servicio / Pinia Store:** Métodos TypeScript para interactuar con los nuevos comandos mediante `invoke('...')`.

---
