use crate::domain::entities::Stock;
use crate::infrastructure::error::AppError;

pub trait StockRepository: Send + Sync {
    fn create(&self, stock: &Stock) -> Result<Stock, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<Stock>, AppError>;
    fn find_by_articulo(&self, id_articulo: i64) -> Result<Option<Stock>, AppError>;
    fn find_all(&self) -> Result<Vec<Stock>, AppError>;
    fn update(&self, stock: &Stock) -> Result<Stock, AppError>;
    fn delete(&self, id: i64) -> Result<(), AppError>;
}
