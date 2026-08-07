use rusqlite::params;

use crate::domain::entities::Categoria;
use crate::domain::repositories::CategoriaRepository;
use crate::infrastructure::database::DB;
use crate::infrastructure::error::AppError;

pub struct SqliteCategoriaRepository;

impl SqliteCategoriaRepository {
    pub fn new() -> Self {
        Self
    }
}

impl CategoriaRepository for SqliteCategoriaRepository {
    fn create(&self, categoria: &Categoria) -> Result<Categoria, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO categorias (categoria) VALUES (?1)",
            params![categoria.categoria],
        )?;

        let id = conn.last_insert_rowid();
        Ok(Categoria {
            id,
            ..categoria.clone()
        })
    }

    fn find_by_id(&self, id: i64) -> Result<Option<Categoria>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare("SELECT id, categoria FROM categorias WHERE id = ?1")?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_categoria(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_by_name(&self, categoria: &str) -> Result<Option<Categoria>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare("SELECT id, categoria FROM categorias WHERE categoria = ?1")?;

        let mut rows = stmt.query(params![categoria])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_categoria(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_all(&self) -> Result<Vec<Categoria>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare("SELECT id, categoria FROM categorias ORDER BY categoria")?;

        let mut categorias = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            categorias.push(self.row_to_categoria(row)?);
        }

        Ok(categorias)
    }

    fn update(&self, categoria: &Categoria) -> Result<Categoria, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE categorias SET categoria = ?1 WHERE id = ?2",
            params![categoria.categoria, categoria.id],
        )?;

        Ok(categoria.clone())
    }

    fn delete(&self, id: i64) -> Result<(), AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        conn.execute("DELETE FROM categorias WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn has_sub_categorias(&self, id: i64) -> Result<bool, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sub_categorias WHERE id_categoria = ?1",
            params![id],
            |row| row.get(0),
        )?;

        Ok(count > 0)
    }
}

impl SqliteCategoriaRepository {
    fn row_to_categoria(&self, row: &rusqlite::Row) -> Result<Categoria, AppError> {
        Ok(Categoria {
            id: row.get(0)?,
            categoria: row.get(1)?,
        })
    }
}
