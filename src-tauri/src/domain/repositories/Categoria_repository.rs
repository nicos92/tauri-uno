use crate::domain::entities::Categoria;
use crate::infrastructure::error::AppError;

pub trait CategoriaRepository: Send + Sync {
    fn create(&self, categoria: &Categoria) -> Result<Categoria, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<Categoria>, AppError>;
    fn find_by_name(&self, categoria: &str) -> Result<Option<Categoria>, AppError>;
    fn find_all(&self) -> Result<Vec<Categoria>, AppError>;
    fn update(&self, categoria: &Categoria) -> Result<Categoria, AppError>;
    fn delete(&self, id: i64) -> Result<(), AppError>;
    fn has_sub_categorias(&self, id: i64) -> Result<bool, AppError>;
}
