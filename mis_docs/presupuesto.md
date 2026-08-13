# Modulo presupuesto

## Rol

Actúa como un Arquitecto de Software y Desarrollador Senior en Rust y Vue.js.

## Stack Tecnológico

Framework App: Tauri 2.0

Backend: Rust (Comandos Tauri, Manejo de estado, SQLite mediante  rusqlite)

Frontend: Vue.js (Composition API, Pinia para el estado del carrito, TypeScript)

Base de Datos: SQLite

## Contexto del Proyecto

Tengo una aplicación de escritorio donde el módulo de ventas funciona correctamente. El backend en Rust procesa el registro en la base de datos SQLite sobre las tablas existentes ventas y detalle_ventas, y el frontend en Vue administra el carrito de compras actual.

## Objetivo

Permitir que desde la misma pantalla donde se arma la venta, el usuario pueda guardar el carrito actual como un presupuesto, reutilizando la lógica de interfaz existente pero aislando la persistencia de datos.

## Requerimientos

1. Diseño de Base de Datos (SQLite)

    Crear las tablas presupuestos y detalle_presupuestos mediante migraciones SQLite, clonando la estructura de ventas y detalle_ventas.

    Incluir campos propios de cotizaciones: estado (pendiente, aprobado, vencido, convertido), fecha_vencimiento y referencia opcional a cliente_id.

2. Backend en Rust (Tauri Commands)

    Definir las estructuras/DTOs correspondientes en Rust (Presupuesto, DetallePresupuesto).

    Implementar un comando Tauri crear_presupuesto que ejecute una transacción SQLite atómica para insertar el encabezado y sus detalles.

    Regla de negocio: Guardar un presupuesto no debe decrementar el stock en la tabla de productos.

3. Frontend en Vue.js

    Ajustar la vista del módulo de ventas para incluir una acción/botón secundario "Guardar Presupuesto" junto al de "Procesar Venta".

    Extender el store de Pinia (o el estado del carrito) para invocar el comando Tauri crear_presupuesto pasando la carga útil (payload) actual.

4. Conversión a Venta

    Diseñar la estructura para que más adelante se pueda cargar un presupuesto existente de vuelta al carrito de Vue o convertirlo a venta directamente desde Rust.

## Entregables Esperados

1. SQL DDL: Sentencias CREATE TABLE para SQLite con sus índices y relaciones (FOREIGN KEY).

2. Código Rust: Estructuras serde y la función del comando #[tauri::command] para la inserción.

3. Código Vue.js / TypeScript: Integración en el store o componente para consumir el nuevo comando Tauri (invoke('crear_presupuesto', ...)).
