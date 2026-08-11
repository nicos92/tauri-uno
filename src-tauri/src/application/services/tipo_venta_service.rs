use std::sync::Arc;

use crate::domain::entities::TipoVenta;
use crate::domain::repositories::TipoVentaRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteTipoVentaRepository;

pub struct TipoVentaService {
    repository: Arc<SqliteTipoVentaRepository>,
}

impl Default for TipoVentaService {
    fn default() -> Self {
        Self::new()
    }
}

impl TipoVentaService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(SqliteTipoVentaRepository::new()),
        }
    }

    pub fn create(
        &self,
        nombre: String,
        hacia_donde: Option<String>,
    ) -> Result<TipoVenta, AppError> {
        let nombre = nombre.trim().to_string();
        if nombre.is_empty() {
            return Err(AppError::TipoVentaNombreInvalido);
        }

        let existing = self.repository.find_by_nombre(&nombre)?;
        if existing.is_some() {
            return Err(AppError::TipoVentaExists);
        }

        let new_tipo = TipoVenta::new(nombre, normalize_optional(hacia_donde));
        self.repository.create(&new_tipo)
    }

    pub fn get_all(&self) -> Result<Vec<TipoVenta>, AppError> {
        self.repository.find_all()
    }

    pub fn get_by_id(&self, id: i64) -> Result<TipoVenta, AppError> {
        self.repository
            .find_by_id(id)?
            .ok_or(AppError::TipoVentaNotFound)
    }

    pub fn update(
        &self,
        id: i64,
        nombre: String,
        hacia_donde: Option<String>,
    ) -> Result<TipoVenta, AppError> {
        let mut existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::TipoVentaNotFound)?;

        let nombre = nombre.trim().to_string();
        if nombre.is_empty() {
            return Err(AppError::TipoVentaNombreInvalido);
        }

        let name_exists = self.repository.find_by_nombre(&nombre)?;
        if let Some(ref e) = name_exists {
            if e.id != id {
                return Err(AppError::TipoVentaExists);
            }
        }

        existing.nombre = nombre;
        existing.hacia_donde = normalize_optional(hacia_donde);
        self.repository.update(&existing)
    }

    pub fn delete(&self, id: i64) -> Result<(), AppError> {
        let _existing = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::TipoVentaNotFound)?;

        let has_ventas = self.repository.has_ventas(id)?;
        if has_ventas {
            return Err(AppError::TipoVentaInUse);
        }

        self.repository.delete(id)
    }
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}
