# Análisis de funcionalidad para deshacer cambios de precios de costo

## Rol

Actuá como desarrollador, analista y arquitecto de software especializado en:

* Vue.js + TypeScript.
* Rust.
* Tauri 2.0.
* Clean Architecture.
* SOLID.
* Diseño basado en casos de uso.
* Componentes pequeños y reutilizables.
* Persistencia y consistencia de datos.
* Diseño de operaciones transaccionales.
* Testabilidad.

Este análisis debe realizarse sobre el proyecto existente.

El objetivo inicial **no es implementar directamente la solución**, sino analizar la problemática, estudiar las alternativas posibles y determinar cuál es la solución arquitectónicamente más adecuada.

---

# Contexto

En el módulo de Stock se está incorporando una funcionalidad para actualizar el **precio de costo de los artículos que poseen stock**.

La operación permite seleccionar artículos mediante filtros como:

* Categoría.
* Subcategoría.
* Proveedor.

Posteriormente se puede aplicar una modificación porcentual al precio de costo.

Por ejemplo:

```text
Precio original:       $1.000
Incremento:              20%
Nuevo precio:           $1.200
```

El problema aparece cuando el usuario intenta "deshacer" la operación aplicando el porcentaje contrario:

```text
$1.200 - 20% = $960
```

Esto demuestra que:

```text
+20% y -20%
```

**no son operaciones inversas**.

La operación correcta para revertir matemáticamente un incremento del 20% sería:

```text
$1.200 / 1.20 = $1.000
```

Sin embargo, el objetivo de esta funcionalidad **no debería ser simplemente calcular porcentajes inversos**, sino analizar cómo implementar correctamente una operación de **deshacer los últimos cambios realizados sobre los precios de costo**.

---

# Objetivo funcional

Se necesita incorporar una funcionalidad que permita:

> Deshacer una operación anterior de modificación de precios de costo y restaurar los valores que tenían los artículos antes de dicha operación.

La funcionalidad debe permitir recuperar el estado anterior real de los precios, independientemente de cómo se haya calculado la modificación.

Por ejemplo:

```text
Estado inicial

Artículo A → $1.000
Artículo B → $2.000
Artículo C → $5.000
```

El usuario aplica:

```text
+20%
```

Resultado:

```text
Artículo A → $1.200
Artículo B → $2.400
Artículo C → $6.000
```

Posteriormente selecciona:

```text
Deshacer última operación
```

El sistema debería restaurar:

```text
Artículo A → $1.000
Artículo B → $2.000
Artículo C → $5.000
```

No debería intentar deducir los valores anteriores a partir del porcentaje utilizado.

---

# Problema principal a analizar

Analizá cómo implementar correctamente un mecanismo de **Undo** para modificaciones de precios de costo.

La solución debe considerar que una operación puede afectar múltiples artículos simultáneamente.

Por ejemplo:

```text
Operación #152

Filtro:
Categoría = Bebidas
Proveedor = Proveedor X

Modificación:
+20%

Artículos afectados:
1001 → $1.000 → $1.200
1002 → $2.500 → $3.000
1003 → $4.000 → $4.800
```

El sistema debería considerar esta modificación como **una única operación lógica**, aunque internamente se actualicen múltiples registros.

---

# 1. Analizar el modelo de datos existente

Antes de proponer una solución, inspeccioná la estructura real de la base de datos.

Analizá:

* Artículos.
* Stock.
* Precio de costo.
* Categorías.
* Subcategorías.
* Proveedores.
* Claves primarias.
* Relaciones.
* Restricciones.
* Índices.
* Migraciones existentes.

Determiná dónde se almacena actualmente el precio de costo y cómo se realiza su actualización.

No asumas nombres de tablas, columnas o relaciones.

Utilizá exclusivamente la estructura real del proyecto.

---

# 2. Analizar la funcionalidad actual de actualización de precios

Localizá la funcionalidad implementada para modificar los precios de costo.

Analizá:

* Frontend.
* Components Vue.
* Composables.
* Stores.
* Commands de Tauri.
* Casos de uso.
* Repositories.
* Queries SQL.
* Transacciones.
* Validaciones.
* Manejo de errores.

Determiná exactamente:

> ¿Qué ocurre actualmente desde que el usuario presiona "Actualizar precios" hasta que el nuevo precio queda persistido?

