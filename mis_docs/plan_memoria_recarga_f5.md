# Plan: reducir memoria en recargas F5 — no traer el dataset completo

## Objetivo

Eliminar la carga masiva de datos que la app hace en cada recarga (F5), reduciendo
el uso de memoria y la lentitud acumulada por recarga. Se logra reemplazando los
7 fetchs completos del Home por un único comando liviano de estadísticas, paginando
el listado de ventas (y eliminando su query N+1), y aplicando retención a los
`audit_logs`.

## Contexto del problema

- F5 recarga el webview. WebView2 **no devuelve la memoria commiteada** del heap JS
  anterior al navegar/recargar (memoria *high-water mark*), así que cada recarga
  apila un heap nuevo sobre el anterior → la RAM sube y la app se vuelve más lenta.
- Esto se **amplifica** porque `HomePage.vue` (ruta por defecto tras login) dispara
  al montar: `fetchArticulos`, `fetchUsers`, `fetchCategorias`, `fetchSubCategorias`,
  `fetchStock`, `fetchProveedores` y `fetchVentas` — todos traen datasets completos.
- `get_all_ventas` además es **N+1**: `SqliteVentaRepository::find_all` ejecuta un
  `load_items` por cada venta (venta_repository.rs:147), con lo que crece el costo a
  medida que se acumulan ventas.
- No hay un *leak* de listeners/conexiones en el código (Rust usa singletons y el
  frontend no registra listeners acumulables). El problema es el volumen re-cargado
  por recarga + la retención de WebView2.

## Decisiones confirmadas

- **Home con un solo comando `get_home_stats`**: devuelve conteos y agregados livianos.
  Sin gateo por permiso en backend (el frontend oculta según permiso, como hoy), para
  no cambiar el UX.
- **Ventas paginadas** (50 por página) con UI Anterior/Siguiente. La búsqueda cliente
  queda acotada a la página cargada; búsqueda global en backend queda como follow-up.
- **Retención de audit_logs**: borrar los registros anteriores a 90 días en cada
  arranque.

---

## Fase 1 — HomePage con un solo comando de estadísticas

### Backend (Rust)

#### 1. Entidad nueva — `src-tauri/src/domain/entities/home.rs`

```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StockBajoItem {
    pub id_stock: i64,
    pub id_articulo: i64,
    pub cod_articulo: String,
    pub articulo: String,
    pub cantidad: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubCategoriaInfo {
    pub id: i64,
    pub sub_categoria: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoriaConSub {
    pub id: i64,
    pub categoria: String,
    pub sub_categorias: Vec<SubCategoriaInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HomeStats {
    pub total_articulos: i64,
    pub articulos_con_stock: i64,
    pub total_usuarios: i64,
    pub usuarios_activos: i64,
    pub usuarios_inactivos: i64,
    pub total_proveedores: i64,
    pub total_categorias: i64,
    pub total_sub_categorias: i64,
    pub ventas_hoy: i64,
    pub total_ventas_hoy: f64,
    pub stock_bajo: Vec<StockBajoItem>,
    pub categorias: Vec<CategoriaConSub>,
}
```

Registrar en `src-tauri/src/domain/entities/mod.rs` (`pub mod home;` + `pub use`).

#### 2. Servicio nuevo — `src-tauri/src/application/services/home_service.rs`

`HomeService::get_stats()` bloquea `DB` (patrón de `cierre_service`) y ejecuta queries
agregadas:

