use std::sync::Mutex;
use tauri::State;

use crate::application::services::{log_audit, CierreService};
use crate::domain::entities::{
    AuditAction, AuditScreen, CierreWithTipos, PermissionCode,
};
use crate::infrastructure::error::AppError;

pub struct CierreAppState {
    pub cierre_service: Mutex<CierreService>,
}

impl CierreAppState {
    pub fn new() -> Self {
        Self {
            cierre_service: Mutex::new(CierreService::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CrearCierreRequest {
    pub fecha: String,
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
pub fn crear_cierre(
    user_id: i64,
    request: CrearCierreRequest,
    state: State<CierreAppState>,
) -> Result<CierreWithTipos, AppError> {
    let service = state
        .cierre_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::CreateCierre)?;
    let cierre = service.crear_cierre(&request.fecha)?;
    log_audit(
        user_id,
        AuditScreen::Cierres,
        AuditAction::Create,
        Some(format!(
            "Cierre del día: {} (id {})",
            cierre.cierre.fecha, cierre.cierre.id
        )),
    )?;
    Ok(cierre)
}

#[tauri::command]
pub fn get_all_cierres(
    user_id: i64,
    state: State<CierreAppState>,
) -> Result<Vec<CierreWithTipos>, AppError> {
    let service = state
        .cierre_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewCierres)?;
    service.get_all()
}
