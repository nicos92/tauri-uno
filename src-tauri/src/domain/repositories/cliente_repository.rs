use crate::domain::entities::Cliente;
use crate::infrastructure::error::AppError;

#[cfg_attr(test, mockall::automock)]
pub trait ClienteRepository: Send + Sync {
    fn create(&self, cliente: &Cliente) -> Result<Cliente, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<Cliente>, AppError>;
    fn find_default(&self) -> Result<Option<Cliente>, AppError>;
    fn find_all(&self) -> Result<Vec<Cliente>, AppError>;
    fn update(&self, cliente: &Cliente) -> Result<Cliente, AppError>;
    fn delete(&self, id: i64) -> Result<(), AppError>;
}
