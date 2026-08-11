# Gestión de Clientes (CRUD + regla de validación mínima)

## Problem

La aplicación necesita un módulo de **Gestión de Clientes** para asociar clientes
al registro de ventas. El módulo debe permitir crear, listar, consultar, editar y
eliminar clientes, y debe garantizar dos invariantes:

1. **Regla de contacto mínima**: al guardar (crear o editar) un cliente, al menos
   uno de los campos `nombre`, `apellido`, `telefono`, `email` o `direccion` debe
   estar presente y no vacío. Todos pueden ser `null` individualmente, pero no
   todos a la vez.
2. **Cliente por defecto**: existe un cliente genérico "Consumidor Final"
   (`nombre = "Consumidor"`, `apellido = "Final"`) que se crea automáticamente al
   inicializar la base de datos y que **no puede ser eliminado**.

## Input

- `crear_cliente(request)`: `{ nombre?, apellido?, telefono?, email?, direccion? }`
  (todos opcionales, `Option<String>`).
- `actualizar_cliente(request)`: igual que crear + `id` del cliente existente.
- `obtener_clientes()`: sin parámetros de dominio.
- `obtener_cliente_por_id(id)`: `id: i64`.
- `obtener_cliente_defecto()`: sin parámetros de dominio.
- `eliminar_cliente(id)`: `id: i64`.

Todos los comandos reciben además `user_id: i64` como primer argumento (patrón
del proyecto) para verificación de permisos y auditoría.

## Output

- `crear_cliente` / `actualizar_cliente` → `Cliente` persistido (con `id`,
  `created_at`, `updated_at`).
- `obtener_clientes` → `Vec<Cliente>` ordenado por nombre/apellido.
- `obtener_cliente_por_id` → `Cliente`.
- `obtener_cliente_defecto` → `Cliente` con `nombre = "Consumidor"` y
  `apellido = "Final"`.
- `eliminar_cliente` → `()`.
- Errores: `AppError::ClienteSinDatosDeContacto` si falla la regla mínima;
  `AppError::ClienteNotFound` si el id no existe;
  `AppError::NoSePuedeEliminarClienteDefecto` si se intenta borrar "Consumidor Final".

## Constraints

- La entidad no tiene campos con `UNIQUE`: no hay clave natural, por lo que no
  aplica validación de duplicados.
- El cliente por defecto se identifica por `nombre = 'Consumidor' AND
  apellido = 'Final'` (no existe columna de flag).
- Las cadenas de solo espacios deben tratarse como vacías (normalización
  `trim`).
- La regla mínima debe vivir en el backend (lógica de aplicación) y replicarse
  como validación reactiva en el frontend solo para UX; el backend es la
  fuente de verdad.
- `user_id` es obligatorio en cada comando; cada operación exige su permiso
  (`ver_clientes`, `crear_cliente`, `modificar_cliente`, `eliminar_cliente`).

## Proposed Solution

Implementar un CRUD completo siguiendo el patrón existente del módulo
`Proveedor` (entidad → trait de repositorio → servicio → comando Tauri →
store Pinia → página Vue). El núcleo algorítmico es la regla de validación
mínima:

1. **Normalización**: cada `Option<String>` se recorta (`trim`) y si queda vacío
   se convierte a `None`.
2. **Predicado "tiene dato"**: el cliente es válido si y solo si al menos uno de
   los cinco campos normalizados es `Some`.
3. El predicado se evalúa en `ClienteService::create` y `ClienteService::update`
   antes de tocar el repositorio.

El cliente por defecto se crea en el arranque vía un *seeder* dentro de
`apply_schema` (`seed_cliente_defecto`), usando `INSERT ... WHERE NOT EXISTS`
(no hay `UNIQUE` sobre el que apoyar `INSERT OR IGNORE`). La protección contra
su borrado vive en `ClienteService::delete`: se consulta `find_default()` y si el
id coincide se rechaza con `NoSePuedeEliminarClienteDefecto`.

### Alternatives Considered

1. **Columna flag `es_defecto` en la tabla**: permitiría marcar el cliente por
   defecto de forma explícita y hacer `ON DELETE` imposible por constraint. Se
   descartó porque el requerimiento especifica identificar al cliente por sus
   datos ("Consumidor"/"Final") y no pide flag; además agrega un campo que puede
   desincronizarse.
