use rusqlite::params;

use crate::domain::entities::Stock;
use crate::domain::repositories::StockRepository;
use crate::infrastructure::database::DB;
use crate::infrastructure::error::AppError;

pub struct SqliteStockRepository;

impl Default for SqliteStockRepository {
    fn default() -> Self {
        Self::new()
    }
}

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

    fn has_ventas(&self, id_articulo: i64) -> Result<bool, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM venta_detalle WHERE id_articulo = ?1",
            params![id_articulo],
            |row| row.get(0),
        )?;

        Ok(count > 0)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Articulo, Categoria, Proveedor, SubCategoria};
    use crate::domain::repositories::{
        ArticuloRepository, CategoriaRepository, ProveedorRepository, SubCategoriaRepository,
    };
    use crate::infrastructure::database::{reset_test_db, DB, TEST_LOCK};
    use crate::infrastructure::repositories::{
        SqliteArticuloRepository, SqliteCategoriaRepository, SqliteProveedorRepository,
        SqliteSubCategoriaRepository,
    };
    use std::sync::MutexGuard;

    fn fresh_db() -> MutexGuard<'static, ()> {
        let guard = TEST_LOCK.lock().unwrap();
        reset_test_db().unwrap();
        guard
    }

    fn create_articulo() -> Articulo {
        let cat_repo = SqliteCategoriaRepository::new();
        let cat = cat_repo
            .create(&Categoria::new("Cat STK".to_string()))
            .unwrap();
        let sub_repo = SqliteSubCategoriaRepository::new();
        let sub = sub_repo
            .create(&SubCategoria::new("Sub STK".to_string(), cat.id))
            .unwrap();
        let prov_repo = SqliteProveedorRepository::new();
        let prov = prov_repo
            .create(&Proveedor::new(
                "PROV STK".to_string(),
                "Prov Stk".to_string(),
                None,
                None,
                None,
                None,
            ))
            .unwrap();
        SqliteArticuloRepository::new()
            .create(&Articulo::new(
                "Art STK".to_string(),
                "STK-001".to_string(),
                sub.id,
                prov.id,
            ))
            .unwrap()
    }

    #[test]
    fn create_assigns_id_and_find_by_id_round_trip() {
        let _guard = fresh_db();
        let articulo = create_articulo();
        let repo = SqliteStockRepository::new();

        let created = repo
            .create(&Stock::new(articulo.id, 10.5, 100.0, 25.0))
            .unwrap();
        assert!(created.id > 0);

        let found = repo.find_by_id(created.id).unwrap().unwrap();
        assert_eq!(found.id_articulo, articulo.id);
        assert_eq!(found.cantidad, 10.5);
        assert_eq!(found.costo, 100.0);
        assert_eq!(found.ganancia, 25.0);
    }

    #[test]
    fn find_by_articulo_returns_stock() {
        let _guard = fresh_db();
        let articulo = create_articulo();
        let repo = SqliteStockRepository::new();

        let created = repo
            .create(&Stock::new(articulo.id, 5.0, 50.0, 10.0))
            .unwrap();
        let found = repo.find_by_articulo(articulo.id).unwrap().unwrap();
        assert_eq!(found.id, created.id);
        assert!(repo.find_by_articulo(99999).unwrap().is_none());
    }

    #[test]
    fn update_changes_cantidad() {
        let _guard = fresh_db();
        let articulo = create_articulo();
        let repo = SqliteStockRepository::new();

        let mut created = repo
            .create(&Stock::new(articulo.id, 5.0, 50.0, 10.0))
            .unwrap();
        created.cantidad = 20.0;
        repo.update(&created).unwrap();

        let updated = repo.find_by_id(created.id).unwrap().unwrap();
        assert_eq!(updated.cantidad, 20.0);
    }

    #[test]
    fn delete_removes_stock() {
        let _guard = fresh_db();
        let articulo = create_articulo();
        let repo = SqliteStockRepository::new();

        let created = repo
            .create(&Stock::new(articulo.id, 5.0, 50.0, 10.0))
            .unwrap();
        repo.delete(created.id).unwrap();
        assert!(repo.find_by_id(created.id).unwrap().is_none());
        assert!(repo.find_all().unwrap().iter().all(|s| s.id != created.id));
    }

    fn insert_venta_for_articulo(id_articulo: i64) {
        let conn = DB.lock().unwrap();
        let user_id: i64 = conn
            .query_row(
                "SELECT id FROM users WHERE username = 'admin'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO ventas (user_id, fecha, total, descuento, anulada, observacion, id_tipo_venta, created_at, cliente_id)
             VALUES (?1, ?2, 150.0, 0, 0, NULL, NULL, ?3, (SELECT id FROM clientes WHERE nombre = 'Consumidor' AND apellido = 'Final' LIMIT 1))",
            params![user_id, now, now],
        )
        .unwrap();
        let venta_id = conn.last_insert_rowid();
        conn.execute(
            "INSERT INTO venta_detalle (id_venta, id_articulo, cantidad, costo_unitario, precio_unitario, subtotal) VALUES (?1, ?2, 1.0, 100.0, 150.0, 150.0)",
            params![venta_id, id_articulo],
        )
        .unwrap();
    }

    #[test]
    fn has_ventas_returns_true_when_article_has_sales() {
        let _guard = fresh_db();
        let articulo = create_articulo();
        let repo = SqliteStockRepository::new();
        repo.create(&Stock::new(articulo.id, 10.0, 100.0, 25.0))
            .unwrap();

        insert_venta_for_articulo(articulo.id);

        assert!(repo.has_ventas(articulo.id).unwrap());
    }

    #[test]
    fn has_ventas_returns_false_when_article_has_no_sales() {
        let _guard = fresh_db();
        let articulo = create_articulo();
        let repo = SqliteStockRepository::new();
        repo.create(&Stock::new(articulo.id, 10.0, 100.0, 25.0))
            .unwrap();

        assert!(!repo.has_ventas(articulo.id).unwrap());
        assert!(!repo.has_ventas(99999).unwrap());
    }
}
