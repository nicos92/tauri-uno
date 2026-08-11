use crate::domain::entities::TipoVenta;
use crate::infrastructure::error::AppError;

#[cfg_attr(test, mockall::automock)]
pub trait TipoVentaRepository: Send + Sync {
    fn find_all(&self) -> Result<Vec<TipoVenta>, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<TipoVenta>, AppError>;
    fn find_by_nombre(&self, nombre: &str) -> Result<Option<TipoVenta>, AppError>;
    fn create(&self, tipo: &TipoVenta) -> Result<TipoVenta, AppError>;
    fn update(&self, tipo: &TipoVenta) -> Result<TipoVenta, AppError>;
    fn delete(&self, id: i64) -> Result<(), AppError>;
    fn has_ventas(&self, id: i64) -> Result<bool, AppError>;
}
