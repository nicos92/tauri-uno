use crate::domain::entities::{Presupuesto, PresupuestoDetalle, PresupuestoWithDetalle};
use crate::domain::repositories::Page;
use crate::infrastructure::error::AppError;

#[cfg_attr(test, mockall::automock)]
pub trait PresupuestoRepository: Send + Sync {
    fn create(
        &self,
        presupuesto: &Presupuesto,
        detalles: &[PresupuestoDetalle],
    ) -> Result<PresupuestoWithDetalle, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<PresupuestoWithDetalle>, AppError>;
    fn find_page(&self, limit: i64, offset: i64) -> Result<Page<PresupuestoWithDetalle>, AppError>;
}
