# Plan: Actualización Masiva de Precios de Costo (por Porcentaje)

## Resumen del Requerimiento

Dentro del módulo de **Stock**, incorporar una funcionalidad para ajustar el **precio de costo** de los artículos que poseen stock, aplicando un **porcentaje** de aumento o descuento sobre el costo existente, filtrado por categoría, subcategoría y/o proveedor.

**Ejemplo:** Artículos de categoría "Cables" con costo \$1000, aplicar +20% → costo nuevo \$1200.

**Decisiones del usuario:**
1. Reutilizar el permiso existente `UpdateStock` ("modificar_stock")
2. La operación es un **ajuste por porcentaje** (+ o -), no asignación de costos individuales
3. Sin límite de cantidad. Se actualiza todo lo que devuelvan los filtros
4. No requiere migración de DB

---

## Arquitectura del Flujo

```
StockPage (botón "Actualizar Precios de Costo")
  → Router → ActualizarCostoPage
    → Filtros (categoría, subcategoría, proveedor) + Input porcentaje
    → "Vista previa" → invoke("get_stock_preview_costo") → tabla preview
    → "Aplicar Cambios" → useConfirm() → invoke("apply_costo_percentage_stock")
      → check_permission(UpdateStock)
      → StockService::apply_costo_percentage()
        → StockRepository::apply_costo_percentage()
          → UPDATE stock SET costo = ROUND(costo * (1 + ? / 100), 2) WHERE id IN (subquery filtrada)
    → Toast éxito → redirect StockPage
```

---

## Modelo de Datos (sin cambios)

No se requiere migración. La operación es un UPDATE sobre la tabla `stock` existente.

**Relaciones clave:**
```
stock (id, id_articulo FK, cantidad, costo, ganancia)
  → articulos (id, articulo, cod_articulo, id_sub_categoria FK, id_proveedor FK)
    → sub_categorias (id, sub_categoria, id_categoria FK)
      → categorias (id, categoria)
    → proveedores (id, cuit, proveedor, nombre, ...)
```

---

## Backend Rust

### StockRepository Trait

**Archivo:** `src-tauri/src/domain/repositories/stock_repository.rs`

Agregar 2 métodos:

```rust
fn find_filtered_with_preview(
    &self,
    porcentaje: f64,
    id_categoria: Option<i64>,
    id_sub_categoria: Option<i64>,
    id_proveedor: Option<i64>,
) -> Result<Vec<StockPreview>, AppError>;

fn apply_costo_percentage(
    &self,
    porcentaje: f64,
    id_categoria: Option<i64>,
    id_sub_categoria: Option<i64>,
    id_proveedor: Option<i64>,
) -> Result<i64, AppError>;
```

### StockPreview (nueva entidad)

**Archivo:** `src-tauri/src/domain/entities/stock_preview.rs` (nuevo)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPreview {
    pub id_stock: i64,
    pub id_articulo: i64,
    pub cod_articulo: String,
    pub articulo: String,
    pub categoria: String,
    pub sub_categoria: String,
    pub proveedor: String,
    pub costo_actual: f64,
    pub ganancia: f64,
    pub costo_nuevo: f64,
    pub cantidad: f64,
}
```

Registrar en `domain/entities/mod.rs`.

### SqliteStockRepository

**Archivo:** `src-tauri/src/infrastructure/repositories/stock_repository.rs`

**`find_filtered_with_preview` — Query SELECT (solo lectura):**

```sql
SELECT s.id, s.id_articulo, a.cod_articulo, a.articulo,
       c.categoria, sc.sub_categoria, p.proveedor,
       s.costo, s.ganancia,
       ROUND(s.costo * (1 + ?1 / 100), 2) AS costo_nuevo,
       s.cantidad
FROM stock s
INNER JOIN articulos a ON s.id_articulo = a.id
INNER JOIN sub_categorias sc ON a.id_sub_categoria = sc.id
INNER JOIN categorias c ON sc.id_categoria = c.id
INNER JOIN proveedores p ON a.id_proveedor = p.id
WHERE (?2 IS NULL OR c.id = ?2)
  AND (?3 IS NULL OR sc.id = ?3)
  AND (?4 IS NULL OR p.id = ?4)