```sql
SELECT COUNT(*) FROM articulos;
SELECT COUNT(DISTINCT id_articulo) FROM stock;
SELECT COUNT(*), COALESCE(SUM(active), 0) FROM users;
SELECT COUNT(*) FROM proveedores;
SELECT COUNT(*) FROM categorias;
SELECT COUNT(*) FROM sub_categorias;
-- ventas de hoy (rango local→UTC, misma lógica que cierre_service)
SELECT COUNT(*), COALESCE(SUM(total), 0) FROM ventas
WHERE anulada = 0 AND fecha >= ?start AND fecha < ?end;
-- stock bajo (< 10)
SELECT s.id, s.id_articulo, a.cod_articulo, a.articulo, s.cantidad
FROM stock s JOIN articulos a ON a.id = s.id_articulo
WHERE s.cantidad < 10 ORDER BY s.cantidad ASC;
-- árbol de categorías (un solo query)
SELECT c.id, c.categoria, sc.id, sc.sub_categoria
FROM categorias c LEFT JOIN sub_categorias sc ON sc.id_categoria = c.id
ORDER BY c.categoria, sc.sub_categoria;
```

Registrar en `src-tauri/src/application/services/mod.rs`.

#### 3. Comando nuevo — `src-tauri/src/api/commands/home_commands.rs`

```rust
pub struct HomeStatsAppState { pub home_service: Mutex<HomeService> }

#[tauri::command(async)]
pub fn get_home_stats(user_id: i64, state: State<HomeStatsAppState>) -> Result<HomeStats, AppError>
```

`user_id` se ignora (no se gatea por permiso), manteniendo el contrato "todo comando
recibe `user_id`". Registro en `api/commands/mod.rs` (`pub mod home_commands;` + `pub use`).

#### 4. Registro en `src-tauri/src/lib.rs`

- `.manage(HomeStatsAppState::new())`
- `get_home_stats` en `tauri::generate_handler!`

### Frontend (Vue/TS)

#### 5. Tipos — `src/domain/entities/types.ts`

```ts
export interface StockBajoItem {
  id_stock: number;
  id_articulo: number;
  cod_articulo: string;
  articulo: string;
  cantidad: number;
}

export interface SubCategoriaInfo {
  id: number;
  sub_categoria: string;
}

export interface CategoriaConSub {
  id: number;
  categoria: string;
  sub_categorias: SubCategoriaInfo[];
}

export interface HomeStats {
  total_articulos: number;
  articulos_con_stock: number;
  total_usuarios: number;
  usuarios_activos: number;
  usuarios_inactivos: number;
  total_proveedores: number;
  total_categorias: number;
  total_sub_categorias: number;
  ventas_hoy: number;
  total_ventas_hoy: number;
  stock_bajo: StockBajoItem[];
  categorias: CategoriaConSub[];
}
```

#### 6. Repo — `src/infrastructure/api/homeRepository.ts`

```ts
export class HomeApiRepository {
  async getHomeStats(): Promise<HomeStats> {
    return await invoke<HomeStats>("get_home_stats", { userId: this.getCurrentUserId() });
  }
}
```

#### 7. Store — `useHomeStore` en `src/presentation/stores/index.ts`

```ts
export const useHomeStore = defineStore("home", () => {
  const stats = ref<HomeStats | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  async function fetchStats() { ... }
  return { stats, loading, error, fetchStats };
});
```

#### 8. Página — `src/presentation/pages/HomePage.vue`

- Reemplazar `onMounted` con los 7 `fetch*` por `homeStore.fetchStats()`.
- Computeds (`totalArticulos`, `articulosConStock`, `totalUsuarios`, `usuariosActivos`,
  `usuariosInactivos`, `totalProveedores`, `totalCategorias`, `totalSubCategorias`,
  `ventasDelDia`/`totalVentasDelDia`, `articulosBajoStock`, `categoriasConSubcategorias`)
  pasan a leer `homeStore.stats`.
- `articulosBajoStock` usa `stats.stock_bajo`; `categoriasConSubcategorias` usa
  `stats.categorias`. Se quitan stores de articulos/users/categorias/subCategorias/
  stock/proveedores/ventas del Home.

---

## Fase 2 — Ventas: paginación + fin del N+1

### Backend (Rust)

#### 9. Trait — `src-tauri/src/domain/repositories/venta_repository.rs`

Agregar:

```rust
fn find_page(&self, limit: i64, offset: i64) -> Result<Page<VentaWithDetalle>, AppError>;
```

#### 10. Implementación — `src-tauri/src/infrastructure/repositories/venta_repository.rs`

