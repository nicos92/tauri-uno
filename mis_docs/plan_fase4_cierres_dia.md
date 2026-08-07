# Plan Fase 4 — Cierres del día

## Objetivo

Implementar el módulo de **cierres del día**: registrar en la base de datos el
día, mes y año con los totales de costo, ganancia y el total **por tipo de
venta**.

**Dependencia**: requiere la Fase 3 (columna `ventas.id_tipo_venta` y tabla
`tipos_venta`). Sin Fase 3, no hay "total por tipo de venta".

## Decisiones de diseño (confirmadas)

- **Generación manual**: botón "Cerrar día" que elige una fecha y guarda un
  snapshot persistente.
- **Excluye ventas anuladas** del cálculo.
- El snapshot queda en tablas propias (no se recalculan los históricos al
  cambiar stock/precios). Esto usa los snapshots ya guardados en
  `venta_detalle` (`costo_unitario`, `precio_unitario`, `subtotal`).
- Fórmulas:
  - `total_venta = Σ ventas.total` (ya trae el descuento aplicado).
  - `total_costo = Σ (costo_unitario × cantidad)` de los detalles.
  - `total_ganancia = total_venta − total_costo`.
  - Desglose por tipo: `Σ ventas.total` agrupado por `id_tipo_venta`.
- Zona horaria: `ventas.fecha` se guarda en UTC RFC3339. Para que el "día"
  coincida con el día local del operador, el backend convierte el rango local
  (00:00–24:00 de la fecha elegida) a instantes UTC y filtra por ese rango.
  Se registran `dia`, `mes`, `anio` desde la fecha local elegida.
- Permisos nuevos (2): `ver_cierres`, `crear_cierre` (sync en 3 lugares +
  `usePermissions` + `AuditoriaPage`).
- Errores nuevos: `CierreYaExiste`, `CierreNotFound`.

---

## Backend (Rust)

### 1. Schema — `src-tauri/src/infrastructure/database/mod.rs`

Agregar al `execute_batch` (después de `tipos_venta` de Fase 3):

```sql
CREATE TABLE IF NOT EXISTS cierres (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fecha TEXT NOT NULL UNIQUE,
    dia INTEGER NOT NULL,
    mes INTEGER NOT NULL,
    anio INTEGER NOT NULL,
    total_costo REAL NOT NULL,
    total_ganancia REAL NOT NULL,
    total_venta REAL NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS cierre_tipos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    id_cierre INTEGER NOT NULL,
    id_tipo_venta INTEGER NOT NULL,
    total REAL NOT NULL,
    FOREIGN KEY (id_cierre) REFERENCES cierres(id) ON DELETE CASCADE,
    FOREIGN KEY (id_tipo_venta) REFERENCES tipos_venta(id)
);

CREATE INDEX IF NOT EXISTS idx_cierre_tipos_id_cierre ON cierre_tipos(id_cierre);
```

Agregar los 2 permisos al array `PERMISSIONS`:

```rust
// Cierres del día
"ver_cierres",
"crear_cierre",
```