ORDER BY a.articulo
```

**`apply_costo_percentage` — Query UPDATE (atómica):**

```sql
UPDATE stock
SET costo = ROUND(costo * (1 + ?1 / 100), 2)
WHERE id IN (
    SELECT s.id
    FROM stock s
    INNER JOIN articulos a ON s.id_articulo = a.id
    INNER JOIN sub_categorias sc ON a.id_sub_categoria = sc.id
    INNER JOIN categorias c ON sc.id_categoria = c.id
    INNER JOIN proveedores p ON a.id_proveedor = p.id
    WHERE (?2 IS NULL OR c.id = ?2)
      AND (?3 IS NULL OR sc.id = ?3)
      AND (?4 IS NULL OR p.id = ?4)
)
```

**Por qué un único UPDATE:**
- Atómico (SQLite lo ejecuta como una sola operación)
- Sin riesgo de actualizaciones parciales
- Performático (una sola ronda a DB)
- `ROUND(costo, 2)` evita problemas de precisión flotante

### StockService

**Archivo:** `src-tauri/src/application/services/stock_service.rs`

Agregar 2 métodos:

```rust
pub fn get_preview(
    &self,
    porcentaje: f64,
    id_categoria: Option<i64>,
    id_sub_categoria: Option<i64>,
    id_proveedor: Option<i64>,
) -> Result<Vec<StockPreview>, AppError> {
    self.repository.find_filtered_with_preview(
        porcentaje, id_categoria, id_sub_categoria, id_proveedor,
    )
}

pub fn apply_costo_percentage(
    &self,
    porcentaje: f64,
    id_categoria: Option<i64>,
    id_sub_categoria: Option<i64>,
    id_proveedor: Option<i64>,
) -> Result<i64, AppError> {
    if !porcentaje.is_finite() || porcentaje == 0.0 || porcentaje < -100.0 {
        return Err(AppError::BulkUpdateInvalidPorcentaje);
    }
    self.repository.apply_costo_percentage(
        porcentaje, id_categoria, id_sub_categoria, id_proveedor,
    )
}
```

**Validaciones backend:**
- `porcentaje` no puede ser NaN o infinito
- `porcentaje` no puede ser exactamente 0 (sin efecto)
- `porcentaje` no puede ser < -100% (costo no puede ser negativo)

### Stock Commands

**Archivo:** `src-tauri/src/api/commands/stock_commands.rs`

Agregar 2 commands + 2 DTOs:

```rust
#[derive(serde::Deserialize)]
pub struct ApplyCostoPercentageRequest {
    pub porcentaje: f64,
    pub id_categoria: Option<i64>,
    pub id_sub_categoria: Option<i64>,
    pub id_proveedor: Option<i64>,
}

#[derive(serde::Serialize)]
pub struct ApplyCostoPercentageResult {
    pub updated_count: i64,
}
```

**Command 1: `get_stock_preview_costo`** (solo lectura, permiso `ViewStock`)
**Command 2: `apply_costo_percentage_stock`** (escritura, permiso `UpdateStock`, con audit log)

### AppError

**Archivo:** `src-tauri/src/infrastructure/error.rs`

Agregar 2 variantes:

```rust
#[error("Porcentaje inválido para actualización masiva")]
BulkUpdateInvalidPorcentaje,

#[error("No se encontraron artículos con los filtros seleccionados")]
BulkUpdateNoMatches,
```

Con sus `code()` y `user_message()` correspondientes.

### Registros en lib.rs

**Archivo:** `src-tauri/src/lib.rs`

- Agregar `get_stock_preview_costo` y `apply_costo_percentage_stock` a `generate_handler!`
- Importar los nuevos commands

**Archivo:** `src-tauri/src/api/commands/mod.rs`

- Exportar los nuevos commands y structs

---

## Frontend Vue.js

### Nueva ruta

**Archivo:** `src/presentation/router/index.ts`

```typescript
{
  path: "stock/actualizar-costo",
  name: "actualizar-costo",
  meta: { permission: PERMISSIONS.UPDATE_STOCK },
  component: () => import("../pages/ActualizarCostoPage.vue"),
}
```

### Botón en StockPage

**Archivo:** `src/presentation/pages/StockPage.vue`

Dentro de `<PageHeader>`, junto al botón "Crear Stock":

```vue
<button
    v-if="canUpdateStock()"
    @click="router.push({ name: 'actualizar-costo' })"
    class="btn-secondary"
>
    Actualizar Precios de Costo
