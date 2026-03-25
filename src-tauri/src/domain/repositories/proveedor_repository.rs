use crate::domain::entities::Proveedor;
use crate::infrastructure::error::AppError;

pub trait ProveedorRepository: Send + Sync {
    fn create(&self, proveedor: &Proveedor) -> Result<Proveedor, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<Proveedor>, AppError>;
    fn find_by_cuit(&self, cuit: &str) -> Result<Option<Proveedor>, AppError>;
    fn find_all(&self) -> Result<Vec<Proveedor>, AppError>;
    fn update(&self, proveedor: &Proveedor) -> Result<Proveedor, AppError>;
    fn delete(&self, id: i64) -> Result<(), AppError>;
}
