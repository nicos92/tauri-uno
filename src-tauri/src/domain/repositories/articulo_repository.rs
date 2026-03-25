use crate::domain::entities::Articulo;
use crate::infrastructure::error::AppError;

pub trait ArticuloRepository: Send + Sync {
    fn create(&self, articulo: &Articulo) -> Result<Articulo, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<Articulo>, AppError>;
    fn find_by_codigo(&self, cod_articulo: &str) -> Result<Option<Articulo>, AppError>;
    fn find_all(&self) -> Result<Vec<Articulo>, AppError>;
    fn update(&self, articulo: &Articulo) -> Result<Articulo, AppError>;
    fn delete(&self, id: i64) -> Result<(), AppError>;
}
