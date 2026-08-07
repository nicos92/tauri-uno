# Fix: Se podía eliminar un artículo que tenía stock asociado

Fecha: 2026-08-07

## Problema

Desde la aplicación se podía eliminar un artículo que estaba referenciado en la
tabla `stock`. El stock asociado se perdía silenciosamente y el `DELETE` nunca
fallaba por integridad referencial, por más que la FK estuviera declarada en el
esquema.

## Análisis / Causa raíz

El método `delete()` de `SqliteArticuloRepository`
(`src-tauri/src/infrastructure/repositories/articulo_repository.rs`) implementaba
un **cascade manual**:

```rust
fn delete(&self, id: i64) -> Result<(), AppError> {
    let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

    conn.execute("DELETE FROM stock WHERE id_articulo = ?1", params![id])?; // borra el stock primero
    conn.execute("DELETE FROM articulos WHERE id = ?1", params![id])?;
    Ok(())
}
```

- El esquema define `stock.id_articulo` como FK a `articulos(id)` sin
  `ON DELETE CASCADE` (`src-tauri/src/infrastructure/database/mod.rs`).
- Con `PRAGMA foreign_keys = ON`, borrar el artículo referenciado **debería**
  fallar con `SQLITE_CONSTRAINT_FOREIGNKEY` (787).
- Pero el repositorio borraba primero las filas de `stock` y luego el artículo,
  por lo que la FK nunca llegaba a violarse y el stock se perdía en silencio.
- El service (`articulo_service.rs::delete`) solo verifica que el artículo
  exista; no hay pre-check de stock.

## Solución

Eliminar el `DELETE FROM stock` manual: que la FK haga su trabajo. Al intentar
borrar un artículo con stock, SQLite lanza el error 787, el `From<rusqlite::Error>`
de `error.rs` lo mapea a `AppError::ForeignKeyConstraint`, y el manejo de errores
estructurado ya implementado (`{ code, message }` + toasts) lo muestra en el
frontend sin ocultar la tabla.

Cambio en `src-tauri/src/infrastructure/repositories/articulo_repository.rs`:

```rust
fn delete(&self, id: i64) -> Result<(), AppError> {
    let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

    conn.execute("DELETE FROM articulos WHERE id = ?1", params![id])?;
    Ok(())
}
```

### Flujo resultante

1. `ArticuloService::delete` → `SqliteArticuloRepository::delete` ejecuta el
   `DELETE` de `articulos`.
2. SQLite (FKs activas) devuelve `SQLITE_CONSTRAINT_FOREIGNKEY` (787).
3. `From<rusqlite::Error>` (`error.rs`) lo mapea a `AppError::ForeignKeyConstraint`.
4. Serialización estructurada → `{ code: "foreign_key_constraint", message: "No se puede eliminar porque otros registros hacen referencia a este elemento." }`.
5. Frontend: el store normaliza con `toErrorMessage`, `handleDelete` muestra el
   toast de error (`ArticulosPage.vue`); la tabla se mantiene visible.

## Verificación

- `cd src-tauri && cargo check`
- `pnpm build`
- Prueba manual: artículo con stock → eliminar → toast con mensaje de FK; el
  artículo no se borra. Artículo sin stock → se elimina normal.
