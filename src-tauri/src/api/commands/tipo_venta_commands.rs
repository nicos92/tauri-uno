use std::sync::Mutex;
use tauri::State;

use crate::application::services::{log_audit, TipoVentaService};
use crate::domain::entities::{AuditAction, AuditScreen, PermissionCode, TipoVenta};
use crate::infrastructure::error::AppError;

pub struct TipoVentaAppState {
    pub tipo_venta_service: Mutex<TipoVentaService>,
}

impl TipoVentaAppState {
    pub fn new() -> Self {
        Self {
            tipo_venta_service: Mutex::new(TipoVentaService::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct TipoVentaRequest {
    pub nombre: String,
    pub hacia_donde: Option<String>,
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

#[tauri::command]
pub fn get_all_tipos_venta(
    user_id: i64,
    state: State<TipoVentaAppState>,
) -> Result<Vec<TipoVenta>, AppError> {
    let service = state
        .tipo_venta_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewTiposVenta)?;
    service.get_all()
}

#[tauri::command]
pub fn create_tipo_venta(
    user_id: i64,
    request: TipoVentaRequest,
    state: State<TipoVentaAppState>,
) -> Result<TipoVenta, AppError> {
    let service = state
        .tipo_venta_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::CreateTipoVenta)?;
    let result = service.create(request.nombre, request.hacia_donde)?;
    log_audit(
        user_id,
        AuditScreen::TiposVenta,
        AuditAction::Create,
        Some(format!("Tipo de venta: {} (id {})", result.nombre, result.id)),
    )?;
    Ok(result)
}

#[tauri::command]
pub fn update_tipo_venta(
    user_id: i64,
    id: i64,
    request: TipoVentaRequest,
    state: State<TipoVentaAppState>,
) -> Result<TipoVenta, AppError> {
    let service = state
        .tipo_venta_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::UpdateTipoVenta)?;
    let result = service.update(id, request.nombre, request.hacia_donde)?;
    log_audit(
        user_id,
        AuditScreen::TiposVenta,
        AuditAction::Update,
        Some(format!("Tipo de venta: {} (id {})", result.nombre, result.id)),
    )?;
    Ok(result)
}

#[tauri::command]
pub fn delete_tipo_venta(
    user_id: i64,
    id: i64,
    state: State<TipoVentaAppState>,
) -> Result<(), AppError> {
    let service = state
        .tipo_venta_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::DeleteTipoVenta)?;
    service.delete(id)?;
    log_audit(
        user_id,
        AuditScreen::TiposVenta,
        AuditAction::Delete,
        Some(format!("Tipo de venta (id {})", id)),
    )?;
    Ok(())
}
