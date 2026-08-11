use std::sync::Arc;

use crate::domain::entities::Stock;
use crate::domain::repositories::StockRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteStockRepository;

pub struct StockService {
    repository: Arc<dyn StockRepository>,
}

impl Default for StockService {
    fn default() -> Self {
        Self::new()
    }
}

impl StockService {
    pub fn new() -> Self {
        Self::with_repository(Arc::new(SqliteStockRepository::new()))
    }

    pub fn with_repository(repository: Arc<dyn StockRepository>) -> Self {
        Self { repository }
    }

    pub fn create(
        &self,
        id_articulo: i64,
        cantidad: f64,
        costo: f64,
        ganancia: f64,
    ) -> Result<Stock, AppError> {
        let existing = self.repository.find_by_articulo(id_articulo)?;
        if existing.is_some() {
            return Err(AppError::StockExistsForArticulo);
        }

        let new_stock = Stock::new(id_articulo, cantidad, costo, ganancia);
        self.repository.create(&new_stock)
    }

    pub fn get_all(&self) -> Result<Vec<Stock>, AppError> {
        self.repository.find_all()
    }

    pub fn get_by_id(&self, id: i64) -> Result<Stock, AppError> {
        self.repository
            .find_by_id(id)?
            .ok_or(AppError::StockNotFound)
    }

    pub fn get_by_articulo(&self, id_articulo: i64) -> Result<Option<Stock>, AppError> {
        self.repository.find_by_articulo(id_articulo)
    }

    pub fn update(
        &self,
        id: i64,
        cantidad: f64,
        costo: f64,
        ganancia: f64,
    ) -> Result<Stock, AppError> {
        let mut existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::StockNotFound)?;

        existing.cantidad = cantidad;
        existing.costo = costo;
        existing.ganancia = ganancia;

        self.repository.update(&existing)
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        let existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::StockNotFound)?;

        let has_ventas = self.repository.has_ventas(existing.id_articulo)?;
        if has_ventas {
            return Err(AppError::StockHasVentas);
        }

        self.repository.delete(id)
    }

    pub fn get_precio_venta(&self, id: i64) -> Result<f64, AppError> {
        let stock = self.get_by_id(id)?;
        let precio_venta = stock.costo * (1.0 + stock.ganancia / 100.0);
        Ok(precio_venta)
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
            .create(&Categoria::new("Cat STK SV".to_string()))
            .unwrap();
        let sub_repo = SqliteSubCategoriaRepository::new();
        let sub = sub_repo
            .create(&SubCategoria::new("Sub STK SV".to_string(), cat.id))
            .unwrap();
        let prov_repo = SqliteProveedorRepository::new();
        let prov = prov_repo
            .create(&Proveedor::new(
                "PROV STK SV".to_string(),
                "Prov Stk Sv".to_string(),
                None,
                None,
                None,
                None,
            ))
            .unwrap();
        SqliteArticuloRepository::new()
            .create(&Articulo::new(
                "Art STK SV".to_string(),
                "STK-SV-001".to_string(),
                sub.id,
                prov.id,
            ))
            .unwrap()
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
            "INSERT INTO ventas (user_id, fecha, total, descuento, anulada, observacion, id_tipo_venta, created_at) VALUES (?1, ?2, 150.0, 0, 0, NULL, NULL, ?3)",
            rusqlite::params![user_id, now, now],
        )
        .unwrap();
        let venta_id = conn.last_insert_rowid();
        conn.execute(
            "INSERT INTO venta_detalle (id_venta, id_articulo, cantidad, costo_unitario, precio_unitario, subtotal) VALUES (?1, ?2, 1.0, 100.0, 150.0, 150.0)",
            rusqlite::params![venta_id, id_articulo],
        )
        .unwrap();
    }

    #[test]
    fn delete_rejects_stock_with_sales() {
        let _guard = fresh_db();
        let articulo = create_articulo();
        let service = StockService::new();
        let stock = service.create(articulo.id, 10.0, 100.0, 25.0).unwrap();

        insert_venta_for_articulo(articulo.id);

        let err = service.delete(stock.id).unwrap_err();
        assert!(matches!(err, AppError::StockHasVentas));
        assert!(service.get_by_id(stock.id).is_ok());
    }

    #[test]
    fn delete_allows_stock_without_sales() {
        let _guard = fresh_db();
        let articulo = create_articulo();
        let service = StockService::new();
        let stock = service.create(articulo.id, 10.0, 100.0, 25.0).unwrap();

        service.delete(stock.id).unwrap();
        assert!(matches!(
            service.get_by_id(stock.id),
            Err(AppError::StockNotFound)
        ));
    }
}
