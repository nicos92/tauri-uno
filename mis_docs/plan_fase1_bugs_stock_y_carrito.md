# Plan Fase 1 — Bugs rápidos: stock en negativo y carrito duplicado

## Objetivo

Resolver los dos bugs independientes más simples del documento
`proximos_bugs_a_resolver.md`:

1. **Stock negativo no editable**: no se puede guardar un registro de stock con
   cantidad negativa aunque el backend la acepta.
2. **Artículo duplicado por código**: al registrar un artículo ingresando su
   código, se generan varias filas del mismo producto en el carrito.

Ambos son cambios **solo de frontend**. No se toca Rust en esta fase.

## Bug 1 — Stock negativo no editable

### Causa raíz

La restricción está en el HTML, no en el backend:

- `StockPage.vue:319` — input "Cantidad" del modal **Editar** tiene `min="0"`.
- `StockPage.vue:258` — input "Cantidad" del modal **Crear** tiene `min="0"`.

Con `cantidad = -X`, la validación nativa de HTML5 impide que se dispare el
`submit` → `handleUpdate` (`StockPage.vue:117`) nunca se ejecuta y el navegador
muestra "El valor debe ser mayor o igual a 0".

El backend ya acepta negativos en toda la cadena:

- `domain/entities/stock.rs` — `cantidad: f64` sin validación.
- `application/services/stock_service.rs:19-33` y `:49-66` — sin chequeo.
- `infrastructure/repositories/stock_repository.rs:17-35,85-94` — sin chequeo.
- Schema `database/mod.rs:131-138` — `cantidad REAL NOT NULL`, sin CHECK.

Además, el stock negativo es un estado **esperado**: lo genera el flujo de venta
con permiso `vender_sin_stock` (`venta_repository.rs:52` y `:92`).

### Decisión de diseño

Permitir cantidades negativas **sin restricción** (confirmado con el usuario).
Se elimina el `min="0"` únicamente del campo Cantidad; **se mantiene** `min="0"`
en Costo y Ganancia de ambos modales.

### Cambios

**`src/presentation/pages/StockPage.vue`**

1. Modal de edición (input Cantidad, ~L315-321): quitar el atributo `min="0"`.
2. Modal de creación (input Cantidad, ~L254-260): quitar el atributo `min="0"`.
3. **No tocar** `min="0"` de Costo (L268, L329) ni Ganancia (L278, L339).

No se agrega validación custom: los inputs ya usan `v-model.number` y `step`.

### Verificación

- `pnpm build` (vue-tsc + vite).
- Prueba manual:
  - Registrar una venta con `vender_sin_stock` para dejar stock en negativo.
  - Abrir Editar Stock de ese artículo → el form debe permitir guardar.
  - Verificar que Costo/Ganancia siguen rechazando negativos.

## Bug 2 — Artículo duplicado en el carrito

### Causa raíz

`addArticuloById` (`NuevaVentaPage.vue:110-126`) hace `cart.value.push(...)`
**incondicionalmente**, sin verificar si el `id_articulo` ya está en el carrito.

El flujo que dispara el bug es el de Enter por código exacto:
`onSearchEnter` (`NuevaVentaPage.vue:128-141`) matchea contra la lista completa
`articulosVendibles` (L131-133), que **no** excluye los items ya en carrito (a
diferencia de `searchResults`, L56-68, que sí los excluye vía `inCart`). Cada
Enter del mismo código agrega una fila nueva.

El flujo de clic ya está protegido por `searchResults`, pero conviene que la
lógica de acumulación viva en `addArticuloById` para cubrir ambas rutas.

### Cambios

**`src/presentation/pages/NuevaVentaPage.vue`** — `addArticuloById` (L110-126):

```ts
function addArticuloById(idArticulo: number) {
  const articulo = articulosVendibles.value.find(
    (a) => a.id_articulo === idArticulo,
  );
  if (!articulo) return;

  const existing = cart.value.find((c) => c.id_articulo === idArticulo);
  if (existing) {
    existing.cantidad += 1;
    existing.subtotal = existing.cantidad * existing.precio;
  } else {
    cart.value.push({
      id_articulo: articulo.id_articulo,
      cod_articulo: articulo.cod_articulo,
      articulo: articulo.articulo,
      stockDisponible: articulo.stockDisponible,
      cantidad: 1,
      precio: articulo.precioVenta,
      subtotal: articulo.precioVenta,
    });
  }

  searchQuery.value = "";
  focusSearch();
}
```

Notas:

- El incremento respeta el `precio` ya editado por el usuario (solo suma
  cantidad y recalcula subtotal).
- `subtotal = cantidad * precio` ya coincide con `updateSubtotal` (L151-153).
- El `:key="item.id_articulo"` de la tabla (L343) no cambia; ahora no habrá
  duplicados.
- No se requiere tocar `onSearchEnter` ni `searchResults`.

### Verificación

- `pnpm build`.
- Prueba manual:
  - Escanear el mismo código 3 veces → una sola fila con cantidad 3.
  - Editar el precio de la fila y escanear de nuevo → conserva el precio
    editado y suma cantidad.
  - Agregar por clic desde resultados → no duplica.
  - Buscar por nombre (sin coincidencia exacta) → flujo existente intacto.

## Archivos tocados (resumen)

| Archivo | Cambio |
|---|---|
| `src/presentation/pages/StockPage.vue` | Quitar `min="0"` en Cantidad (2 modales) |
| `src/presentation/pages/NuevaVentaPage.vue` | Acumular cantidad en `addArticuloById` |

## Verificación global de la fase

- `pnpm build` sin errores de tipo (`vue-tsc --noEmit`).
- Pruebas manuales de ambos bugs según lo detallado arriba.
