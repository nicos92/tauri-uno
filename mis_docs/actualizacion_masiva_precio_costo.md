# Análisis e incorporación de actualización masiva de precios de costo

## Rol

Actuá como desarrollador, analista y arquitecto de software especializado en:

* Vue.js + TypeScript para frontend.
* Rust para backend.
* Tauri 2.0 como framework de aplicación.
* Clean Architecture.
* Principios SOLID.
* Diseño orientado a casos de uso.
* Componentes pequeños, cohesivos y reutilizables.
* Código mantenible y fácilmente testeable.

Tu objetivo inicial **no es comenzar a implementar inmediatamente**. Primero tenés que realizar un análisis completo del proyecto y determinar la mejor forma de incorporar el nuevo requerimiento respetando la arquitectura existente.

---

## Nuevo requerimiento funcional

Dentro del módulo de **Stock** se necesita incorporar una nueva funcionalidad para permitir actualizar el **precio de costo de los artículos que actualmente poseen stock**.

La funcionalidad debe estar disponible desde el módulo de Stock mediante un botón que permita acceder a una nueva pantalla específica para realizar esta operación.

La nueva pantalla debe permitir:

1. Visualizar los artículos que poseen stock.
2. Filtrar los artículos sobre los cuales se desea trabajar.
3. Seleccionar filtros relacionados con:

   * Categoría.
   * Subcategoría.
   * Proveedor.
4. Mostrar los artículos resultantes de los filtros.
5. Permitir actualizar el precio de costo de los artículos seleccionados o resultantes, según lo que determine el análisis funcional y técnico.
6. Evitar modificar artículos que no correspondan al conjunto filtrado.
7. Mantener consistencia entre el stock y la información del artículo.

---

## Contexto del modelo de datos

Actualmente la relación conceptual de los datos es aproximadamente la siguiente:

```text
Categoría
   │
   └── Subcategoría
          │
          └── Artículo
                 │
                 ├── Proveedor
                 │
                 └── Stock
                        └── Precio de costo
```

Las relaciones conocidas son:

* Un artículo pertenece a una única subcategoría.
* Una subcategoría pertenece a una única categoría.
* Un artículo pertenece a un único proveedor.
* Un artículo está relacionado con una entidad o tabla de Stock.
* La información del **precio de costo** se encuentra asociada al Stock.

No asumas que esta representación conceptual coincide exactamente con la implementación actual.

Primero inspeccioná el código y la base de datos para determinar cómo están implementadas realmente estas relaciones.

---

# Objetivo del análisis

Antes de modificar código, analizá exhaustivamente el proyecto.

No realices cambios de código durante esta primera etapa.

Quiero que determines:

* Qué partes existentes del sistema pueden reutilizarse.
* Qué partes deberían modificarse.
* Qué nuevas piezas arquitectónicas serían necesarias.
* Qué impacto tendría la nueva funcionalidad.
* Qué riesgos o inconsistencias existen.
* Qué alternativas de implementación existen.
* Cuál alternativa considerás más adecuada y por qué.

---

# 1. Analizar la arquitectura existente

Identificá cómo está organizado actualmente el proyecto.

Prestá especial atención a:

### Frontend

* Estructura de carpetas.
* Componentes Vue.
* Composables.
* Stores.
* Servicios.
* Modelos/interfaces TypeScript.
* Manejo de estado.
* Sistema de routing.
* Componentes reutilizables.
* Formularios.
* Tablas.
* Filtros existentes.
* Manejo de errores.
* Comunicación con Tauri/Rust.

### Backend

Analizá:

* Organización de módulos Rust.
* Commands de Tauri.
* Casos de uso existentes.
* Servicios.
* Repositories.
* Entidades.
* DTOs.
* Queries.
* Manejo de errores.
* Transacciones.
* Acceso a base de datos.
* Inyección o composición de dependencias.
* Convenciones utilizadas actualmente.

Determiná si la arquitectura actual sigue realmente Clean Architecture o si existen dependencias que deberían tenerse en cuenta antes de incorporar la funcionalidad.

---

# 2. Analizar el modelo de datos

Inspeccioná las entidades/tablas relacionadas con:

* Artículos.
* Stock.
* Categorías.
* Subcategorías.
* Proveedores.

Determiná exactamente:

* Claves primarias.
* Claves foráneas.
* Relaciones.
* Cardinalidades.
* Campos relevantes.
* Nombre real del campo que representa el precio de costo.
* Cómo se relaciona Stock con Artículo.
* Cómo se relaciona Artículo con Proveedor.
* Cómo se obtiene la categoría a partir del artículo.
* Si existen restricciones de integridad.
* Si existen índices relevantes.
* Si existen consultas reutilizables.

