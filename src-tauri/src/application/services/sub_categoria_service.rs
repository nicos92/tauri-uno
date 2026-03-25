use std::sync::Arc;

use crate::domain::entities::SubCategoria;
use crate::domain::repositories::SubCategoriaRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteSubCategoriaRepository;

pub struct SubCategoriaService {
    repository: Arc<SqliteSubCategoriaRepository>,
}

impl SubCategoriaService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(SqliteSubCategoriaRepository::new()),
        }
    }

    pub fn create(
        &self,
        sub_categoria: String,
        id_categoria: i64,
    ) -> Result<SubCategoria, AppError> {
        let existing = self.repository.find_by_name(&sub_categoria)?;
        if existing.is_some() {
            return Err(AppError::SubCategoriaExists);
        }

        let new_sub_categoria = SubCategoria::new(sub_categoria, id_categoria);
        self.repository.create(&new_sub_categoria)
    }

    pub fn get_all(&self) -> Result<Vec<SubCategoria>, AppError> {
        self.repository.find_all()
    }

    pub fn get_by_categoria(&self, id_categoria: i64) -> Result<Vec<SubCategoria>, AppError> {
        self.repository.find_by_categoria(id_categoria)
    }

    pub fn get_by_id(&self, id: i64) -> Result<SubCategoria, AppError> {
        self.repository
            .find_by_id(id)?
            .ok_or(AppError::SubCategoriaNotFound)
    }

    pub fn update(
        &self,
        id: i64,
        sub_categoria: String,
        id_categoria: i64,
    ) -> Result<SubCategoria, AppError> {
        let mut existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::SubCategoriaNotFound)?;

        let name_exists = self.repository.find_by_name(&sub_categoria)?;
        if let Some(ref e) = name_exists {
            if e.id != id {
                return Err(AppError::SubCategoriaExists);
            }
        }

        existing.sub_categoria = sub_categoria;
        existing.id_categoria = id_categoria;
        self.repository.update(&existing)
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        let _existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::SubCategoriaNotFound)?;

        let has_articulos = self.repository.has_articulos(id)?;
        if has_articulos {
            return Err(AppError::SubCategoriaHasArticulos);
        }

        self.repository.delete(id)
    }
}
