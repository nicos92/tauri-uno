# Fix: Warning de clippy `too_many_arguments` en ProveedorService

Fecha: 2026-08-11

## Problema

`cargo clippy` emite un warning:

```
warning: this function has too many arguments (8/7)
  --> src\application\services\proveedor_service.rs:57:5
```

## Análisis / Causa raíz

Clippy activa `too_many_arguments` (umbral por defecto de 7). El método
`ProveedorService::update()` recibe 8 argumentos contando `&self`:

```rust
pub fn update(
    &self,
    id: i64,
    proveedor: String,
    nombre: String,
    cuit: Option<String>,
    tel: Option<String>,
    email: Option<String>,
    observacion: Option<String>,
) -> Result<Proveedor, AppError>
```

- `create()` (línea 25) queda justo en 7 argumentos, por eso no lo marca.
- Es el único warning porque los demás services tienen menos argumentos
  (Articulo 6, Categoria 3, Stock/SubCategoria 5-6, User 4).
- El patrón problemático: el service desarma la entidad `Proveedor` campo por
  campo, cuando:
  - el trait `ProveedorRepository::update(&self, proveedor: &Proveedor)` ya
    recibe la entidad completa, y
  - el command layer ya define `UpdateProveedorRequest` con exactamente los
    mismos campos (`src-tauri/src/api/commands/proveedor_commands.rs`).

## Solución

Cambiar la firma de `update()` para recibir la entidad `Proveedor` como único
argumento de dominio. El `id` queda embebido en `proveedor.id`, con lo que la
validación de CUIT duplicado (que excluye el propio registro) no cambia de
lógica.

### `src-tauri/src/application/services/proveedor_service.rs`

```rust
pub fn update(&self, proveedor: &Proveedor) -> Result<Proveedor, AppError> {
    let mut existing = self
        .repository
        .find_by_id(proveedor.id)?
        .ok_or(AppError::ProveedorNotFound)?;

    if let Some(ref c) = proveedor.cuit {
        if !c.is_empty() {
            let existing_cuit = self.repository.find_by_cuit(c)?;
            if let Some(ref ec) = existing_cuit {
                if ec.id != proveedor.id {
                    return Err(AppError::DuplicateCuit);
                }
            }
        }
    }

    existing.proveedor = proveedor.proveedor.clone();
    existing.nombre = proveedor.nombre.clone();
    existing.cuit = proveedor.cuit.clone();
    existing.tel = proveedor.tel.clone();
    existing.email = proveedor.email.clone();
    existing.observacion = proveedor.observacion.clone();

    self.repository.update(&existing)
}
```

### `src-tauri/src/api/commands/proveedor_commands.rs`

El command `update_proveedor` construye la entidad a partir del request:

```rust
let proveedor = Proveedor {
    id: request.id,
    proveedor: request.proveedor,
    nombre: request.nombre,
    cuit: request.cuit,
    tel: request.tel,
    email: request.email,
    observacion: request.observacion,
};
let result = service.update(&proveedor)?;
```

### Impacto

- Sin cambios en el frontend: la firma del `invoke` (`update_proveedor` con
  `request`) no cambia.
- La firma de `update()` pasa de 8 argumentos a 2 (`&self`, `&Proveedor`),
  eliminando el warning.
- `create()` se deja igual (no genera warning y su refactor es opcional).

## Verificación

- `cd src-tauri && cargo check`
- `cd src-tauri && cargo clippy` → sin warnings
- `pnpm build` (typecheck del frontend)
- Prueba manual: editar un proveedor con CUIT duplicado de otro → error; editar
  un proveedor normalmente → se guarda.