Esto es fundamental para diseñar correctamente el mecanismo de Undo.

---

# 3. Analizar alternativas para implementar Undo

Evaluá al menos las siguientes estrategias.

## Alternativa A — Revertir mediante cálculo matemático

Ejemplo:

```text
Precio actual = $1.200
Porcentaje original = 20%

Precio anterior = $1.200 / 1.20
Precio anterior = $1.000
```

Analizá:

* Ventajas.
* Desventajas.
* Problemas con redondeos.
* Problemas con múltiples modificaciones sucesivas.
* Problemas si se modifican manualmente los precios posteriormente.
* Problemas si diferentes artículos tenían valores diferentes.
* Problemas con porcentajes combinados.
* Problemas con operaciones que no sean porcentuales.

Determiná si esta alternativa debería descartarse o si puede tener algún uso específico.

---

# 4. Alternativa B — Guardar el valor anterior

Analizá la posibilidad de registrar el valor anterior y posterior de cada artículo cuando se realiza una modificación.

Conceptualmente:

```text
Artículo
Precio anterior
Precio nuevo
Operación
Fecha/hora
```

Por ejemplo:

```text
Operación #152

Artículo 1001
Anterior: $1.000
Nuevo:    $1.200

Artículo 1002
Anterior: $2.500
Nuevo:    $3.000
```

Analizá:

* Estructura de datos necesaria.
* Relaciones.
* Tamaño esperado del historial.
* Índices.
* Integridad.
* Recuperación.
* Eliminación o conservación histórica.
* Impacto en las operaciones actuales.

---

# 5. Alternativa C — Historial de operaciones

Analizá una arquitectura basada en operaciones.

Por ejemplo:

```text
PriceChangeOperation
        │
        ├── OperationItem
        │      ├── ArticleId
        │      ├── PreviousCost
        │      └── NewCost
        │
        ├── Tipo de operación
        ├── Porcentaje aplicado
        ├── Fecha
        └── Usuario/contexto
```

No tomes estos nombres como definitivos.

Determiná cómo debería modelarse realmente en función de la arquitectura y base de datos existente.

Analizá si esta solución permitiría:

* Deshacer la última operación.
* Consultar operaciones anteriores.
* Saber qué artículos fueron afectados.
* Saber qué valores tenían antes.
* Evitar depender de cálculos matemáticos inversos.
* Auditar cambios.
* Futuramente rehacer una operación.
* Recuperar un estado anterior.

---

# 6. Definir qué significa "último cambio"

Este punto es especialmente importante.

Analizá cómo debería determinarse cuál es la operación que puede deshacerse.

Considerá escenarios como:

```text
Operación 1 → +10%
Operación 2 → +20%
Operación 3 → -5%
```

Si el usuario presiona Undo:

```text
Debe deshacerse solamente la Operación 3.
```

Luego:

```text
Undo nuevamente
```

debería deshacer:

```text
Operación 2.
```

Analizá si realmente se necesita un stack de operaciones:

```text
Operation 1
Operation 2
Operation 3
     ↑
   Undo
```

y cómo debería manejarse.

---

# 7. Analizar modificaciones posteriores

Considerá este escenario:

```text
Precio inicial: $1.000

Operación 1:
+20%

Resultado:
$1.200

Luego un usuario modifica manualmente:
$1.200 → $1.350
```

Posteriormente se intenta:

```text
Undo Operación 1
```

Analizá qué debería ocurrir.

No asumas automáticamente la respuesta.

Evaluá alternativas:

### Alternativa 1

Restaurar `$1.000`.

### Alternativa 2

No permitir Undo porque el precio fue modificado posteriormente.

### Alternativa 3

Advertir que existen modificaciones posteriores y solicitar confirmación.

### Alternativa 4

Otro mecanismo.

Determiná cuál es la alternativa más segura y coherente con el sistema.

---

# 8. Analizar operaciones parciales

Considerá una operación que afecta:

```text
100 artículos
```

pero ocurre un error al actualizar el artículo número 57.

Analizá:

* Si debe revertirse toda la operación.
* Si se deben conservar los primeros 56 cambios.
* Cómo garantizar atomicidad.
* Qué papel debería cumplir una transacción.
* Cómo debería registrarse la operación.
* Qué debería visualizar el usuario.

La prioridad debe ser evitar estados parcialmente actualizados.

---

