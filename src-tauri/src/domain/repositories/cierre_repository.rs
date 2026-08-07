use crate::domain::entities::{Cierre, CierreTipo, CierreWithTipos};
use crate::infrastructure::error::AppError;

pub trait CierreRepository: Send + Sync {
    fn create(
        &self,
        cierre: &Cierre,
        tipos: &[CierreTipo],
    ) -> Result<CierreWithTipos, AppError>;
    fn find_by_fecha(&self, fecha: &str) -> Result<Option<CierreWithTipos>, AppError>;
    fn find_all(&self) -> Result<Vec<CierreWithTipos>, AppError>;
}