No inventes relaciones ni nombres de campos.

Basá el análisis en el código y esquema real del proyecto.

---

# 3. Analizar funcionalidades existentes

Buscá funcionalidades existentes relacionadas con:

* Consulta de stock.
* Listado de artículos.
* Búsqueda de artículos.
* Filtros.
* Categorías.
* Subcategorías.
* Proveedores.
* Actualización de precios.
* Actualización de stock.
* Operaciones masivas.
* Formularios de edición.
* Tablas reutilizables.

Determiná qué componentes, composables, casos de uso, repositories, DTOs o queries pueden reutilizarse.

La prioridad debe ser **reutilizar lo existente cuando tenga sentido**, evitando duplicar lógica.

---

# 4. Diseñar conceptualmente la nueva funcionalidad

Sin implementar todavía, definí cómo debería funcionar el flujo completo.

Por ejemplo:

```text
Stock
  │
  └── Actualizar precios de costo
          │
          ├── Filtros
          │     ├── Categoría
          │     ├── Subcategoría
          │     └── Proveedor
          │
          ├── Resultados
          │
          └── Actualización
```

Analizá también:

* Si los filtros deben combinarse mediante AND.
* Qué ocurre cuando no se selecciona ningún filtro.
* Si debería existir un límite máximo de resultados.
* Si se debe paginar.
* Cómo seleccionar artículos.
* Si el precio se modifica individualmente o de manera masiva.
* Si debería permitirse establecer un mismo precio para múltiples artículos.
* Qué validaciones deben realizarse.
* Qué ocurre si un artículo no posee stock.
* Qué ocurre si existen inconsistencias entre Artículo y Stock.
* Qué ocurre si la actualización falla parcialmente.

No tomes decisiones arbitrarias. Si existen varias alternativas, explicá las ventajas y desventajas de cada una.

---

# 5. Analizar los casos de uso

La funcionalidad debe diseñarse utilizando **casos de uso pequeños y atómicos**, evitando crear un único servicio que concentre toda la lógica.

Analizá qué casos de uso serían necesarios.

Por ejemplo, evaluá si tendría sentido separar responsabilidades como:

```text
Obtener categorías
Obtener subcategorías
Obtener proveedores
Buscar artículos con stock
Obtener artículos filtrados
Actualizar precio de costo
Actualizar precios de costo masivamente
```

No tomes esta lista como una implementación obligatoria.

Determiná cuáles casos de uso realmente corresponden según la arquitectura existente.

Cada caso de uso debe tener una responsabilidad clara y fácilmente testeable.

---

# 6. Analizar SOLID y Clean Architecture

Evaluá específicamente cómo incorporar esta funcionalidad respetando:

### Single Responsibility Principle

Evitar componentes, servicios o casos de uso que hagan demasiadas cosas.

### Open/Closed Principle

Evitar modificar innecesariamente funcionalidades existentes cuando puedan extenderse mediante abstracciones apropiadas.

### Liskov Substitution Principle

Verificar las abstracciones existentes cuando corresponda.

### Interface Segregation Principle

Evitar interfaces demasiado grandes.

### Dependency Inversion Principle

La lógica de negocio no debería depender directamente de detalles de infraestructura.

---

# 7. Frontend Vue.js

Diseñá conceptualmente la nueva pantalla identificando qué componentes serían necesarios.

Buscá evitar un componente Vue monolítico.

Evaluá una estructura similar a:

```text
CostPriceUpdateView
│
├── CostPriceFilters
├── CostPriceResults
│     └── CostPriceResultRow
└── CostPriceUpdateActions
```

Esto es solamente una referencia conceptual.

Analizá la estructura real del proyecto y proponé la organización más adecuada.

Determiná:

* Qué componentes deberían ser reutilizables.
* Qué lógica debería estar en composables.
* Qué estado debería manejarse localmente.
* Qué estado debería manejarse mediante store, si corresponde.
* Qué modelos TypeScript serían necesarios.
* Cómo debería comunicarse Vue con los commands de Tauri.
* Cómo manejar loading.
* Cómo manejar errores.
* Cómo informar operaciones exitosas.
* Cómo evitar estados inconsistentes de la UI.

---

# 8. Backend Rust + Tauri

Analizá cómo debería atravesar la operación las diferentes capas.

Por ejemplo:

```text
Vue
 ↓
Tauri Command
 ↓
Use Case
 ↓
Repository
 ↓
Database
```

Determiná si esta estructura coincide con la arquitectura existente.

Analizá:

