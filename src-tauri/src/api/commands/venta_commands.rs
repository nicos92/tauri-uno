use std::sync::Mutex;
use tauri::State;

use crate::application::services::{log_audit, VentaService};
use crate::domain::entities::{
    AuditAction, AuditScreen, PermissionCode, VentaDetalle, VentaWithDetalle,
};
use crate::infrastructure::error::AppError;

pub struct VentaAppState {
    pub venta_service: Mutex<VentaService>,
}

impl VentaAppState {
    pub fn new() -> Self {
        Self {
            venta_service: Mutex::new(VentaService::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateVentaDetalleRequest {
    pub id_articulo: i64,
    pub cantidad: f64,
    pub precio_unitario: Option<f64>,
}

#[derive(serde::Deserialize)]
pub struct CreateVentaRequest {
    pub items: Vec<CreateVentaDetalleRequest>,
    pub observacion: Option<String>,
}

fn check_permission(user_id: i64, permission: PermissionCode) -> Result<(), AppError> {
    let conn = crate::infrastructure::database::DB
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_permissions up
         INNER JOIN permissions p ON up.permission_id = p.id
         WHERE up.user_id = ?1 AND p.permission = ?2",
        rusqlite::params![user_id, permission.as_str()],
        |row| row.get(0),
    )?;

    if count == 0 {
        return Err(AppError::PermissionDenied);
    }
    Ok(())
}

fn can_sell_without_stock(user_id: i64) -> Result<bool, AppError> {
    match check_permission(user_id, PermissionCode::VenderSinStock) {
        Ok(()) => Ok(true),
        Err(AppError::PermissionDenied) => Ok(false),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub fn create_venta(
    user_id: i64,
    request: CreateVentaRequest,
    state: State<VentaAppState>,
) -> Result<VentaWithDetalle, AppError> {
    let service = state
        .venta_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::CreateVenta)?;
    let allow_negative_stock = can_sell_without_stock(user_id)?;

    let detalles: Vec<VentaDetalle> = request
        .items
        .into_iter()
        .map(|item| {
            VentaDetalle::new(
                item.id_articulo,
                item.cantidad,
                0.0,
                item.precio_unitario.unwrap_or(0.0),
            )
        })
        .collect();

    let venta = service.create(user_id, detalles, request.observacion, allow_negative_stock)?;
    log_audit(
        user_id,
        AuditScreen::Ventas,
        AuditAction::Create,
        Some(format!("Venta (id {})", venta.id)),
    )?;
    Ok(venta)
}

#[tauri::command]
pub fn get_all_ventas(
    user_id: i64,
    state: State<VentaAppState>,
) -> Result<Vec<VentaWithDetalle>, AppError> {
    let service = state
        .venta_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewVentas)?;
    service.get_all()
}

#[tauri::command]
pub fn get_venta_by_id(
    user_id: i64,
    id: i64,
    state: State<VentaAppState>,
) -> Result<VentaWithDetalle, AppError> {
    let service = state
        .venta_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewVentas)?;
    service.get_by_id(id)
}

#[tauri::command]
pub fn anular_venta(user_id: i64, id: i64, state: State<VentaAppState>) -> Result<(), AppError> {
    let service = state
        .venta_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::AnularVenta)?;
    service.anular(id)?;
    log_audit(
        user_id,
        AuditScreen::Ventas,
        AuditAction::Update,
        Some(format!("Venta (id {}) anulada", id)),
    )?;
    Ok(())
}