</button>
```

Importar `useRouter` de vue-router.

### Nueva página: ActualizarCostoPage.vue

**Archivo:** `src/presentation/pages/ActualizarCostoPage.vue` (nuevo)

**Estructura visual:**

```
┌─────────────────────────────────────────────────┐
│ Actualizar Precios de Costo                     │
├─────────────────────────────────────────────────┤
│                                                 │
│  Categoría: [Todas      ▼]                     │
│  Subcategoría: [Todas    ▼]  (filtrado por cat)│
│  Proveedor: [Todos      ▼]                     │
│                                                 │
│  Porcentaje de ajuste: [____] %               │
│                                                 │
│  [ Vista Previa ]                               │
│                                                 │
│  ┌───────────────────────────────────────────┐  │
│  │ Código │ Art. │ Cat. │ Costo │ Nuevo │ % │  │
│  │--------│------│------│-------│-------│---│  │
│  │ C-001  │ Cab2 │ Cable│ $1000 │ $1200 │+20│  │
│  │ C-002  │ Cab4 │ Cable│ $1500 │ $1800 │+20│  │
│  └───────────────────────────────────────────┘  │
│  15 artículos serán actualizados                 │
│                                                 │
│  [ Aplicar Cambios ]                            │
│                                                 │
└─────────────────────────────────────────────────┘
```

**Estado del componente:**

```typescript
const idCategoria = ref<number | null>(null);
const idSubCategoria = ref<number | null>(null);
const idProveedor = ref<number | null>(null);
const porcentaje = ref<number>(0);
const preview = ref<StockPreview[]>([]);
const loadingPreview = ref(false);
const applying = ref(false);
```

**Computed:**

- `subCategoriasFiltradas`: Filtra subcategorías según categoría seleccionada
- `previewValido`: `porcentaje !== 0 && preview.length > 0`
- `costoTotalActual`: Suma de costos actuales del preview
- `costoTotalNuevo`: Suma de costos nuevos del preview

**Flujo:**

1. `onMounted`: Cargar categorías, subcategorías, proveedores de stores existentes
2. Seleccionar filtros + porcentaje
3. Clic "Vista Previa" → `invoke("get_stock_preview_costo")` → mostrar tabla
4. Clic "Aplicar Cambios" → `useConfirm()` → `invoke("apply_costo_percentage_stock")` → toast → redirect

### Tipos TypeScript

**Archivo:** `src/domain/entities/types.ts`

```typescript
export interface StockPreview {
  id_stock: number;
  id_articulo: number;
  cod_articulo: string;
  articulo: string;
  categoria: string;
  sub_categoria: string;
  proveedor: string;
  costo_actual: number;
  ganancia: number;
  costo_nuevo: number;
  cantidad: number;
}

export interface ApplyCostoPercentageRequest {
  porcentaje: number;
  id_categoria: number | null;
  id_sub_categoria: number | null;
  id_proveedor: number | null;
}

export interface ApplyCostoPercentageResult {
  updated_count: number;
}
```

### Repository Interface

**Archivo:** `src/domain/interfaces/stockRepository.ts`

```typescript
getStockPreviewCosto(
  porcentaje: number,
  idCategoria: number | null,
  idSubCategoria: number | null,
  idProveedor: number | null,
): Promise<StockPreview[]>;

applyCostoPercentage(
  request: ApplyCostoPercentageRequest,
): Promise<ApplyCostoPercentageResult>;
```

### ApiRepository

**Archivo:** `src/infrastructure/api/stockRepository.ts`

```typescript
async getStockPreviewCosto(
  porcentaje: number,
  idCategoria: number | null,
  idSubCategoria: number | null,
  idProveedor: number | null,
): Promise<StockPreview[]> {
  return await invoke<StockPreview[]>("get_stock_preview_costo", {
    userId: getCurrentUserId(),
    porcentaje,
    idCategoria,
    idSubCategoria,
    idProveedor,
  });
}

async applyCostoPercentage(
  request: ApplyCostoPercentageRequest,
): Promise<ApplyCostoPercentageResult> {
  return await invoke<ApplyCostoPercentageResult>(
    "apply_costo_percentage_stock",
    { userId: getCurrentUserId(), request },
  );
}
```

### UseCase

**Archivo:** `src/application/usecases/stock.ts`

Delegación directa al repository (patrón existente).

### Store

**Archivo:** `src/presentation/stores/stockStore.ts`

Agregar 2 métodos:

```typescript
async function getStockPreviewCosto(
  porcentaje: number,
  idCategoria: number | null,
  idSubCategoria: number | null,
  idProveedor: number | null,
): Promise<StockPreview[]> { ... }