`find_page(limit, offset)` con **2 queries** (sin N+1):

1. `SELECT COUNT(*) FROM ventas;`
2. Ventas paginadas: `SELECT ... FROM ventas v LEFT JOIN users ... LEFT JOIN tipos_venta ...
   ORDER BY v.id DESC LIMIT ? OFFSET ?`.
3. Items de las ventas de la página con un único `IN`:
   `SELECT d.*, COALESCE(a.cod_articulo,''), COALESCE(a.articulo,'') FROM venta_detalle d
   LEFT JOIN articulos a ON a.id = d.id_articulo WHERE d.id_venta IN (...) ORDER BY d.id`
   (se construye la lista de ids de la página y se agrupa en un `HashMap<i64, Vec<...>>`).

`find_all` se deja intacto (ya no se usa desde el frontend, pero se conserva).

#### 11. Comando — `src-tauri/src/api/commands/venta_commands.rs`

```rust
#[derive(serde::Deserialize)]
pub struct GetVentasRequest { pub limit: Option<i64>, pub offset: Option<i64> }

#[tauri::command(async)]
pub fn get_all_ventas(user_id, request: Option<GetVentasRequest>, state)
  -> Result<Page<VentaWithDetalle>, AppError>
```

`limit` default 50. Mantiene `check_permission(ViewVentas)`.

### Frontend (Vue/TS)

#### 12. Tipos — `src/domain/entities/types.ts`

```ts
export interface VentaPage {
  items: VentaWithDetalle[];
  total: number;
  limit: number;
  offset: number;
}
```

#### 13. Repo — `src/infrastructure/api/ventaRepository.ts`

```ts
async getAllVentas(filters: { limit: number; offset: number }): Promise<VentaPage> {
  return await invoke<VentaPage>("get_all_ventas", {
    userId: this.getCurrentUserId(),
    request: { limit: filters.limit, offset: filters.offset },
  });
}
```

#### 14. Store — `useVentasStore` en `src/presentation/stores/index.ts`

- Estado nuevo: `total`, `limit` (default 50), `offset`.
- `fetchVentas(filters?)`: guarda `page.items` en `ventas` y `page.total` en `total`.
- `createVenta` / `anularVenta` siguen actualizando la página actual.

#### 15. Página — `src/presentation/pages/VentasPage.vue`

- `onMounted` → `ventasStore.fetchVentas({ limit: 50, offset: 0 })`.
- Buscador local acotado a la página cargada (comportamiento actual, sin cambios de UX
  más allá de la página).
- Controles de paginación Anterior/Siguiente con `currentPage`, `totalPages`,
  guardando en `ventasStore` los `offset`. Búsqueda global en backend: follow-up.

---

## Fase 3 — Retención de audit_logs

### `src-tauri/src/infrastructure/database/mod.rs`

Al final de `init_database`, después de `seed_demo_data`:

```rust
const AUDIT_LOG_RETENTION_DAYS: i64 = 90;
// ...
conn.execute(
    "DELETE FROM audit_logs WHERE created_at < datetime('now', ?1, 'localtime')",
    rusqlite::params![format!("-{} days", AUDIT_LOG_RETENTION_DAYS)],
)?;
```

Idempotente; aprovecha el índice `idx_audit_logs_created_at`.

---

## Verificación

- `cd src-tauri && cargo check` y `cargo clippy`.
- `pnpm build` (`vue-tsc --noEmit` + vite).
- Prueba manual: login → F5 repetido → la memoria de `msedgewebview2.exe` debe quedar
  plana; Ventas con paginación; Home mostrando las mismas estadísticas.

## Notas / follow-ups

- Búsqueda global de ventas en backend (por usuario/artículo) cuando se pague la
  tabla.
- Paginar las demás tablas (artículos, stock, proveedores) cuando crezcan.
- La retención de WebView2 en recargas es de plataforma: solo reiniciar la app la
  libera. Reducir el volumen por recarga (este plan) mitiga la velocidad de crecimiento.
