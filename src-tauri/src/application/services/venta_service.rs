use std::sync::Arc;

use crate::domain::entities::{Venta, VentaDetalle, VentaWithDetalle};
use crate::domain::repositories::VentaRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteVentaRepository;

pub struct VentaService {
    repository: Arc<SqliteVentaRepository>,
}

impl VentaService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(SqliteVentaRepository::new()),
        }
    }

    pub fn create(
        &self,
        user_id: i64,
        detalles: Vec<VentaDetalle>,
        descuento: f64,
        observacion: Option<String>,
        id_tipo_venta: Option<i64>,
        allow_negative_stock: bool,
    ) -> Result<VentaWithDetalle, AppError> {
        if !(0.0..=100.0).contains(&descuento) {
            return Err(AppError::DescuentoInvalido);
        }

        let mut venta = Venta::new(
            user_id,
            chrono::Utc::now().to_rfc3339(),
            descuento,
            observacion,
        );
        venta.id_tipo_venta = id_tipo_venta;
        self.repository.create(&venta, &detalles, allow_negative_stock)
    }

    pub fn get_all(&self) -> Result<Vec<VentaWithDetalle>, AppError> {
        self.repository.find_all()
    }

    pub fn get_by_id(&self, id: i64) -> Result<VentaWithDetalle, AppError> {
        self.repository
            .find_by_id(id)?
            .ok_or(AppError::VentaNotFound)
    }

    pub fn anular(&self, id: i64) -> Result<(), AppError> {
        self.repository.anular(id)
    }
}