* Commands necesarios.
* DTOs de entrada.
* DTOs de salida.
* Entidades involucradas.
* Interfaces/traits.
* Repositories.
* Queries.
* Transacciones.
* Manejo de errores.
* Validaciones.
* Concurrencia, si aplica.

La lógica de negocio debe permanecer fuera de los Commands de Tauri.

Los Commands deberían actuar principalmente como adaptadores entre la interfaz de Tauri y la aplicación.

---

# 9. Actualización masiva y transacciones

Analizá cuidadosamente la operación de actualización de precios.

Es especialmente importante determinar:

* Si debe ejecutarse dentro de una transacción.
* Qué ocurre si falla una actualización.
* Si la operación debe ser atómica.
* Si conviene realizar una única sentencia SQL o múltiples operaciones.
* Cómo evitar actualizaciones parciales.
* Cómo informar al frontend cuántos registros fueron afectados.
* Cómo evitar actualizar artículos que no correspondan al filtro.
* Cómo garantizar que solamente se actualice el precio de costo correspondiente al Stock.

Proponé la alternativa más segura y mantenible.

---

# 10. Seguridad e integridad de datos

Analizá posibles problemas como:

* Actualizaciones accidentales de demasiados artículos.
* Filtros vacíos que afecten todos los registros.
* Valores negativos.
* Valores inválidos.
* Precios excesivamente altos o bajos.
* Artículos sin stock.
* Registros inexistentes.
* Relaciones inválidas.
* Actualizaciones concurrentes.

Proponé validaciones apropiadas tanto en frontend como backend.

Las validaciones críticas de negocio deben existir en backend independientemente de las validaciones del frontend.

---

# 11. Testabilidad

Analizá desde el comienzo cómo debería poder probarse la funcionalidad.

Identificá posibilidades de:

### Backend

* Tests unitarios de casos de uso.
* Tests de validaciones.
* Tests de filtros.
* Tests de actualización.
* Tests de errores.
* Tests relacionados con transacciones, si corresponden.

### Frontend

* Tests de composables.
* Tests de componentes.
* Tests de validación.
* Tests del comportamiento de filtros.
* Tests de interacción con la actualización.

No agregues tests simplemente por cantidad.

Priorizá los tests que protejan la lógica de negocio y los casos donde exista mayor riesgo de modificar datos incorrectamente.

---

# 12. Impacto sobre código existente

Antes de proponer modificaciones, identificá:

* Archivos que deberían modificarse.
* Archivos que podrían reutilizarse.
* Archivos nuevos necesarios.
* Dependencias nuevas, si fueran necesarias.
* Migraciones de base de datos, si fueran necesarias.
* Riesgos de romper funcionalidades existentes.

Indicá claramente qué parte es:

```text
REUTILIZAR
MODIFICAR
CREAR
ELIMINAR
```

No propongas crear archivos nuevos cuando una abstracción existente pueda reutilizarse correctamente.

---

# 13. Propuesta de diseño

Una vez finalizado el análisis, presentá una propuesta concreta.

La propuesta debe incluir:

1. Arquitectura del flujo.
2. Componentes frontend.
3. Composables/stores necesarios.
4. DTOs.
5. Casos de uso.
6. Interfaces/traits.
7. Repositories.
8. Queries.
9. Commands de Tauri.
10. Validaciones.
11. Manejo de errores.
12. Estrategia transaccional.
13. Estrategia de testing.
14. Archivos a modificar.
15. Archivos a crear.

Para cada elemento explicá brevemente su responsabilidad.

---

# 14. No implementar todavía

En esta etapa **NO realices cambios en el código**.

Primero entregá el análisis.

El resultado esperado es un documento técnico que permita revisar y validar la arquitectura propuesta antes de comenzar la implementación.

Si detectás información faltante o ambigüedades importantes en el modelo de datos o en el comportamiento funcional, señalalas explícitamente.

No inventes información para completar esos vacíos.

---

# Resultado esperado

Quiero obtener un análisis que permita responder claramente:

> "¿Cuál es la forma más limpia, mantenible, reutilizable y testeable de incorporar esta nueva funcionalidad al proyecto actual sin romper la arquitectura existente?"

La solución debe priorizar:

* Clean Architecture.
* SOLID.
* Casos de uso atómicos.
* Bajo acoplamiento.
* Alta cohesión.
* Componentes pequeños.
* Reutilización.
* Testabilidad.
* Integridad de los datos.
* Claridad del código.
* Mínimo impacto sobre funcionalidades existentes.

Después de presentar el análisis, **esperá mi aprobación antes de comenzar la implementación**.
