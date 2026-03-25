use rusqlite::params;

use crate::domain::entities::Proveedor;
use crate::domain::repositories::ProveedorRepository;
use crate::infrastructure::database::DB;
use crate::infrastructure::error::AppError;

pub struct SqliteProveedorRepository;

impl SqliteProveedorRepository {
    pub fn new() -> Self {
        Self
    }
}

impl ProveedorRepository for SqliteProveedorRepository {
    fn create(&self, proveedor: &Proveedor) -> Result<Proveedor, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO proveedores (cuit, proveedor, nombre, tel, email, observacion) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                proveedor.cuit,
                proveedor.proveedor,
                proveedor.nombre,
                proveedor.tel,
                proveedor.email,
                proveedor.observacion
            ],
        )?;

        let id = conn.last_insert_rowid();
        Ok(Proveedor {
            id,
            ..proveedor.clone()
        })
    }

    fn find_by_id(&self, id: i64) -> Result<Option<Proveedor>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, cuit, proveedor, nombre, tel, email, observacion FROM proveedores WHERE id = ?1"
        )?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_proveedor(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_by_cuit(&self, cuit: &str) -> Result<Option<Proveedor>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, cuit, proveedor, nombre, tel, email, observacion FROM proveedores WHERE cuit = ?1"
        )?;

        let mut rows = stmt.query(params![cuit])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_proveedor(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_all(&self) -> Result<Vec<Proveedor>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, cuit, proveedor, nombre, tel, email, observacion FROM proveedores ORDER BY proveedor"
        )?;

        let mut proveedores = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            proveedores.push(self.row_to_proveedor(row)?);
        }

        Ok(proveedores)
    }

    fn update(&self, proveedor: &Proveedor) -> Result<Proveedor, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE proveedores SET cuit = ?1, proveedor = ?2, nombre = ?3, tel = ?4, email = ?5, observacion = ?6 WHERE id = ?7",
            params![
                proveedor.cuit,
                proveedor.proveedor,
                proveedor.nombre,
                proveedor.tel,
                proveedor.email,
                proveedor.observacion,
                proveedor.id
            ],
        )?;

        Ok(proveedor.clone())
    }

    fn delete(&self, id: i64) -> Result<(), AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        conn.execute("DELETE FROM proveedores WHERE id = ?1", params![id])?;
        Ok(())
    }
}

impl SqliteProveedorRepository {
    fn row_to_proveedor(&self, row: &rusqlite::Row) -> Result<Proveedor, AppError> {
        Ok(Proveedor {
            id: row.get(0)?,
            cuit: row.get(1)?,
            proveedor: row.get(2)?,
            nombre: row.get(3)?,
            tel: row.get(4)?,
            email: row.get(5)?,
            observacion: row.get(6)?,
        })
    }
}
