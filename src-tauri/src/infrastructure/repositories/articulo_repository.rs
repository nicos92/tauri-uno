use rusqlite::params;

use crate::domain::entities::Articulo;
use crate::domain::repositories::ArticuloRepository;
use crate::infrastructure::database::DB;
use crate::infrastructure::error::AppError;

pub struct SqliteArticuloRepository;

impl SqliteArticuloRepository {
    pub fn new() -> Self {
        Self
    }
}

impl ArticuloRepository for SqliteArticuloRepository {
    fn create(&self, articulo: &Articulo) -> Result<Articulo, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO articulos (articulo, cod_articulo, id_sub_categoria, id_proveedor) VALUES (?1, ?2, ?3, ?4)",
            params![
                articulo.articulo,
                articulo.cod_articulo,
                articulo.id_sub_categoria,
                articulo.id_proveedor
            ],
        )?;

        let id = conn.last_insert_rowid();
        Ok(Articulo {
            id,
            ..articulo.clone()
        })
    }

    fn find_by_id(&self, id: i64) -> Result<Option<Articulo>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, articulo, cod_articulo, id_sub_categoria, id_proveedor FROM articulos WHERE id = ?1"
        )?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_articulo(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_by_codigo(&self, cod_articulo: &str) -> Result<Option<Articulo>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, articulo, cod_articulo, id_sub_categoria, id_proveedor FROM articulos WHERE cod_articulo = ?1"
        )?;

        let mut rows = stmt.query(params![cod_articulo])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_articulo(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_all(&self) -> Result<Vec<Articulo>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, articulo, cod_articulo, id_sub_categoria, id_proveedor FROM articulos ORDER BY articulo"
        )?;

        let mut articulos = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            articulos.push(self.row_to_articulo(row)?);
        }

        Ok(articulos)
    }

    fn update(&self, articulo: &Articulo) -> Result<Articulo, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE articulos SET articulo = ?1, cod_articulo = ?2, id_sub_categoria = ?3, id_proveedor = ?4 WHERE id = ?5",
            params![
                articulo.articulo,
                articulo.cod_articulo,
                articulo.id_sub_categoria,
                articulo.id_proveedor,
                articulo.id
            ],
        )?;

        Ok(articulo.clone())
    }

    fn delete(&self, id: i64) -> Result<(), AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute("DELETE FROM stock WHERE id_articulo = ?1", params![id])?;
        conn.execute("DELETE FROM articulos WHERE id = ?1", params![id])?;
        Ok(())
    }
}

impl SqliteArticuloRepository {
    fn row_to_articulo(&self, row: &rusqlite::Row) -> Result<Articulo, AppError> {
        Ok(Articulo {
            id: row.get(0)?,
            articulo: row.get(1)?,
            cod_articulo: row.get(2)?,
            id_sub_categoria: row.get(3)?,
            id_proveedor: row.get(4)?,
        })
    }
}
