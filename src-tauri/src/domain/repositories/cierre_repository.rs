use crate::domain::entities::CierreWithTipos;
use crate::infrastructure::error::AppError;

pub trait CierreRepository: Send + Sync {
    fn find_by_fecha(&self, fecha: &str) -> Result<Option<CierreWithTipos>, AppError>;
    fn find_all(&self) -> Result<Vec<CierreWithTipos>, AppError>;
    fn delete_by_fecha(&self, fecha: &str) -> Result<(), AppError>;
}