# 9. Analizar concurrencia y consistencia

Considerá escenarios donde:

* Se realiza una operación de actualización.
* Otro proceso modifica un precio.
* El usuario intenta hacer Undo.
* Dos operaciones se ejecutan rápidamente.
* La aplicación se cierra durante la actualización.
* La aplicación se cierra después de actualizar pero antes de registrar el historial.

Analizá cómo garantizar consistencia entre:

```text
Precio actual
      +
Historial de operaciones
```

Idealmente, la actualización y el registro histórico deberían formar parte de una misma unidad transaccional cuando la tecnología utilizada lo permita.

---

# 10. Diseñar los casos de uso

La funcionalidad debe respetar el principio de **casos de uso atómicos**.

Analizá qué casos de uso serían necesarios.

Por ejemplo:

```text
Aplicar modificación de precio
Registrar operación de modificación
Obtener última operación
Deshacer operación
Obtener historial de operaciones
```

Esta lista es solamente orientativa.

Determiná los casos de uso realmente necesarios.

Cada caso de uso debe tener una responsabilidad clara.

Evitá crear un servicio gigantesco como:

```text
PriceService
```

que termine realizando consultas, cálculos, actualizaciones, historial, Undo y validaciones.

---

# 11. Clean Architecture y SOLID

Analizá cómo incorporar esta funcionalidad respetando:

* Single Responsibility Principle.
* Open/Closed Principle.
* Dependency Inversion Principle.
* Bajo acoplamiento.
* Alta cohesión.
* Separación entre dominio, aplicación e infraestructura.

La lógica para determinar si una operación puede deshacerse no debería depender directamente de Vue ni de Tauri.

Los Commands de Tauri deberían actuar como adaptadores.

Conceptualmente:

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

Validá esta estructura contra la arquitectura real del proyecto.

---

# 12. Diseñar el modelo de historial

Proponé el modelo necesario para almacenar las operaciones.

Determiná si se necesita:

### Cabecera de operación

Por ejemplo:

```text
Operation
```

con información como:

* ID.
* Fecha/hora.
* Tipo.
* Porcentaje.
* Filtro aplicado.
* Cantidad de artículos afectados.
* Estado.
* Otros datos relevantes.

### Detalle de operación

Por ejemplo:

```text
OperationItem
```

con:

* ID de operación.
* ID del artículo.
* Precio anterior.
* Precio nuevo.

No tomes estos campos como definitivos.

Determiná qué información realmente es necesaria.

---

# 13. Analizar almacenamiento del filtro

Evaluá si vale la pena almacenar los filtros utilizados para generar una operación.

Por ejemplo:

```text
Categoría = 5
Subcategoría = 12
Proveedor = 8
Porcentaje = 20
```

Analizá si esto aporta valor para:

* Auditoría.
* Explicación de la operación.
* Recuperación.
* Historial.
* Debugging.
* Soporte al usuario.

También determiná si debería almacenarse el filtro original o solamente los artículos afectados.

---

# 14. Analizar el comportamiento del frontend

Diseñá conceptualmente cómo debería presentarse Undo.

Evaluá opciones como:

```text
[ Deshacer última actualización ]
```

o:

```text
Última operación:
+20%
125 artículos modificados
Hace 2 minutos

[Deshacer]
```

Analizá:

* Dónde debería ubicarse.
* Cuándo debería estar habilitado.
* Qué información debería mostrar.
* Qué confirmación debería solicitar.
* Cómo informar el resultado.
* Qué hacer si no existen operaciones para deshacer.
* Cómo actualizar la tabla después del Undo.

No implementes todavía.

---

# 15. Analizar historial futuro

Aunque inicialmente solamente se necesita:

> Deshacer la última operación.

Evaluá si la arquitectura propuesta debería permitir posteriormente:

```text
Historial

Operación 10
Operación 11
Operación 12

[Deshacer]
```

y eventualmente:

```text
Deshacer operación específica
```

No implementes funcionalidades innecesarias ahora, pero evitá diseñar una solución que impida evolucionar hacia ellas.

---

# 16. Auditoría

Analizá si esta funcionalidad debería servir también como mecanismo de auditoría.

Considerá:

* Qué usuario realizó el cambio, si el sistema maneja usuarios.
* Fecha/hora.
* Valores anteriores.
* Valores nuevos.
* Cantidad de artículos.
* Tipo de modificación.
* Filtros utilizados.
* Operación de Undo.

