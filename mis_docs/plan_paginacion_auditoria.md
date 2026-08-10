# Plan: Paginación con total en Auditoría + combo de tamaño de página

## Objetivo

Completar la paginación de la consulta de auditoría agregando el **total de
registros** (COUNT) y una UI con **botones de paginación numerados** y un
**combo box para elegir la cantidad de registros por página** (5, 10, 15, 20, 25).

## Estado actual

- Backend ya recibe `limit`/`offset` en `GetAuditLogsRequest` y aplica
  `LIMIT n OFFSET m`, pero devuelve `Vec<AuditLog>` sin el total.
- Frontend: `AuditoriaPage.vue` usa `PAGE_SIZE = 50` fijo y paginación
  Anterior/Siguiente donde `hasMore` se infiere con `logs.length === PAGE_SIZE`
  (impreciso: falla si la última página trae exactamente el límite).

## Decisiones confirmadas

- **Offset + COUNT total**: cada página = `SELECT COUNT(*)` (con el mismo WHERE) +
  `SELECT ... ORDER BY ... LIMIT/OFFSET`. `hasMore` se calcula con el total real
  del servidor (`offset + limit < total`), no por la longitud de la respuesta.
- **Botones numerados** en la UI: primera/última + ventana alrededor de la página
  actual con `...` en los saltos.
- **Combo de tamaño de página**: opciones 5/10/15/20/25, default 10. Al cambiarlo
  se resetea `offset = 0` y se recarga.
- **Layout responsive**: en pantallas ≤ 960px el texto "Mostrando X de Y" va siempre
  arriba y los botones abajo (columna determinista); en pantallas anchas el texto
  queda fijo a la izquierda y los botones a la derecha (ver sección 10).
- **Estructura `Page<T>` genérica** en `domain` para reutilizarla cuando se
  paginen las demás tablas (ventas, cierres, etc.).

## Backend (Rust)

### 1. Domain — `Page<T>` genérica

- Archivo nuevo `src-tauri/src/domain/repositories/pagination.rs`:

```rust
#[derive(Debug, Clone, serde::Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}
```

- `src-tauri/src/domain/repositories/mod.rs`: `pub mod pagination;` +
  `pub use pagination::Page;`

### 2. Domain — trait `AuditLogRepository`

`src-tauri/src/domain/repositories/audit_log_repository.rs`:

- Cambiar firma a `fn find_with_filters(&self, filter: &AuditLogFilter) ->
  Result<Page<AuditLog>, AppError>;`
- `AuditLogFilter` no cambia (ya tiene `limit`/`offset`).

### 3. Infrastructure — COUNT + datos

`src-tauri/src/infrastructure/repositories/audit_log_repository.rs`
(`find_with_filters`):

- Reutilizar la construcción de `conditions` (ya escapa comillas simples).
- Query de total (sin ORDER BY/LIMIT):

```sql
SELECT COUNT(*) FROM audit_logs WHERE 1 = 1 AND <conditions>
```

- Query de datos existente (ORDER BY + LIMIT/OFFSET) → `items`.
- `limit`/`offset` default: `limit.max(1)` y `offset.max(0)` (ya se usan en el SQL).
- Retornar `Page { items, total, limit, offset }`.

### 4. Application — servicio

`src-tauri/src/application/services/audit_log_service.rs`:

- `pub fn get_logs(&self, filter: &AuditLogFilter) -> Result<Page<AuditLog>, AppError>`
  (passthrough del repositorio).

### 5. API — comando

`src-tauri/src/api/commands/audit_log_commands.rs`:

- `get_audit_logs` retorna `Result<Page<AuditLog>, AppError>`.
- `GetAuditLogsRequest` no cambia.

## Frontend (Vue/TS)

### 6. Tipos

`src/domain/entities/types.ts`:

```ts
export interface AuditLogPage {
  items: AuditLog[];
  total: number;
  limit: number;
  offset: number;
}
```

### 7. Repo — `src/infrastructure/api/auditRepository.ts`

- `getAuditLogs(filters): Promise<AuditLogPage>` (mismo `invoke`).

### 8. Store — `useAuditStore`

`src/presentation/stores/index.ts`:

- Agregar `const total = ref(0);`
- `fetchLogs(filters)`: `const page = await ...; logs.value = page.items;
  total.value = page.total;`
- Exponer `total`.

### 9. Página — `src/presentation/pages/AuditoriaPage.vue`

- Reemplazar `PAGE_SIZE` por `const pageSize = ref(10)`.
- Combo box en la barra de filtros:

```html
<select v-model="pageSize" class="filter-input" @change="handlePageSizeChange">
  <option v-for="n in [5, 10, 15, 20, 25]" :key="n" :value="n">{{ n }} por página</option>
</select>
```

- Computeds:

```ts
const currentPage = computed(() => Math.floor(offset.value / pageSize.value) + 1);
const totalPages = computed(() => Math.max(1, Math.ceil(auditStore.total / pageSize.value)));
const pages = computed<Array<number | "...">>(() => {
  const total = totalPages.value;
  const current = currentPage.value;
  const window = 2;
  const result: Array<number | "..."> = [];
  for (let p = 1; p <= total; p++) {
    if (p === 1 || p === total || (p >= current - window && p <= current + window)) {
      result.push(p);
    } else if (result[result.length - 1] !== "...") {
      result.push("...");
    }
  }
  return result;
});
```

- Acciones:

```ts
function handlePageSizeChange() { offset.value = 0; fetchLogs(); }
function goToPage(page: number) {
  offset.value = (page - 1) * pageSize.value;
  fetchLogs();
}
```

- Guard en `fetchLogs`: si devuelve items vacíos con `offset > 0` y `total > 0`,
  saltar a la última página válida (evita páginas fuera de rango tras filtros).

```ts
async function fetchLogs() {
  await auditStore.fetchLogs(buildFilters());
  if (auditStore.logs.length === 0 && offset.value > 0 && auditStore.total > 0) {
    const lastPage = Math.max(1, Math.ceil(auditStore.total / pageSize.value));
    offset.value = (lastPage - 1) * pageSize.value;
    await auditStore.fetchLogs(buildFilters());
  }
}
```

- `buildFilters()` usa `limit: pageSize.value`.
- Paginación numerada en el template (se muestra si `total > 0`):

```html
<div v-if="!auditStore.loading && auditStore.total > 0" class="pagination">
  <span class="pagination-info">
    Mostrando {{ auditStore.logs.length }} de {{ auditStore.total }} registros
  </span>
  <div class="pagination-buttons">
    <button class="btn-secondary" :disabled="currentPage <= 1" @click="prevPage">‹ Anterior</button>
    <button
      v-for="p in pages"
      :key="p"
      :class="['page-btn', { active: p === currentPage }]"
      :disabled="p === '...'"
      @click="goToPage(p)"
    >{{ p }}</button>
    <button class="btn-secondary" :disabled="currentPage >= totalPages" @click="nextPage">Siguiente ›</button>
  </div>
</div>
```

- `goToPage` acepta `number | "..."` y descarta los saltos:

```ts
function goToPage(page: number | "...") {
  if (typeof page !== "number") return;
  if (page === currentPage.value || page < 1 || page > totalPages.value) return;
  offset.value = (page - 1) * pageSize.value;
  fetchLogs();
}
```

### 10. Responsive — layout estable de la paginación

**Problema**: el ancho de `.pagination-buttons` cambia según la página actual (la
cantidad de botones y dígitos crece al avanzar). Con `justify-content: space-between`
+ `flex-wrap: wrap` sobre el contenedor, a la misma resolución los botones a veces
cabían al lado del texto "Mostrando X de Y" y otras saltaban encima/abajo, saltando el
layout página a página.

**Solución (solo CSS)**:

```css
.pagination {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    margin-top: 1.5rem;
    flex-wrap: wrap;
}

.pagination-info {
    color: var(--color-text-muted);
    font-size: 0.9rem;
    white-space: nowrap;   /* el texto nunca se comprime ni se corta */
    flex-shrink: 0;
}

.pagination-buttons {
    display: flex;
    gap: 0.4rem;
    align-items: center;
    justify-content: flex-end; /* alineado a la derecha en fila */
    flex-wrap: wrap;            /* los botones wrapean dentro del contenedor */
    flex: 1 1 auto;             /* ocupa el espacio restante: el ancho ya no depende de la página */
    min-width: 0;
}

@media (max-width: 960px) {
    .pagination {
        flex-direction: column;  /* texto arriba, botones abajo, siempre */
        align-items: stretch;
        gap: 0.75rem;
    }

    .pagination-info {
        text-align: center;
    }

    .pagination-buttons {
        justify-content: center; /* botones centrados y con wrap interno */
    }
}
```

- **Pantallas anchas (> 960px)**: texto fijo a la izquierda (nowrap) y botones a la
  derecha ocupando el espacio restante; el ancho de la fila deja de depender de la
  página actual.
- **Pantallas chicas (≤ 960px)**: columna determinista — el texto va siempre arriba
  y los botones abajo, centrados y con wrap interno, sin mezclarse con el texto.
- `@media (max-width: 960px)`: breakpoint elegido; no existían media queries previas
  en el proyecto.

## Verificación

- `cd src-tauri && cargo check`
- `pnpm build` (vue-tsc + vite build)

## Notas

- El comando `get_audit_logs` no cambia su contrato de request; cambia solo el tipo
  de respuesta a `Page<AuditLog>`.
- El COUNT reutiliza el mismo WHERE, por lo que el total respeta filtros y rango de
  fechas.
- Los índices existentes (`idx_audit_logs_created_at`, `idx_audit_logs_screen_action`)
  cubren el ORDER BY y los filtros de pantalla/acción.
