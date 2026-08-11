use crate::domain::entities::{Venta, VentaDetalle, VentaWithDetalle};
use crate::domain::repositories::Page;
use crate::infrastructure::error::AppError;

#[cfg_attr(test, mockall::automock)]
pub trait VentaRepository: Send + Sync {
    fn create(
        &self,
        venta: &Venta,
        detalles: &[VentaDetalle],
        allow_negative_stock: bool,
    ) -> Result<VentaWithDetalle, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<VentaWithDetalle>, AppError>;
    fn find_by_cliente(&self, cliente_id: i64) -> Result<Vec<VentaWithDetalle>, AppError>;
    fn find_all(&self) -> Result<Vec<VentaWithDetalle>, AppError>;
    fn find_page(&self, limit: i64, offset: i64) -> Result<Page<VentaWithDetalle>, AppError>;
    fn anular(&self, id: i64) -> Result<(), AppError>;
}
