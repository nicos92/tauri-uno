use rusqlite::params;

use crate::domain::entities::SubCategoria;
use crate::domain::repositories::SubCategoriaRepository;
use crate::infrastructure::database::DB;
use crate::infrastructure::error::AppError;

pub struct SqliteSubCategoriaRepository;

impl SqliteSubCategoriaRepository {
    pub fn new() -> Self {
        Self
    }
}

impl SubCategoriaRepository for SqliteSubCategoriaRepository {
    fn create(&self, sub_categoria: &SubCategoria) -> Result<SubCategoria, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO sub_categorias (sub_categoria, id_categoria) VALUES (?1, ?2)",
            params![sub_categoria.sub_categoria, sub_categoria.id_categoria],
        )?;

        let id = conn.last_insert_rowid();
        Ok(SubCategoria {
            id,
            ..sub_categoria.clone()
        })
    }

    fn find_by_id(&self, id: i64) -> Result<Option<SubCategoria>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn
            .prepare("SELECT id, sub_categoria, id_categoria FROM sub_categorias WHERE id = ?1")?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_sub_categoria(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_by_name(&self, sub_categoria: &str) -> Result<Option<SubCategoria>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, sub_categoria, id_categoria FROM sub_categorias WHERE sub_categoria = ?1",
        )?;

        let mut rows = stmt.query(params![sub_categoria])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_sub_categoria(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_all(&self) -> Result<Vec<SubCategoria>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, sub_categoria, id_categoria FROM sub_categorias ORDER BY sub_categoria",
        )?;

        let mut sub_categorias = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            sub_categorias.push(self.row_to_sub_categoria(row)?);
        }

        Ok(sub_categorias)
    }

    fn find_by_categoria(&self, id_categoria: i64) -> Result<Vec<SubCategoria>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, sub_categoria, id_categoria FROM sub_categorias WHERE id_categoria = ?1 ORDER BY sub_categoria"
        )?;

        let mut sub_categorias = Vec::new();
        let mut rows = stmt.query(params![id_categoria])?;

        while let Some(row) = rows.next()? {
            sub_categorias.push(self.row_to_sub_categoria(row)?);
        }

        Ok(sub_categorias)
    }

    fn update(&self, sub_categoria: &SubCategoria) -> Result<SubCategoria, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE sub_categorias SET sub_categoria = ?1, id_categoria = ?2 WHERE id = ?3",
            params![
                sub_categoria.sub_categoria,
                sub_categoria.id_categoria,
                sub_categoria.id
            ],
        )?;

        Ok(sub_categoria.clone())
    }

    fn delete(&self, id: i64) -> Result<(), AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        conn.execute("DELETE FROM sub_categorias WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn has_articulos(&self, id: i64) -> Result<bool, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM articulos WHERE id_sub_categoria = ?1",
            params![id],
            |row| row.get(0),
        )?;

        Ok(count > 0)
    }
}

impl SqliteSubCategoriaRepository {
    fn row_to_sub_categoria(&self, row: &rusqlite::Row) -> Result<SubCategoria, AppError> {
        Ok(SubCategoria {
            id: row.get(0)?,
            sub_categoria: row.get(1)?,
            id_categoria: row.get(2)?,
        })
    }
}
