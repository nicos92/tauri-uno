# Plan Fase 3 — Tipos de venta: registro con combo + módulo de gestión

## Objetivo

Implementar dos puntos del documento:

1. **Registro de tipo de venta**: combo box en la pantalla de nueva venta para
   marcar si la venta fue por efectivo, transferencia, tarjeta de crédito o
   débito. Por defecto: **Efectivo**.
2. **Módulo de gestión de tipos de venta**: CRUD de tipos (efectivo, tarjeta,
   transferencia, QR, etc.) con campo "hacia dónde" (texto libre, ej. alias /
   CBU para transferencia).

Este es el módulo base que Fase 4 (cierres del día) consumirá para agregar
totales por tipo de venta.

## Decisiones de diseño (confirmadas)

- Seed inicial: **Efectivo, Tarjeta Crédito, Tarjeta Débito, Transferencia, QR**.
- El tipo de venta se guarda en la **cabecera** de la venta (`ventas`), no en el
  detalle.
- "Hacia dónde" es **texto libre opcional** (ej. "Alias: calise.mp",
  "CBU: 000000...", "QR: alias").
- Columnas nuevas en DBs existentes: patrón `ensure_column` (ya usado para
  `ventas.descuento` en `database/mod.rs:182`). La columna FK se agrega
  **nullable** y se hace backfill al id de "Efectivo"; en las lecturas se usa
  `LEFT JOIN` con fallback.
- Permisos nuevos (4): `ver_tipos_venta`, `crear_tipo_venta`,
  `modificar_tipo_venta`, `eliminar_tipo_venta`. Sincronizados en los 3 lugares
  habituales (Rust `PermissionCode`, TS `PERMISSIONS`, seed) + `usePermissions`
  + `AuditoriaPage`.

---

## Sub-fase 3a — Backend (Rust)

### 1. Schema — `src-tauri/src/infrastructure/database/mod.rs`

Agregar al `execute_batch` (después de `venta_detalle`, ~L178):

```sql
CREATE TABLE IF NOT EXISTS tipos_venta (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL UNIQUE,
    hacia_donde TEXT,
    created_at TEXT NOT NULL
);
```

Agregar columna a la tabla `ventas` del `CREATE TABLE` (L154-164):

```sql
    id_tipo_venta INTEGER REFERENCES tipos_venta(id),
```

Después de `ensure_column` de `descuento` (L182), agregar migración para DBs
existentes:

```rust
ensure_column(&conn, "ventas", "id_tipo_venta", "INTEGER REFERENCES tipos_venta(id)")?;
```

> SQLite permite `ADD COLUMN` con FK solo si el default es NULL (nullable).
> Por eso la columna es nullable y se resuelve con backfill + LEFT JOIN.

Backfill (después del `ensure_column`): asignar "Efectivo" a las ventas viejas
y a los tipos nuevos:

```rust
conn.execute(
    "UPDATE ventas SET id_tipo_venta = (SELECT id FROM tipos_venta WHERE nombre = 'Efectivo') WHERE id_tipo_venta IS NULL",
    [],
)?;
```

Seed de tipos (nueva función `seed_tipos_venta`, llamada desde `init_database`):

```rust
const TIPOS_VENTA: &[(&str, Option<&str>)] = &[
    ("Efectivo", None),
    ("Tarjeta Crédito", None),
    ("Tarjeta Débito", None),
    ("Transferencia", None),
    ("QR", None),
];

fn seed_tipos_venta(conn: &Connection) -> Result<(), rusqlite::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    for (nombre, hacia_donde) in TIPOS_VENTA {
        conn.execute(
            "INSERT OR IGNORE INTO tipos_venta (nombre, hacia_donde, created_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![nombre, hacia_donde, now],
        )?;
    }
    Ok(())
}
```

Orden dentro de `init_database`: primero `seed_tipos_venta`, luego el
`ensure_column` + backfill (así el id de "Efectivo" ya existe). Alternativa:
`ensure_column` → `seed_tipos_venta` → backfill. **El backfill siempre después
del seed.**

Agregar los 4 permisos al array `PERMISSIONS` (L22-65):

```rust
// Tipos de Venta
"ver_tipos_venta",
"crear_tipo_venta",
"modificar_tipo_venta",
"eliminar_tipo_venta",
```