Determiná qué información ya existe en el sistema y cuál sería necesario incorporar.

---

# 17. Tests

Analizá los tests necesarios para garantizar que Undo sea seguro.

Como mínimo, evaluá casos como:

```text
Precio 1000
+20%
= 1200
Undo
= 1000
```

También:

```text
1000
+20%
= 1200

+10%
= 1320

Undo
= 1200
```

Y:

```text
1000
+20%
= 1200

+10%
= 1320

Undo
= 1200

Undo
= 1000
```

Además:

* Operación sin artículos.
* Operación con múltiples artículos.
* Error durante actualización.
* Error durante Undo.
* Operación concurrente.
* Modificación posterior.
* Precios con decimales.
* Redondeos.
* Valores límite.
* Aplicación cerrada durante una operación.
* Historial inexistente.
* Intento de Undo cuando no existe una operación válida.

Los tests deben concentrarse especialmente en la lógica de negocio y en la integridad de los datos.

---

# 18. Comparación de alternativas

Al finalizar el análisis, generá una comparación entre las soluciones estudiadas.

Como mínimo:

| Alternativa              | Ventajas | Desventajas | Riesgos | Recomendación |
| ------------------------ | -------- | ----------- | ------- | ------------- |
| Invertir porcentaje      |          |             |         |               |
| Guardar valor anterior   |          |             |         |               |
| Historial de operaciones |          |             |         |               |

Explicá cuál recomendás y por qué.

No elijas una solución solamente porque sea más sencilla de implementar.

Priorizá:

1. Integridad de datos.
2. Correcto comportamiento de Undo.
3. Atomicidad.
4. Testabilidad.
5. Mantenibilidad.
6. Evolución futura.
7. Compatibilidad con Clean Architecture.
8. Bajo acoplamiento.
9. Simplicidad razonable.

---

# 19. Propuesta final

Después del análisis, proponé una solución concreta.

La propuesta debe incluir:

* Modelo de datos.
* Tablas nuevas, si fueran necesarias.
* Relaciones.
* Casos de uso.
* Repositories.
* DTOs.
* Commands de Tauri.
* Componentes Vue.
* Composables.
* Flujo de actualización.
* Flujo de Undo.
* Estrategia transaccional.
* Manejo de errores.
* Validaciones.
* Estrategia de testing.
* Archivos a modificar.
* Archivos nuevos.

Mostrá también el flujo completo:

```text
Usuario
 ↓
Actualiza precios
 ↓
Validación
 ↓
Crear operación
 ↓
Guardar valores anteriores
 ↓
Actualizar precios
 ↓
Confirmar transacción
 ↓
Operación disponible para Undo
```

Y:

```text
Usuario
 ↓
Undo
 ↓
Obtener última operación válida
 ↓
Validar que pueda deshacerse
 ↓
Restaurar valores anteriores
 ↓
Marcar operación como deshecha
 ↓
Confirmar transacción
```

Adaptá este flujo a la arquitectura real del proyecto.

---

# 20. Restricción importante

**No implementes código todavía.**

Primero presentá el análisis técnico completo.

No modifiques archivos.

No crees migraciones.

No agregues tablas.

No agregues componentes.

No agregues casos de uso.

Primero quiero revisar y aprobar la solución arquitectónica.

Si encontrás información que necesitás para tomar una decisión y no existe en el proyecto, indicá exactamente qué información falta.

No inventes datos.

---

# Resultado esperado

El resultado final debe permitir responder con claridad:

> ¿Cómo podemos implementar un mecanismo confiable para deshacer las últimas modificaciones de precios de costo sin depender de cálculos matemáticos inversos y sin poner en riesgo la integridad de los datos?

La solución debe considerar que **una modificación porcentual es una operación de negocio**, no simplemente una operación matemática sobre un precio.

El diseño debe permitir recuperar el **valor anterior real** de cada artículo afectado.

Priorizá:

* Correctitud.
* Integridad de datos.
* Atomicidad.
* Trazabilidad.
* Clean Architecture.
* SOLID.
* Casos de uso atómicos.
* Bajo acoplamiento.
* Alta cohesión.
* Testabilidad.
* Reutilización.
* Evolución futura.

Al finalizar, **esperá mi aprobación antes de realizar cualquier implementación**.
