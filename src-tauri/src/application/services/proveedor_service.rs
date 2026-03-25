use std::sync::Arc;

use crate::domain::entities::Proveedor;
use crate::domain::repositories::ProveedorRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteProveedorRepository;

pub struct ProveedorService {
    repository: Arc<SqliteProveedorRepository>,
}

impl ProveedorService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(SqliteProveedorRepository::new()),
        }
    }

    pub fn create(
        &self,
        proveedor: String,
        nombre: String,
        cuit: Option<String>,
        tel: Option<String>,
        email: Option<String>,
        observacion: Option<String>,
    ) -> Result<Proveedor, AppError> {
        if let Some(ref c) = cuit {
            if !c.is_empty() {
                let existing = self.repository.find_by_cuit(c)?;
                if existing.is_some() {
                    return Err(AppError::DuplicateCuit);
                }
            }
        }

        let new_proveedor = Proveedor::new(proveedor, nombre, cuit, tel, email, observacion);
        self.repository.create(&new_proveedor)
    }

    pub fn get_all(&self) -> Result<Vec<Proveedor>, AppError> {
        self.repository.find_all()
    }

    pub fn get_by_id(&self, id: i64) -> Result<Proveedor, AppError> {
        self.repository
            .find_by_id(id)?
            .ok_or(AppError::ProveedorNotFound)
    }

    pub fn update(
        &self,
        id: i64,
        proveedor: String,
        nombre: String,
        cuit: Option<String>,
        tel: Option<String>,
        email: Option<String>,
        observacion: Option<String>,
    ) -> Result<Proveedor, AppError> {
        let mut existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::ProveedorNotFound)?;

        if let Some(ref c) = cuit {
            if !c.is_empty() {
                let existing_cuit = self.repository.find_by_cuit(c)?;
                if let Some(ref ec) = existing_cuit {
                    if ec.id != id {
                        return Err(AppError::DuplicateCuit);
                    }
                }
            }
        }

        existing.proveedor = proveedor;
        existing.nombre = nombre;
        existing.cuit = cuit;
        existing.tel = tel;
        existing.email = email;
        existing.observacion = observacion;

        self.repository.update(&existing)
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        let _existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::ProveedorNotFound)?;

        self.repository.delete(id)
    }
}