> El admin los recibe automáticamente (`seed_admin_user`, L225-263).

### 2. Entidad — `src-tauri/src/domain/entities/tipo_venta.rs` (nuevo)

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipoVenta {
    pub id: i64,
    pub nombre: String,
    pub hacia_donde: Option<String>,
    pub created_at: String,
}
```

Registrar en `src-tauri/src/domain/entities/mod.rs`.

### 3. Modificar `Venta` y `VentaWithDetalle` — `domain/entities/venta.rs`

- `Venta` (L3-13): agregar `pub id_tipo_venta: Option<i64>`.
- `Venta::new` (L15-28): inicializar `id_tipo_venta: None`.
- `VentaWithDetalle` (L72-85): agregar `pub tipo_venta: Option<String>` (nombre,
  para mostrar en frontend y cierres).

### 4. Repositorio

**Trait** — `src-tauri/src/domain/repositories/tipo_venta_repository.rs` (nuevo):

```rust
pub trait TipoVentaRepository: Send + Sync {
    fn find_all(&self) -> Result<Vec<TipoVenta>, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<TipoVenta>, AppError>;
    fn find_by_nombre(&self, nombre: &str) -> Result<Option<TipoVenta>, AppError>;
    fn create(&self, tipo: &TipoVenta) -> Result<TipoVenta, AppError>;
    fn update(&self, tipo: &TipoVenta) -> Result<TipoVenta, AppError>;
    fn delete(&self, id: i64) -> Result<(), AppError>;
}
```

**Implementación** — `src-tauri/src/infrastructure/repositories/tipo_venta_repository.rs`
(nuevo, patrón `SqliteCategoriaRepository`): bloquea `DB`, CRUD directo. El
`delete` falla con `ForeignKeyConstraint` si hay ventas referenciando (se mapea
a `AppError::TipoVentaInUse` en el servicio o con `From<rusqlite::Error>`).

Registrar ambos en sus `mod.rs`.

### 5. Servicio — `src-tauri/src/application/services/tipo_venta_service.rs` (nuevo)

- `create(nombre, hacia_donde)`: validar nombre no vacío; `find_by_nombre` →
  `TipoVentaExists`.
- `update(id, nombre, hacia_donde)`: `find_by_id` → `TipoVentaNotFound`;
  validar unicidad de nombre (excluyendo el propio id).
- `delete(id)`: `find_by_id` → `TipoVentaNotFound`; verificar que no haya
  ventas con ese tipo → `TipoVentaInUse`.
- `get_all()`.
- Registrar en `src-tauri/src/application/services/mod.rs`.

### 6. Errores nuevos — `src-tauri/src/infrastructure/error.rs`

| Variante | `code()` | `user_message()` |
|---|---|---|
| `TipoVentaNotFound` | `tipo_venta_not_found` | "El tipo de venta no existe." |
| `TipoVentaExists` | `tipo_venta_exists` | "Ya existe un tipo de venta con ese nombre." |
| `TipoVentaInUse` | `tipo_venta_in_use` | "No se puede eliminar el tipo de venta porque tiene ventas asociadas." |

Agregar a las tres secciones (enum, `code()`, `user_message()`).

### 7. Permiso — `src-tauri/src/domain/entities/permission_code.rs`

Agregar variantes `ViewTiposVenta`, `CreateTipoVenta`, `UpdateTipoVenta`,
`DeleteTipoVenta` a la enum (L5-39), a `as_str()` (L42-78) y a `all()` (L80-116)
con strings `ver_tipos_venta`, `crear_tipo_venta`, `modificar_tipo_venta`,
`eliminar_tipo_venta`.

### 8. Commands — `src-tauri/src/api/commands/tipo_venta_commands.rs` (nuevo)

- `TipoVentaAppState { tipo_venta_service: Mutex<TipoVentaService> }` con
  `new()`.
- `TipoVentaRequest { nombre, hacia_donde: Option<String> }` (derive Deserialize).
- `get_all_tipos_venta(user_id, state)` → `check_permission(ViewTiposVenta)`.
- `create_tipo_venta(user_id, request, state)` → `check_permission(CreateTipoVenta)` +
  `log_audit(...)`.
- `update_tipo_venta(user_id, id, request, state)` → `check_permission(UpdateTipoVenta)` +
  `log_audit(...)`.
- `delete_tipo_venta(user_id, id, state)` → `check_permission(DeleteTipoVenta)` +
  `log_audit(...)`.
- Usar `AppState` propio con los `AuditScreen`/`AuditAction` correspondientes
  (ver patrón de `venta_commands.rs`). `log_audit` siempre fuera de la
  transacción (deadlock del Mutex no reentrante).

**Audit** — `domain/entities/audit_log.rs`: agregar `AuditScreen::TiposVenta`
(`"Tipos de Venta"`).

Registrar en `src-tauri/src/api/commands/mod.rs`.

### 9. Venta: aceptar `id_tipo_venta`

**`api/commands/venta_commands.rs`**:
- `CreateVentaRequest` (L29-34): agregar `pub id_tipo_venta: Option<i64>`.
- En `create_venta` (L63-103): pasar el valor a `service.create(...)`.

**`application/services/venta_service.rs`**:
- `create` (L19-38): agregar parámetro `id_tipo_venta: Option<i64>` y asignarlo
  a la `Venta` (`venta.id_tipo_venta = id_tipo_venta;` antes de
  `repository.create`).

**`infrastructure/repositories/venta_repository.rs`**:
- INSERT de cabecera (L72-75): agregar `id_tipo_venta` a columnas y params.
- `find_all` (L121-126) y `load_venta` (L205-210): agregar `LEFT JOIN tipos_venta
  t ON t.id = v.id_tipo_venta` y seleccionar `COALESCE(t.nombre, 'Efectivo')`.
- `row_to_venta` (L184-198): mapear el nuevo campo (ajustar índices).

### 10. Registro — `src-tauri/src/lib.rs`

- Imports de `TipoVentaAppState`.
- `.manage(TipoVentaAppState::new())`.
- Comandos en `tauri::generate_handler!`: `get_all_tipos_venta`,
  `create_tipo_venta`, `update_tipo_venta`, `delete_tipo_venta`.

---

## Sub-fase 3b — Frontend: combo en nueva venta

### 1. Tipos — `src/domain/entities/types.ts`

```ts
export interface TipoVenta {
  id: number;
  nombre: string;
  hacia_donde: string | null;
  created_at: string;
}
```

- `Venta` (L169-178): no es necesario exponer `id_tipo_venta` (se envía por
  request).
- `VentaWithDetalle` (L201-213): agregar `tipo_venta: string | null`.
- `CreateVentaRequest` (L221-225): agregar `id_tipo_venta?: number`.

### 2. Permisos — `src/domain/entities/permissions.ts`

Agregar:

```ts
VIEW_TIPOS_VENTA: "ver_tipos_venta",
CREATE_TIPO_VENTA: "crear_tipo_venta",
UPDATE_TIPO_VENTA: "modificar_tipo_venta",
DELETE_TIPO_VENTA: "eliminar_tipo_venta",
```

### 3. Repositorio — `src/infrastructure/api/tipoVentaRepository.ts` (nuevo)

Patrón `categoriaRepository.ts`: `getAllTiposVenta`, `createTipoVenta`,
`updateTipoVenta`, `deleteTipoVenta`; cada `invoke` pasa `userId` y `request`.

### 4. Store — `src/presentation/stores/index.ts`

`useTiposVentaStore` (patrón `useCategoriasStore`): `tipos`, `loading`, `error`,
`fetchTiposVenta`, `createTipoVenta`, `updateTipoVenta`, `deleteTipoVenta`.

### 5. Combo — `src/presentation/pages/NuevaVentaPage.vue`

- Importar `useTiposVentaStore`.
- En `onMounted` (L96-102): agregar `tiposVentaStore.fetchTiposVenta()` al
  `Promise.all`.
- Estado: `const tipoVentaId = ref<number | null>(null);` y un `computed` que
  seleccione por defecto el tipo "Efectivo" cuando carguen los tipos (si aún es
  null).
- En `resetForm()` (L161-166): resetear `tipoVentaId` al id de "Efectivo".
- Template: en la sección de cabecera (junto a Observación, L214-249), agregar
  el combo:

  ```vue
  <div class="form-group">
      <label>Tipo de venta</label>
      <select v-model.number="tipoVentaId" class="cart-input">
          <option
              v-for="tipo in tiposVentaStore.tipos"
              :key="tipo.id"
              :value="tipo.id"
          >
              {{ tipo.nombre }}
          </option>
      </select>
  </div>
  ```

- En `handleCreate` (L168-187): agregar `id_tipo_venta: tipoVentaId.value || undefined`
  al `CreateVentaRequest`.
- La pantalla de venta depende de que exista al menos un tipo (el seed garantiza
  5); si la lista estuviera vacía, `carritoValido` debería exigir tipo no null
  (se muestra el aviso con toast de error del backend si llegara null).

---

## Sub-fase 3c — Frontend: módulo de gestión

### 1. Composable — `src/presentation/composables/usePermissions.ts`

Agregar `canViewTiposVenta`, `canCreateTipoVenta`, `canUpdateTipoVenta`,
`canDeleteTipoVenta` (y devolverlas en el objeto L146-182).

### 2. Página — `src/presentation/pages/TiposVentaPage.vue` (nuevo)

Patrón de `CategoriasPage.vue` / `ProveedoresPage.vue`:

- Tabla: Nombre, "Hacia dónde", acciones (editar/eliminar).
- Modal crear/editar con inputs `nombre` (texto) y `hacia_donde` (texto libre
  opcional).
- Guardar/eliminar con `useTiposVentaStore` y toasts.
- Botones visibles según `canCreateTipoVenta` / `canUpdateTipoVenta` /
  `canDeleteTipoVenta`.

### 3. Ruta — `src/presentation/router/index.ts`

```ts
{
  path: "tipos-venta",
  name: "tipos-venta",
  component: () => import("../pages/TiposVentaPage.vue"),
  meta: { permission: PERMISSIONS.VIEW_TIPOS_VENTA },
},
```

### 4. Menú — `src/presentation/layouts/MainLayout.vue`

Agregar item al `menuItems` (L12-75) después de "Ventas":

```ts
{
  name: "tipos-venta",
  label: "Tipos de Venta",
  icon: "card",
  permission: "ver_tipos_venta",
},
```

Agregar el SVG `src/public/svg/card.svg` (o reutilizar uno existente).

### 5. Auditoría — `src/presentation/pages/AuditoriaPage.vue`

Agregar `"Tipos de Venta"` al array `screens`.

---

## Archivos tocados (resumen)

**Rust nuevos**: `domain/entities/tipo_venta.rs`, `domain/repositories/tipo_venta_repository.rs`,
`infrastructure/repositories/tipo_venta_repository.rs`, `application/services/tipo_venta_service.rs`,
`api/commands/tipo_venta_commands.rs`.

**Rust modificados**: `database/mod.rs`, `entities/mod.rs`, `entities/venta.rs`,
`entities/permission_code.rs`, `entities/audit_log.rs`, `error.rs`,
`repositories/venta_repository.rs`, `services/venta_service.rs`, `services/mod.rs`,
`api/commands/venta_commands.rs`, `api/commands/mod.rs`, `lib.rs`.

**Frontend nuevos**: `infrastructure/api/tipoVentaRepository.ts`,
`presentation/pages/TiposVentaPage.vue`, `public/svg/card.svg`.

**Frontend modificados**: `domain/entities/types.ts`, `domain/entities/permissions.ts`,
`presentation/stores/index.ts`, `presentation/pages/NuevaVentaPage.vue`,
`presentation/composables/usePermissions.ts`, `presentation/router/index.ts`,
`presentation/layouts/MainLayout.vue`, `presentation/pages/AuditoriaPage.vue`,
`presentation/pages/VentasPage.vue` (mostrar `tipo_venta` en listado/detalle).

## Verificación

- `cd src-tauri && cargo check` y `cargo clippy`.
- `pnpm build`.
- Prueba manual:
  - Nueva venta con combo default "Efectivo"; registrar y ver el tipo en
    listado/detalle de `VentasPage`.
  - Crear tipo nuevo ("Mercado Pago", hacia dónde "Alias: x") → aparece en el
    combo.
  - Editar nombre/hacia dónde → se refleja en la venta guardada (el nombre se
    lee con JOIN).
  - Eliminar un tipo sin ventas → OK. Con ventas asociadas → error
    "No se puede eliminar...".
  - Un usuario sin `ver_tipos_venta` no ve el módulo ni la ruta.
