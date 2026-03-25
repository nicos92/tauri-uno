use std::sync::Mutex;
use tauri::State;

use crate::application::services::ProveedorService;
use crate::domain::entities::{PermissionCode, Proveedor};
use crate::infrastructure::error::AppError;

pub struct ProveedorAppState {
    pub proveedor_service: Mutex<ProveedorService>,
}

impl ProveedorAppState {
    pub fn new() -> Self {
        Self {
            proveedor_service: Mutex::new(ProveedorService::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateProveedorRequest {
    pub proveedor: String,
    pub nombre: String,
    pub cuit: Option<String>,
    pub tel: Option<String>,
    pub email: Option<String>,
    pub observacion: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct UpdateProveedorRequest {
    pub id: i64,
    pub proveedor: String,
    pub nombre: String,
    pub cuit: Option<String>,
    pub tel: Option<String>,
    pub email: Option<String>,
    pub observacion: Option<String>,
}

fn check_permission(
    _service: &ProveedorService,
    user_id: i64,
    permission: PermissionCode,
) -> Result<(), AppError> {
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
pub fn get_all_proveedores(
    user_id: i64,
    state: State<ProveedorAppState>,
) -> Result<Vec<Proveedor>, AppError> {
    let service = state
        .proveedor_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&*service, user_id, PermissionCode::ViewProveedores)?;
    service.get_all()
}

#[tauri::command]
pub fn get_proveedor_by_id(
    user_id: i64,
    id: i64,
    state: State<ProveedorAppState>,
) -> Result<Proveedor, AppError> {
    let service = state
        .proveedor_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&*service, user_id, PermissionCode::ViewProveedores)?;
    service.get_by_id(id)
}

#[tauri::command]
pub fn create_proveedor(
    user_id: i64,
    request: CreateProveedorRequest,
    state: State<ProveedorAppState>,
) -> Result<Proveedor, AppError> {
    let service = state
        .proveedor_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&*service, user_id, PermissionCode::CreateProveedor)?;
    service.create(
        request.proveedor,
        request.nombre,
        request.cuit,
        request.tel,
        request.email,
        request.observacion,
    )
}

#[tauri::command]
pub fn update_proveedor(
    user_id: i64,
    request: UpdateProveedorRequest,
    state: State<ProveedorAppState>,
) -> Result<Proveedor, AppError> {
    let service = state
        .proveedor_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&*service, user_id, PermissionCode::UpdateProveedor)?;
    service.update(
        request.id,
        request.proveedor,
        request.nombre,
        request.cuit,
        request.tel,
        request.email,
        request.observacion,
    )
}

#[tauri::command]
pub fn delete_proveedor(
    user_id: i64,
    id: i64,
    state: State<ProveedorAppState>,
) -> Result<(), AppError> {
    let service = state
        .proveedor_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&*service, user_id, PermissionCode::DeleteProveedor)?;
    service.delete(id)
}
