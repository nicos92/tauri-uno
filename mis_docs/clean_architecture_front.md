# Objetivo

Revisa y analiza el código completo del frontend desarrollado con **Vue.js + TypeScript** con el objetivo de evaluar cómo puede mejorarse su arquitectura, mantenibilidad, legibilidad y testabilidad.

La refactorización debe orientarse hacia:

* Principios **SOLID**.
* **Clean Architecture**.
* Separación clara de responsabilidades.
* Componentes Vue pequeños, simples y reutilizables.
* Casos de uso independientes y atómicos.
* Bajo acoplamiento entre la UI, la lógica de negocio y la infraestructura.
* Alta cohesión.
* Código fácil de comprender y modificar.
* Código preparado para implementar **tests unitarios** posteriormente.

## Regla principal

**No comiences modificando código inmediatamente.**

Primero debes analizar el proyecto existente y determinar:

1. Cómo está estructurado actualmente.
2. Qué responsabilidades tiene cada módulo.
3. Qué problemas arquitectónicos existen.
4. Qué principios SOLID se están incumpliendo.
5. Qué partes deberían separarse.
6. Qué lógica pertenece a presentación, dominio, aplicación o infraestructura.
7. Qué componentes deberían dividirse.
8. Qué lógica debería convertirse en casos de uso.
9. Qué dependencias deberían abstraerse.
10. Qué partes serían difíciles de testear actualmente y por qué.

Primero presenta el análisis y una propuesta de refactorización. **No hagas cambios importantes hasta tener definida la estrategia.**

---

# Principios arquitectónicos

Utiliza Clean Architecture como guía conceptual.

La estructura final debería buscar una separación similar a:

```text
Presentation
    ↓
Application
    ↓
Domain

Infrastructure
    ↓
Application / Domain
```

La UI no debería contener directamente lógica de negocio ni conocer detalles innecesarios de infraestructura.

No fuerces una estructura de carpetas artificial si el proyecto no la necesita. La arquitectura debe adaptarse al proyecto y no al revés.

---

# Casos de uso

Identifica las operaciones que representan acciones reales del sistema y conviértelas en **casos de uso atómicos**.

Un caso de uso debe representar una única intención del usuario o una operación concreta del negocio.

Por ejemplo, evitar casos de uso excesivamente grandes como:

```text
GestionarProducto
```

si internamente realiza múltiples operaciones.

Preferir casos como:

```text
CrearProducto
ObtenerProducto
ActualizarProducto
EliminarProducto
BuscarProductos
```

Los casos de uso deben encapsular la lógica de aplicación y permitir que posteriormente puedan probarse mediante tests unitarios sin necesidad de montar toda la interfaz Vue.

La idea es que un caso de uso pueda ser entendido de forma aislada.

---

# Componentes Vue

Revisa los componentes existentes y detecta componentes que estén realizando demasiadas responsabilidades.

Cuando corresponda, dividir componentes grandes en componentes pequeños y reutilizables.

Un componente debería encargarse principalmente de:

* Presentación.
* Interacción con el usuario.
* Emisión/recepción de eventos.
* Composición de otros componentes.

Evita colocar dentro de los componentes Vue:

* Lógica de negocio compleja.
* Acceso directo a APIs.
* Consultas a infraestructura.
* Transformaciones complejas de datos.
* Reglas de negocio.
* Operaciones que deberían pertenecer a casos de uso.

Cuando exista lógica compleja, analiza si debe trasladarse a:

```text
composables
services
use cases
domain
utilities
```

según su responsabilidad real.

No conviertas automáticamente toda lógica en un composable. Determina primero qué responsabilidad tiene.

---

# SOLID

Analiza explícitamente los cinco principios:

### S — Single Responsibility Principle

Detecta clases, funciones, composables y componentes que tengan demasiadas responsabilidades.

### O — Open/Closed Principle

Detecta código que necesite ser modificado constantemente para agregar nuevos comportamientos y analiza si puede diseñarse mediante abstracciones o composición.

### L — Liskov Substitution Principle

Si existen abstracciones, interfaces o implementaciones intercambiables, verifica que puedan sustituirse correctamente.

### I — Interface Segregation Principle

Evita interfaces grandes que obliguen a implementar funcionalidades que un consumidor no necesita.

### D — Dependency Inversion Principle

La lógica de aplicación y dominio no debería depender directamente de detalles concretos de infraestructura.

Por ejemplo, analiza situaciones como:

```text
UseCase → Axios
```

y determina si sería mejor:

```text
UseCase → Repository Interface ← ApiRepository
```

De esta forma, el caso de uso puede probarse utilizando un mock o fake del repositorio.

No introduzcas abstracciones únicamente para "cumplir SOLID". Cada abstracción debe resolver un problema real de acoplamiento, testabilidad o evolución del código.

---

# Testabilidad

Aunque el objetivo principal sea la arquitectura, analiza cada decisión pensando en la futura implementación de tests unitarios.

Para cada parte importante determina:

* ¿Puede probarse aisladamente?
* ¿Depende directamente de Vue?
* ¿Depende directamente de Axios/fetch?
* ¿Depende de localStorage?
* ¿Depende del DOM?
* ¿Depende de otra implementación concreta?
* ¿Necesita mocks innecesarios para poder probarse?

Prioriza que la lógica de negocio y los casos de uso puedan probarse sin Vue ni navegador.

Por ejemplo:

```text
CreateProductUseCase
        ↓
ProductRepository
```

debería poder probarse utilizando:

```text
FakeProductRepository
```

sin realizar una petición HTTP real.

---

# Refactorización incremental

No realices una reescritura completa del frontend.

Trabaja de manera incremental:

1. Analizar.
2. Identificar problemas.
3. Clasificar problemas por importancia.
4. Proponer arquitectura objetivo.
5. Elegir una parte representativa del sistema.
6. Refactorizar esa parte.
7. Verificar que continúe funcionando.
8. Utilizar esa implementación como patrón para el resto del proyecto.
9. Continuar con la siguiente parte.

Evita realizar cambios masivos que dificulten detectar errores.

No modifiques código que funciona correctamente solamente por preferencias de estilo.

Cada refactorización debe tener una justificación arquitectónica concreta.

---

## Mantener simplicidad

No sobrearquitectures el frontend.

No agregues:

* Interfaces innecesarias.
* Clases innecesarias.
* Patrones de diseño sin necesidad.
* Capas que no aporten valor.
* Abstracciones artificiales.
* Factories innecesarias.
* Repositories únicamente porque "Clean Architecture los recomienda".

La arquitectura debe ser proporcional al tamaño y complejidad real del proyecto.

Prioriza:

```text
simple
↓
cohesivo
↓
desacoplado
↓
testeable
↓
extensible
```

sobre una arquitectura excesivamente compleja.

---

## Antes de modificar código

Entrega primero un informe con esta estructura:

## 1. Estado actual

Describe cómo está organizado actualmente el frontend.

## 2. Problemas encontrados

Lista los problemas arquitectónicos encontrados y explica por qué representan un problema.

## 3. Violaciones SOLID

Indica las violaciones encontradas y proporciona ejemplos concretos del código.

## 4. Problemas de Clean Architecture

Identifica dependencias incorrectas entre:

```text
Presentation
Application
Domain
Infrastructure
```

## 5. Problemas de testabilidad

Identifica qué partes serían difíciles de probar y por qué.

## 6. Componentes que deberían dividirse

Identifica componentes Vue demasiado grandes y explica qué responsabilidades deberían separarse.

## 7. Casos de uso

Identifica las operaciones que deberían convertirse en casos de uso atómicos.

## 8. Arquitectura propuesta

Propón una estructura de carpetas y responsabilidades adaptada al proyecto existente.

## 9. Plan de refactorización

Ordena las modificaciones de menor a mayor riesgo.

Para cada etapa indica:

* Qué se modifica.
* Por qué.
* Qué principio SOLID o concepto arquitectónico mejora.
* Qué dependencias se eliminan.
* Cómo mejora la testabilidad.
* Qué riesgo tiene el cambio.

## 10. Ejemplo concreto

Selecciona una parte representativa del proyecto y muestra cómo debería quedar después de la refactorización.

No implementes todavía todo el plan.

---

## Criterio final

El resultado buscado no es simplemente "tener Clean Architecture".

El objetivo es que otro desarrollador pueda leer el código y entender rápidamente:

```text
¿Qué quiere hacer el usuario?
        ↓
¿Qué caso de uso se ejecuta?
        ↓
¿Qué reglas de negocio intervienen?
        ↓
¿Qué datos necesita?
        ↓
¿Qué infraestructura utiliza?
```

Y que cada una de esas responsabilidades pueda evolucionar y probarse de forma independiente.

Prioriza **comprensión del código, separación de responsabilidades, bajo acoplamiento y testabilidad** por encima de aplicar patrones arquitectónicos de manera dogmática.