async function applyCostoPercentage(
  request: ApplyCostoPercentageRequest,
): Promise<ApplyCostoPercentageResult | null> { ... }
```

---

## Validaciones

### Frontend

| Campo | Regla | Mensaje |
|-------|-------|---------|
| Porcentaje | Requerido, numérico, no 0 | "Ingrese un porcentaje válido" |
| Porcentaje | >= -100 | "El porcentaje no puede ser menor a -100%" |
| Vista previa | Debe haber resultado | "No se encontraron artículos con los filtros seleccionados" |
| Aplicar | Debe haber preview cargado | Botón deshabilitado |

### Backend

| Regla | Error |
|-------|-------|
| `porcentaje` es NaN o infinito | `BulkUpdateInvalidPorcentaje` |
| `porcentaje` == 0 | `BulkUpdateInvalidPorcentaje` |
| `porcentaje` < -100 | `BulkUpdateInvalidPorcentaje` |
| Sin artículos que coincidan | UPDATE retorna 0 affected rows (no es error, se informa al frontend) |

---

## Seguridad e Integridad

| Riesgo | Mitigación |
|--------|-----------|
| Filtros vacíos actualizan todo | Vista previa obligatoria antes de aplicar. Confirmación con `useConfirm()`. |
| Porcentaje inválido (< -100%) | Validación backend. Costo no puede ser negativo. |
| Costo resultante negativo | `ROUND(costo * (1 + p/100), 2)` con p >= -100 garantiza costo >= 0 |
| Actualización accidental | Diálogo de confirmación: "¿Está seguro de actualizar el precio de costo de X artículos en un Y%?" |
| Concurrencia | Mutex global DB serializa accesos |
| Permiso | Se reutiliza `UpdateStock` ("modificar_stock") |
| Audit log | Se registra la operación con porcentaje y cantidad de artículos afectados |

---

## Testing

### Backend (Rust)

| Test | Tipo | Qué verifica |
|------|------|-------------|
| `apply_costo_percentage_increases` | Integration | +20% sobre categoría específica, verificar costos |
| `apply_costo_percentage_decreases` | Integration | -10%, verificar reducción |
| `apply_costo_percentage_no_filter` | Integration | Sin filtros, todos los stocks se actualizan |
| `apply_costo_percentage_rejects_zero` | Unit | Retorna `BulkUpdateInvalidPorcentaje` |
| `apply_costo_percentage_rejects_below_minus_100` | Unit | -101% retorna error |
| `apply_costo_percentage_rejects_nan` | Unit | NaN retorna error |
| `apply_costo_percentage_rounds_to_2_decimals` | Integration | Verificar redondeo correcto |
| `find_filtered_with_preview_matches` | Integration | Filtros retornan los artículos correctos |
| `find_filtered_with_preview_no_filter` | Integration | Sin filtros retorna todo |

---

## Archivos a Modificar (15)

| # | Archivo | Cambio |
|---|---------|--------|
| 1 | `src-tauri/src/domain/entities/stock_preview.rs` | **CREAR** — nueva entidad `StockPreview` |
| 2 | `src-tauri/src/domain/entities/mod.rs` | Agregar `pub mod stock_preview` + re-export |
| 3 | `src-tauri/src/domain/repositories/stock_repository.rs` | +2 métodos al trait |
| 4 | `src-tauri/src/infrastructure/repositories/stock_repository.rs` | +2 implementaciones SQL |
| 5 | `src-tauri/src/application/services/stock_service.rs` | +2 métodos con validación |
| 6 | `src-tauri/src/api/commands/stock_commands.rs` | +2 commands + DTOs |
| 7 | `src-tauri/src/api/commands/mod.rs` | Exportar nuevos symbols |
| 8 | `src-tauri/src/lib.rs` | Registrar commands en `generate_handler!` |
| 9 | `src-tauri/src/infrastructure/error.rs` | +2 variantes AppError |
| 10 | `src/domain/entities/types.ts` | +3 interfaces TS |
| 11 | `src/domain/interfaces/stockRepository.ts` | +2 métodos |
| 12 | `src/infrastructure/api/stockRepository.ts` | +2 métodos invoke |
| 13 | `src/application/usecases/stock.ts` | +2 métodos delegación |
| 14 | `src/presentation/stores/stockStore.ts` | +2 métodos store |
| 15 | `src/presentation/pages/StockPage.vue` | +1 botón en PageHeader |
| 16 | `src/presentation/router/index.ts` | +1 ruta |

## Archivos a Crear (1)

| # | Archivo | Propósito |
|---|---------|-----------|
| 1 | `src/presentation/pages/ActualizarCostoPage.vue` | Pantalla de ajuste por porcentaje con preview |

---

## Orden de Implementación Sugerido

1. **Backend - Entidad:** Crear `StockPreview` + registrar en mod.rs
2. **Backend - Error:** Agregar variantes a `AppError`
3. **Backend - Repository trait:** Agregar 2 métodos al trait
4. **Backend - Repository impl:** Implementar queries SQL
5. **Backend - Service:** Agregar métodos con validación
6. **Backend - Commands:** Crear DTOs y commands
7. **Backend - Registro:** Exportar en mod.rs y registrar en lib.rs
8. **Backend - Tests:** Escribir tests de integración
9. **Frontend - Types:** Agregar interfaces TS
10. **Frontend - Repository:** Agregar métodos a interfaz e implementación
11. **Frontend - UseCase + Store:** Delegación
12. **Frontend - Router:** Agregar ruta
13. **Frontend - StockPage:** Agregar botón
14. **Frontend - ActualizarCostoPage:** Crear la página completa
15. **Verificación:** `cargo clippy --lib --tests` + `pnpm build`