2. **Validación solo en el frontend** (deshabilitar botón): no es seguro; el
   backend debe validar porque `invoke` puede llamarse fuera de la UI.
3. **`NOT NULL` combinado a nivel de DB** (check constraint de que al menos un
   campo sea no nulo): no cubre el caso "string vacío" y complica el esquema.
   La validación en servicio cubre ambos casos (null y vacío) y es el patrón del
   proyecto (validaciones de negocio en servicios).

| Approach | Time | Space | Requerimiento |
|----------|------|-------|---------------|
| Guard en servicio (normalización + predicado) | O(5) = O(1) | O(1) | Ninguno adicional |
| Check constraint en DB | O(1) | O(1) | Solo cubre NULL, no strings vacíos |
| Validación solo frontend | O(1) | O(1) | No seguro (bypass del backend) |

### Why This Approach

- Respeta el patrón de clean architecture del proyecto: la regla de negocio vive
  en `application/services`, no en la capa de datos ni en la UI.
- La normalización `trim` + predicado resuelve de forma uniforme los casos
  `None`, `""` y `"   "` sin agregar complejidad.
- El seeder idempotente (`WHERE NOT EXISTS`) garantiza que "Consumidor Final"
  exista una sola vez desde el primer arranque, sin migraciones.

## Algorithm

```
1. Normalizar cada campo:
      campo_norm = trim(campo) si Some, else None
      si campo_norm es string vacío -> None

2. Validar al menos un dato:
      valido = nombre.is_some() OR apellido.is_some() OR
               telefono.is_some() OR email.is_some() OR direccion.is_some()
      si !valido -> error ClienteSinDatosDeContacto

3. Persistir (create): INSERT con created_at = updated_at = ahora UTC (RFC3339).

   Persistir (update): verificar existencia (ClienteNotFound),
       re-normalizar y re-validar, UPDATE con updated_at = ahora UTC.

4. Eliminar:
      si find_default().id == id -> error NoSePuedeEliminarClienteDefecto
      si no existe -> ClienteNotFound
      DELETE por id.
```

### Pseudocode

```text
function normalize(field):
    if field is null:
        return null
    trimmed = field.trim()
    if trimmed == "":
        return null
    return trimmed

function hasAnyContactData(nombre, apellido, telefono, email, direccion):
    return nombre != null OR apellido != null OR telefono != null
           OR email != null OR direccion != null

function crearCliente(nombre, apellido, telefono, email, direccion):
    nombre    = normalize(nombre)
    apellido  = normalize(apellido)
    telefono  = normalize(telefono)
    email     = normalize(email)
    direccion = normalize(direccion)

    if not hasAnyContactData(nombre, apellido, telefono, email, direccion):
        return error "ClienteSinDatosDeContacto"

    return repository.insert(cliente)

function actualizarCliente(id, nombre, apellido, telefono, email, direccion):
    if repository.findById(id) is null:
        return error "ClienteNotFound"

    [misma normalización y validación que en crear]

    return repository.update(cliente)

function eliminarCliente(id):
    if repository.findDefault().id == id:
        return error "NoSePuedeEliminarClienteDefecto"
    if repository.findById(id) is null:
        return error "ClienteNotFound"
    return repository.delete(id)

function seedClienteDefecto():
    INSERT INTO clientes (nombre, apellido, created_at, updated_at)
    SELECT 'Consumidor', 'Final', now, now
    WHERE NOT EXISTS (SELECT 1 FROM clientes
                      WHERE nombre = 'Consumidor' AND apellido = 'Final')
```

## Complexity

### Time

O(1).

La validación evalúa un número fijo de 5 campos (operación constante). Las
operaciones de repositorio usan la PK (`id`) o un filtro puntual
(`nombre/apellido` del seeder), ambas constantes con el índice de PK. En la
práctica el conteo de clientes es pequeño y no hay escaneos significativos.

### Space

O(1).

Solo se mantienen en memoria los 5 `Option<String>` normalizados y el registro
`Cliente` en operación. La lista `obtener_clientes` es O(n) inherente a su
propia respuesta.

## Edge Cases

