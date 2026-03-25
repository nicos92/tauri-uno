use std::sync::Arc;

use crate::domain::entities::Stock;
use crate::domain::repositories::StockRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteStockRepository;

pub struct StockService {
    repository: Arc<SqliteStockRepository>,
}

impl StockService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(SqliteStockRepository::new()),
        }
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
        let _existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::StockNotFound)?;

        self.repository.delete(id)
    }

    pub fn get_precio_venta(&self, id: i64) -> Result<f64, AppError> {
        let stock = self.get_by_id(id)?;
        let precio_venta = stock.costo * (1.0 + stock.ganancia / 100.0);
        Ok(precio_venta)
    }
}
