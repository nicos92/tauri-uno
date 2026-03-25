use std::sync::Arc;

use crate::domain::entities::Articulo;
use crate::domain::repositories::ArticuloRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteArticuloRepository;

pub struct ArticuloService {
    repository: Arc<SqliteArticuloRepository>,
}

impl ArticuloService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(SqliteArticuloRepository::new()),
        }
    }

    pub fn create(
        &self,
        articulo: String,
        cod_articulo: String,
        id_sub_categoria: i64,
        id_proveedor: i64,
    ) -> Result<Articulo, AppError> {
        let existing = self.repository.find_by_codigo(&cod_articulo)?;
        if existing.is_some() {
            return Err(AppError::CodArticuloExists);
        }

        let new_articulo = Articulo::new(articulo, cod_articulo, id_sub_categoria, id_proveedor);
        self.repository.create(&new_articulo)
    }

    pub fn get_all(&self) -> Result<Vec<Articulo>, AppError> {
        self.repository.find_all()
    }

    pub fn get_by_id(&self, id: i64) -> Result<Articulo, AppError> {
        self.repository
            .find_by_id(id)?
            .ok_or(AppError::ArticuloNotFound)
    }

    pub fn update(
        &self,
        id: i64,
        articulo: String,
        cod_articulo: String,
        id_sub_categoria: i64,
        id_proveedor: i64,
    ) -> Result<Articulo, AppError> {
        let mut existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::ArticuloNotFound)?;

        let cod_exists = self.repository.find_by_codigo(&cod_articulo)?;
        if let Some(ref e) = cod_exists {
            if e.id != id {
                return Err(AppError::CodArticuloExists);
            }
        }

        existing.articulo = articulo;
        existing.cod_articulo = cod_articulo;
        existing.id_sub_categoria = id_sub_categoria;
        existing.id_proveedor = id_proveedor;

        self.repository.update(&existing)
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        let _existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::ArticuloNotFound)?;

        self.repository.delete(id)
    }
}
