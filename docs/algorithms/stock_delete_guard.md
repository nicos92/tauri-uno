# Guard de borrado de Stock con ventas asociadas

## Problem

La aplicación permite eliminar un registro de `stock` sin verificar si su artículo
fue vendido. Las ventas (`venta_detalle`) descuentan y restauran el inventario
usando `id_articulo`, y `anular_venta` falla si el registro de stock ya no existe.
Eliminar un stock con ventas asociadas rompe la integridad del inventario vivo
y deja el histórico de ventas huérfano.

Se debe impedir el borrado de un stock cuando su artículo tiene ventas asociadas.

## Input

- `id`: identificador entero (`i64`) del registro de `stock` a eliminar.

## Output

- `Ok(())` si el stock no tiene ventas asociadas y fue eliminado.
- `Err(AppError::StockHasVentas)` si el artículo del stock tiene al menos una
  fila en `venta_detalle`.

## Constraints

- El stock está ligado a un único artículo vía `stock.id_articulo`.
- Las ventas referencian artículos (`venta_detalle.id_articulo`), no `stock.id`.
- El esquema SQLite no impone una FK desde `venta_detalle` a `stock`, por lo que
  la protección debe resolverse en lógica de aplicación.
- Cualquier fila de `venta_detalle` para el artículo, anulada o no, bloquea el
  borrado (decisión: "Cualquier venta").

## Proposed Solution

Añadir un chequeo de existencia previo al `DELETE`: consultar si existe al menos
una fila en `venta_detalle` cuyo `id_articulo` coincida con el del stock que se
quiere borrar. Si existe, rechazar con `AppError::StockHasVentas`; si no, borrar.

El chequeo se implementa como un método `has_ventas` en el repositorio de stock,
siguiendo el patrón existente `TipoVentaRepository::has_ventas`.

### Alternatives Considered

1. **FK de base de datos** (`venta_detalle.id_stock REFERENCES stock(id)`):
   requeriría cambiar el esquema, migrar datos existentes y alterar la semántica
   de las ventas (hoy ligadas al artículo, no a una fila de stock concreta).
2. **Chequeo en el frontend** (ocultar el botón eliminar si hay ventas):
   no es seguro, la validación debe vivir en el backend; además exigiría cargar
   el conteo de ventas en cada fila.
3. **Chequeo por query de existencia en el servicio** (usando el repositorio):
   es la solución elegida; espejo del patrón `TipoVentaInUse`.

| Approach                     | Time | Space | Requerimiento |
|------------------------------|------|-------|---------------|
| FK de base de datos          | O(1) | O(1)  | Cambio de esquema + migración |
| Chequeo en frontend          | O(n) | O(1)  | No seguro (bypass del backend) |
| Query de existencia en repo  | O(1) avg | O(1) | Índice en `venta_detalle.id_articulo` |

### Why This Approach

- Es el patrón establecido en el repositorio (`TipoVentaInUse`), por lo que
  respeta las convenciones existentes.
- Es seguro: la regla se aplica en el backend y cualquier llamada a `delete_stock`
  queda protegida.
- No requiere migrar datos ni cambiar el esquema.

## Algorithm

1. Buscar el stock por `id`. Si no existe, devolver `AppError::StockNotFound`.
2. Obtener el `id_articulo` del stock encontrado.
3. Contar filas en `venta_detalle` donde `id_articulo = stock.id_articulo`.
4. Si el conteo es mayor que 0, devolver `AppError::StockHasVentas`.
5. Ejecutar `DELETE FROM stock WHERE id = ?`.
6. Devolver `Ok(())`.

### Pseudocode

```text
function deleteStock(id):
    stock = findStockById(id)
    if stock is null:
        return error "StockNotFound"

    if hasVentas(stock.id_articulo):
        return error "StockHasVentas"

    execute DELETE FROM stock WHERE id = id
    return ok

function hasVentas(id_articulo):
    count = SELECT COUNT(*) FROM venta_detalle WHERE id_articulo = id_articulo
    return count > 0
```

## Complexity

### Time

O(1) en promedio.

`hasVentas` usa `COUNT(*)` con filtro por `id_articulo`; con el índice
`idx_venta_detalle_id_venta` (sobre `id_venta`) y sin índice sobre
`id_articulo`, el peor caso es un escaneo de `venta_detalle` (O(n)). El `DELETE`
es por PK (O(1)). En la práctica, con pocas filas por artículo, es constante.

### Space

O(1).

Solo se mantienen el `stock` leído y un valor de conteo en memoria.

## Edge Cases

- El stock no existe → `StockNotFound` (comportamiento actual preservado).
- El artículo del stock tiene al menos una venta (anulada o no) → `StockHasVentas`, no se borra.
- El artículo nunca fue vendido → borrado normal.
- `id_articulo` sin filas en `venta_detalle` → conteo 0, borrado permitido.

## Examples

### Example 1

Input:

```text
id = 3 (stock del artículo 7)
venta_detalle: [(id_venta=1, id_articulo=7, ...)]
```

Output:

```text
Err(AppError::StockHasVentas) — "No se puede eliminar el stock porque el artículo tiene ventas asociadas."
```

Explanation:

Existe al menos una fila de `venta_detalle` con `id_articulo = 7`; el borrado se
rechaza y el registro de stock permanece.

### Example 2

Input:

```text
id = 4 (stock del artículo 9)
venta_detalle: sin filas con id_articulo = 9
```

Output:

```text
Ok(()) — el stock es eliminado.
```

Explanation:

El conteo de ventas del artículo es 0; el `DELETE` procede.

## Implementation

Implementado en:

- `src-tauri/src/domain/repositories/stock_repository.rs` — nuevo método `has_ventas` en el trait `StockRepository`.
- `src-tauri/src/infrastructure/repositories/stock_repository.rs` — implementación SQL de `has_ventas` y tests de repositorio.
- `src-tauri/src/application/services/stock_service.rs` — guard en `StockService::delete` y tests de servicio.
- `src-tauri/src/infrastructure/error.rs` — variante `AppError::StockHasVentas`, `code()` y `user_message()`.

Sin cambios en el frontend: el error del backend se serializa como
`{code, message}` y `StockPage.vue` ya lo muestra como toast.

## Validation

- [x] Caso normal (borrado permitido sin ventas)
- [x] Stock inexistente → `StockNotFound`
- [x] Artículo con venta asociada → `StockHasVentas`, stock no se borra
- [x] `has_ventas` retorna `true`/`false` según existan filas en `venta_detalle`
- [x] Tests automatizados Rust (`cargo test`): 55 passed, 0 failed
- [x] `cargo clippy --lib --tests` sin warnings

## Observations

- El esquema no impone FK desde `venta_detalle` a `stock`, por lo que el guard
  se resuelve en lógica de aplicación, siguiendo el patrón
  `TipoVentaService::delete` + `AppError::TipoVentaInUse`.
- El bloqueo aplica a cualquier `venta_detalle`, incluida ventas anuladas:
  es el comportamiento más seguro porque `anular_venta` restaura stock
  exigiendo que la fila exista (`AppError::ArticuloWithoutStock` si no).