### 2. Entidades — `src-tauri/src/domain/entities/cierre.rs` (nuevo)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cierre {
    pub id: i64,
    pub fecha: String,          // "YYYY-MM-DD"
    pub dia: i64,
    pub mes: i64,
    pub anio: i64,
    pub total_costo: f64,
    pub total_ganancia: f64,
    pub total_venta: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CierreTipo {
    pub id_tipo_venta: i64,
    pub tipo_venta: String,     // nombre del tipo
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CierreWithTipos {
    #[serde(flatten)]
    pub cierre: Cierre,
    pub tipos: Vec<CierreTipo>,
}
```

Registrar en `src-tauri/src/domain/entities/mod.rs`.

### 3. Repositorio

**Trait** — `src-tauri/src/domain/repositories/cierre_repository.rs` (nuevo):

```rust
pub trait CierreRepository: Send + Sync {
    fn create(
        &self,
        cierre: &Cierre,
        tipos: &[CierreTipo],
    ) -> Result<CierreWithTipos, AppError>;
    fn find_by_fecha(&self, fecha: &str) -> Result<Option<CierreWithTipos>, AppError>;
    fn find_all(&self) -> Result<Vec<CierreWithTipos>, AppError>;
}
```

**Implementación** — `src-tauri/src/infrastructure/repositories/cierre_repository.rs`
(nuevo). `create` bloquea el `Mutex<Connection>` una sola vez y corre la
inserción de cabecera + hijos en una **transacción** (patrón `venta_repository`).
`find_all` hace `LEFT JOIN cierre_tipos` + `tipos_venta` y agrupa en memoria.
`log_audit` siempre fuera de la transacción (deadlock del Mutex).

Registrar ambos en sus `mod.rs`.

### 4. Servicio — `src-tauri/src/application/services/cierre_service.rs` (nuevo)

`crear_cierre(fecha: &str)`:

1. Validar `fecha` (formato `YYYY-MM-DD`) y que no exista ya un cierre
   (`find_by_fecha` → `CierreYaExiste`).
2. Resolver el rango local de la fecha → instantes UTC:

   ```rust
   use chrono::{NaiveDate, Local, TimeZone};
   let day = NaiveDate::parse_from_str(fecha, "%Y-%m-%d")?;
   let start = Local
       .from_local_datetime(&day.and_hms_opt(0, 0, 0).unwrap())
       .single()
       .unwrap()
       .with_timezone(&chrono::Utc)
       .to_rfc3339();
   let end = Local
       .from_local_datetime(&(day + chrono::Duration::days(1)).and_hms_opt(0, 0, 0).unwrap())
       .single()
       .unwrap()
       .with_timezone(&chrono::Utc)
       .to_rfc3339();
   ```

   > `ventas.fecha` es un string RFC3339; la comparación lexicográfica de
   > strings UTC es correcta y aprovecha el formato canónico.

3. Consultar ventas del día (`anulada = 0`, `fecha >= start AND fecha < end`)
   y agregar:

   ```sql
   SELECT v.id_tipo_venta, v.total
   FROM ventas v
   WHERE v.anulada = 0 AND v.fecha >= ?1 AND v.fecha < ?2;

   SELECT d.costo_unitario * d.cantidad
   FROM venta_detalle d
   INNER JOIN ventas v ON v.id = d.id_venta
   WHERE v.anulada = 0 AND v.fecha >= ?1 AND v.fecha < ?2;
   ```

4. Calcular `total_venta`, `total_costo`, `total_ganancia`, desglose por tipo
   (los tipos sin movimientos no entran; en el frontend se completan con 0).
5. Guardar vía repositorio (cabecera + `cierre_tipos`).

`get_all()` → delega al repositorio.

Registrar en `src-tauri/src/application/services/mod.rs`.

### 5. Errores nuevos — `src-tauri/src/infrastructure/error.rs`

| Variante | `code()` | `user_message()` |
|---|---|---|
| `CierreYaExiste` | `cierre_ya_existe` | "Ya existe un cierre para esa fecha." |
| `CierreNotFound` | `cierre_not_found` | "El cierre no existe." |

Agregar a las tres secciones (enum, `code()`, `user_message()`).

### 6. Permiso — `src-tauri/src/domain/entities/permission_code.rs`

Variantes `ViewCierres`, `CreateCierre` en la enum, `as_str()` y `all()` con
strings `ver_cierres`, `crear_cierre`.

### 7. Commands — `src-tauri/src/api/commands/cierre_commands.rs` (nuevo)

- `CierreAppState { cierre_service: Mutex<CierreService> }`.
- `CrearCierreRequest { fecha: String }` (derive Deserialize).
- `crear_cierre(user_id, request, state)` → `check_permission(CreateCierre)` +
  `log_audit(...)` (fuera de la transacción).
- `get_all_cierres(user_id, state)` → `check_permission(ViewCierres)`.
- Registrar en `src-tauri/src/api/commands/mod.rs`.

**Audit** — `domain/entities/audit_log.rs`: agregar `AuditScreen::Cierres`
(`"Cierres del día"`).

### 8. Registro — `src-tauri/src/lib.rs`

- Imports + `.manage(CierreAppState::new())` + comandos en
  `tauri::generate_handler!` (`crear_cierre`, `get_all_cierres`).

---

## Frontend (Vue)

### 1. Tipos — `src/domain/entities/types.ts`

```ts
export interface Cierre {
  id: number;
  fecha: string;
  dia: number;
  mes: number;
  anio: number;
  total_costo: number;
  total_ganancia: number;
  total_venta: number;
  created_at: string;
}

export interface CierreTipo {
  id_tipo_venta: number;
  tipo_venta: string;
  total: number;
}

export interface CierreWithTipos extends Cierre {
  tipos: CierreTipo[];
}
```

### 2. Permisos — `src/domain/entities/permissions.ts`

```ts
VIEW_CIERRES: "ver_cierres",
CREATE_CIERRE: "crear_cierre",
```

### 3. Repositorio — `src/infrastructure/api/cierreRepository.ts` (nuevo)

`getAllCierres`, `crearCierre` (patrón `ventaRepository.ts`, pasando `userId`).

### 4. Store — `src/presentation/stores/index.ts`

`useCierresStore`: `cierres`, `loading`, `error`, `fetchCierres`,
`crearCierre(fecha)` (tras crear, refresca la lista).

### 5. Composable — `src/presentation/composables/usePermissions.ts`

Agregar `canViewCierres`, `canCreateCierre` y devolverlas en el objeto.

### 6. Página — `src/presentation/pages/CierresPage.vue` (nuevo)

- Selector de fecha (`<input type="date">`, default hoy) + botón "Cerrar día"
  (visible con `canCreateCierre`) con confirmación.
- Listado de cierres (patrón `VentasPage`): fecha, día/mes/año, total venta,
  total costo, ganancia, desglose por tipo (tabla o detalle expandible).
- Formato de dinero con `formatMoney` (`src/presentation/utils/format.ts`).
- Muestra el error del backend si la fecha ya tiene cierre ("Ya existe un
  cierre para esa fecha.").

### 7. Ruta — `src/presentation/router/index.ts`

```ts
{
  path: "cierres",
  name: "cierres",
  component: () => import("../pages/CierresPage.vue"),
  meta: { permission: PERMISSIONS.VIEW_CIERRES },
},
```

### 8. Menú — `src/presentation/layouts/MainLayout.vue`

Item "Cierres del día" (icono `./svg/calendar.svg` o similar, permission
`ver_cierres`). Agregar el SVG en `src/public/svg/` si no existe.

### 9. Auditoría — `src/presentation/pages/AuditoriaPage.vue`

Agregar `"Cierres del día"` al array `screens`.

---

## Archivos tocados (resumen)

**Rust nuevos**: `domain/entities/cierre.rs`, `domain/repositories/cierre_repository.rs`,
`infrastructure/repositories/cierre_repository.rs`, `application/services/cierre_service.rs`,
`api/commands/cierre_commands.rs`.

**Rust modificados**: `database/mod.rs`, `entities/mod.rs`, `entities/permission_code.rs`,
`entities/audit_log.rs`, `error.rs`, `services/mod.rs`, `api/commands/mod.rs`, `lib.rs`.

**Frontend nuevos**: `infrastructure/api/cierreRepository.ts`,
`presentation/pages/CierresPage.vue`, `public/svg/calendar.svg`.

**Frontend modificados**: `domain/entities/types.ts`, `domain/entities/permissions.ts`,
`presentation/stores/index.ts`, `presentation/composables/usePermissions.ts`,
`presentation/router/index.ts`, `presentation/layouts/MainLayout.vue`,
`presentation/pages/AuditoriaPage.vue`.

## Verificación

- `cd src-tauri && cargo check` y `cargo clippy`.
- `pnpm build`.
- Prueba manual:
  - Crear ventas (varios tipos) de hoy y anular una → cerrar el día de hoy:
    totales correctos, sin la anulada, desglose por tipo correcto.
  - Cerrar la misma fecha de nuevo → error "Ya existe un cierre para esa fecha.".
  - Cerrar un día sin ventas → cierre con totales en 0 y desglose vacío.
  - Verificar el listado histórico y el desglose.
  - Un usuario sin `ver_cierres` no ve el módulo ni la ruta.
