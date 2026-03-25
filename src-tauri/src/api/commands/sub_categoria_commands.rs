use std::sync::Mutex;
use tauri::State;

use crate::application::services::SubCategoriaService;
use crate::domain::entities::{PermissionCode, SubCategoria};
use crate::infrastructure::error::AppError;

pub struct SubCategoriaAppState {
    pub sub_categoria_service: Mutex<SubCategoriaService>,
}

impl SubCategoriaAppState {
    pub fn new() -> Self {
        Self {
            sub_categoria_service: Mutex::new(SubCategoriaService::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateSubCategoriaRequest {
    pub sub_categoria: String,
    pub id_categoria: i64,
}

#[derive(serde::Deserialize)]
pub struct UpdateSubCategoriaRequest {
    pub id: i64,
    pub sub_categoria: String,
    pub id_categoria: i64,
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
pub fn get_all_sub_categorias(
    user_id: i64,
    state: State<SubCategoriaAppState>,
) -> Result<Vec<SubCategoria>, AppError> {
    let service = state
        .sub_categoria_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewSubCategorias)?;
    service.get_all()
}

#[tauri::command]
pub fn get_sub_categorias_by_categoria(
    user_id: i64,
    id_categoria: i64,
    state: State<SubCategoriaAppState>,
) -> Result<Vec<SubCategoria>, AppError> {
    let service = state
        .sub_categoria_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewSubCategorias)?;
    service.get_by_categoria(id_categoria)
}

#[tauri::command]
pub fn create_sub_categoria(
    user_id: i64,
    request: CreateSubCategoriaRequest,
    state: State<SubCategoriaAppState>,
) -> Result<SubCategoria, AppError> {
    let service = state
        .sub_categoria_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::CreateSubCategoria)?;
    service.create(request.sub_categoria, request.id_categoria)
}

#[tauri::command]
pub fn update_sub_categoria(
    user_id: i64,
    request: UpdateSubCategoriaRequest,
    state: State<SubCategoriaAppState>,
) -> Result<SubCategoria, AppError> {
    let service = state
        .sub_categoria_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::UpdateSubCategoria)?;
    service.update(request.id, request.sub_categoria, request.id_categoria)
}

#[tauri::command]
pub fn delete_sub_categoria(
    user_id: i64,
    id: i64,
    state: State<SubCategoriaAppState>,
) -> Result<(), AppError> {
    let service = state
        .sub_categoria_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::DeleteSubCategoria)?;
    service.delete(id)
}
