use crate::domain::entities::{Stock, StockPreview};
use crate::infrastructure::error::AppError;

#[cfg_attr(test, mockall::automock)]
pub trait StockRepository: Send + Sync {
    fn create(&self, stock: &Stock) -> Result<Stock, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<Stock>, AppError>;
    fn find_by_articulo(&self, id_articulo: i64) -> Result<Option<Stock>, AppError>;
    fn find_all(&self) -> Result<Vec<Stock>, AppError>;
    fn update(&self, stock: &Stock) -> Result<Stock, AppError>;
    fn delete(&self, id: i64) -> Result<(), AppError>;
    fn has_ventas(&self, id_articulo: i64) -> Result<bool, AppError>;
    fn find_filtered_with_preview(
        &self,
        porcentaje: f64,
        id_categoria: Option<i64>,
        id_sub_categoria: Option<i64>,
        id_proveedor: Option<i64>,
    ) -> Result<Vec<StockPreview>, AppError>;
    fn apply_costo_percentage(
        &self,
        porcentaje: f64,
        id_categoria: Option<i64>,
        id_sub_categoria: Option<i64>,
        id_proveedor: Option<i64>,
    ) -> Result<i64, AppError>;
}
