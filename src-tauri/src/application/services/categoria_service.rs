use std::sync::Arc;

use crate::domain::entities::Categoria;
use crate::domain::repositories::CategoriaRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteCategoriaRepository;

pub struct CategoriaService {
    repository: Arc<SqliteCategoriaRepository>,
}

impl CategoriaService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(SqliteCategoriaRepository::new()),
        }
    }

    pub fn create(&self, categoria: String) -> Result<Categoria, AppError> {
        let existing = self.repository.find_by_name(&categoria)?;
        if existing.is_some() {
            return Err(AppError::CategoriaExists);
        }

        let new_categoria = Categoria::new(categoria);
        self.repository.create(&new_categoria)
    }

    pub fn get_all(&self) -> Result<Vec<Categoria>, AppError> {
        self.repository.find_all()
    }

    pub fn get_by_id(&self, id: i64) -> Result<Categoria, AppError> {
        self.repository
            .find_by_id(id)?
            .ok_or(AppError::CategoriaNotFound)
    }

    pub fn update(&self, id: i64, categoria: String) -> Result<Categoria, AppError> {
        let mut existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::CategoriaNotFound)?;

        let name_exists = self.repository.find_by_name(&categoria)?;
        if let Some(ref e) = name_exists {
            if e.id != id {
                return Err(AppError::CategoriaExists);
            }
        }

        existing.categoria = categoria;
        self.repository.update(&existing)
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        let _existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::CategoriaNotFound)?;

        let has_sub = self.repository.has_sub_categorias(id)?;
        if has_sub {
            return Err(AppError::CategoriaHasSubCategorias);
        }

        self.repository.delete(id)
    }
}
