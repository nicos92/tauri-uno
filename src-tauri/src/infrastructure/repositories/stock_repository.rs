use rusqlite::params;

use crate::domain::entities::Stock;
use crate::domain::repositories::StockRepository;
use crate::infrastructure::database::DB;
use crate::infrastructure::error::AppError;

pub struct SqliteStockRepository;

impl SqliteStockRepository {
    pub fn new() -> Self {
        Self
    }
}

impl StockRepository for SqliteStockRepository {
    fn create(&self, stock: &Stock) -> Result<Stock, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO stock (id_articulo, cantidad, costo, ganancia) VALUES (?1, ?2, ?3, ?4)",
            params![
                stock.id_articulo,
                stock.cantidad,
                stock.costo,
                stock.ganancia
            ],
        )?;

        let id = conn.last_insert_rowid();
        Ok(Stock {
            id,
            ..stock.clone()
        })
    }

    fn find_by_id(&self, id: i64) -> Result<Option<Stock>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, id_articulo, cantidad, costo, ganancia FROM stock WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_stock(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_by_articulo(&self, id_articulo: i64) -> Result<Option<Stock>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, id_articulo, cantidad, costo, ganancia FROM stock WHERE id_articulo = ?1",
        )?;

        let mut rows = stmt.query(params![id_articulo])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_stock(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_all(&self) -> Result<Vec<Stock>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn
            .prepare("SELECT id, id_articulo, cantidad, costo, ganancia FROM stock ORDER BY id")?;

        let mut stocks = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            stocks.push(self.row_to_stock(row)?);
        }

        Ok(stocks)
    }

    fn update(&self, stock: &Stock) -> Result<Stock, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE stock SET cantidad = ?1, costo = ?2, ganancia = ?3 WHERE id = ?4",
            params![stock.cantidad, stock.costo, stock.ganancia, stock.id],
        )?;

        Ok(stock.clone())
    }

    fn delete(&self, id: i64) -> Result<(), AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        conn.execute("DELETE FROM stock WHERE id = ?1", params![id])?;
        Ok(())
    }
}

impl SqliteStockRepository {
    fn row_to_stock(&self, row: &rusqlite::Row) -> Result<Stock, AppError> {
        Ok(Stock {
            id: row.get(0)?,
            id_articulo: row.get(1)?,
            cantidad: row.get(2)?,
            costo: row.get(3)?,
            ganancia: row.get(4)?,
        })
    }
}
