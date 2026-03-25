use crate::domain::entities::SubCategoria;
use crate::infrastructure::error::AppError;

pub trait SubCategoriaRepository: Send + Sync {
    fn create(&self, sub_categoria: &SubCategoria) -> Result<SubCategoria, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<SubCategoria>, AppError>;
    fn find_by_name(&self, sub_categoria: &str) -> Result<Option<SubCategoria>, AppError>;
    fn find_all(&self) -> Result<Vec<SubCategoria>, AppError>;
    fn find_by_categoria(&self, id_categoria: i64) -> Result<Vec<SubCategoria>, AppError>;
    fn update(&self, sub_categoria: &SubCategoria) -> Result<SubCategoria, AppError>;
    fn delete(&self, id: i64) -> Result<(), AppError>;
    fn has_articulos(&self, id: i64) -> Result<bool, AppError>;
}