- Todos los campos `None` → `ClienteSinDatosDeContacto`.
- Todos los campos `""` o solo espacios → normalización a `None` →
  `ClienteSinDatosDeContacto`.
- Un solo campo con contenido (cualquiera de los 5) → cliente válido.
- Editar un cliente inexistente → `ClienteNotFound`.
- Eliminar "Consumidor Final" → `NoSePuedeEliminarClienteDefecto`.
- Eliminar un cliente inexistente → `ClienteNotFound`.
- Cliente con un solo dato de identificación (ej. solo email) → válido.
- Seeder ejecutado varias veces → no duplica "Consumidor Final" (idempotente).

## Examples

### Example 1

Input:

```text
crear_cliente { nombre: null, apellido: null, telefono: null, email: null, direccion: null }
```

Output:

```text
Err(AppError::ClienteSinDatosDeContacto)
```

Explanation:

Ninguno de los cinco campos tiene contenido; la regla de contacto mínima falla y
el registro no se inserta.

### Example 2

Input:

```text
crear_cliente { nombre: "   ", apellido: "Pérez", telefono: "", email: null, direccion: null }
```

Output:

```text
Ok(Cliente { nombre: None, apellido: Some("Pérez"), telefono: None, ... })
```

Explanation:

`"   "` y `""` se normalizan a `None`, pero `apellido` conserva "Pérez"; el
predicado se satisface y el cliente se persiste.

### Example 3

Input:

```text
eliminar_cliente(id del cliente "Consumidor Final")
```

Output:

```text
Err(AppError::NoSePuedeEliminarClienteDefecto)
```

Explanation:

`find_default()` devuelve el cliente "Consumidor Final" cuyo `id` coincide con el
recibido; el borrado se rechaza.

## Implementation

Implementado en:

- `src-tauri/src/domain/entities/cliente.rs` — entidad `Cliente`.
- `src-tauri/src/domain/repositories/cliente_repository.rs` — trait `ClienteRepository`.
- `src-tauri/src/application/services/cliente_service.rs` — validación mínima y guards.
- `src-tauri/src/infrastructure/repositories/cliente_repository.rs` — implementación SQLite.
- `src-tauri/src/api/commands/cliente_commands.rs` — comandos Tauri.
- `src-tauri/src/infrastructure/database/mod.rs` — tabla `clientes` + seeder.
- `src-tauri/src/infrastructure/error.rs` — variantes `AppError`.
- `src-tauri/src/domain/entities/permission_code.rs` — permisos de clientes.
- `src-tauri/src/domain/entities/audit_log.rs` — `AuditScreen::Clientes`.
- `src-tauri/src/lib.rs` — registro de estado y comandos.
- Frontend: `src/domain/entities/types.ts`, `permissions.ts`,
  `src/infrastructure/api/clienteRepository.ts`,
  `src/presentation/stores/index.ts`, `usePermissions.ts`,
  `src/presentation/pages/ClientesPage.vue`, `router/index.ts`,
  `layouts/MainLayout.vue`.

## Validation

- [x] Normal case (crear/editar con un solo dato)
- [x] Todos los campos vacíos → `ClienteSinDatosDeContacto`
- [x] Campos de solo espacios → `ClienteSinDatosDeContacto`
- [x] Eliminar "Consumidor Final" → `NoSePuedeEliminarClienteDefecto`
- [x] Cliente inexistente → `ClienteNotFound`
- [x] Seeder idempotente ("Consumidor Final" aparece una sola vez)
- [x] Tests automatizados Rust (`cargo test`): 76 passed, 0 failed
- [x] `cargo clippy --lib --tests` sin warnings
- [x] `pnpm build` (vue-tsc + vite build) sin errores

## Observations

- La asociación de clientes con el registro de ventas (FK `ventas.id_cliente`)
  queda pendiente: este módulo crea la base de datos y la API de clientes, pero
  el requerimiento de las ventas no forma parte de esta entrega.
- El cliente por defecto se crea al inicializar la base (`Lazy<DB>` →
  `apply_schema` → `seed_cliente_defecto`), por lo que al primer arranque ya
  existe. No requiere comando manual.
- La regla mínima se valida en el backend y, como UX, el formulario Vue
  deshabilita el botón de guardado cuando todos los campos están vacíos.
