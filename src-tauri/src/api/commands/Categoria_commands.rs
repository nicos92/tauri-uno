use std::sync::Mutex;
use tauri::State;

use crate::application::services::CategoriaService;
use crate::domain::entities::{Categoria, PermissionCode};
use crate::infrastructure::error::AppError;

pub struct CategoriaAppState {
    pub categoria_service: Mutex<CategoriaService>,
}

impl CategoriaAppState {
    pub fn new() -> Self {
        Self {
            categoria_service: Mutex::new(CategoriaService::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateCategoriaRequest {
    pub categoria: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateCategoriaRequest {
    pub id: i64,
    pub categoria: String,
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
pub fn get_all_categorias(
    user_id: i64,
    state: State<CategoriaAppState>,
) -> Result<Vec<Categoria>, AppError> {
    let service = state
        .categoria_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewCategorias)?;
    service.get_all()
}

#[tauri::command]
pub fn create_categoria(
    user_id: i64,
    request: CreateCategoriaRequest,
    state: State<CategoriaAppState>,
) -> Result<Categoria, AppError> {
    let service = state
        .categoria_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::CreateCategoria)?;
    service.create(request.categoria)
}

#[tauri::command]
pub fn update_categoria(
    user_id: i64,
    request: UpdateCategoriaRequest,
    state: State<CategoriaAppState>,
) -> Result<Categoria, AppError> {
    let service = state
        .categoria_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::UpdateCategoria)?;
    service.update(request.id, request.categoria)
}

#[tauri::command]
pub fn delete_categoria(
    user_id: i64,
    id: i64,
    state: State<CategoriaAppState>,
) -> Result<(), AppError> {
    let service = state
        .categoria_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::DeleteCategoria)?;
    service.delete(id)
}
