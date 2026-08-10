use crate::domain::entities::CierreWithTipos;
use crate::domain::repositories::Page;
use crate::infrastructure::error::AppError;

pub trait CierreRepository: Send + Sync {
    fn find_by_fecha(&self, fecha: &str) -> Result<Option<CierreWithTipos>, AppError>;
    fn find_page(&self, limit: i64, offset: i64) -> Result<Page<CierreWithTipos>, AppError>;
    fn delete_by_fecha(&self, fecha: &str) -> Result<(), AppError>;
}
