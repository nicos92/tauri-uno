use std::sync::Mutex;
use tauri::State;

use crate::application::services::ArticuloService;
use crate::domain::entities::{Articulo, PermissionCode};
use crate::infrastructure::error::AppError;

pub struct ArticuloAppState {
    pub articulo_service: Mutex<ArticuloService>,
}

impl ArticuloAppState {
    pub fn new() -> Self {
        Self {
            articulo_service: Mutex::new(ArticuloService::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateArticuloRequest {
    pub articulo: String,
    pub cod_articulo: String,
    pub id_sub_categoria: i64,
    pub id_proveedor: i64,
}

#[derive(serde::Deserialize)]
pub struct UpdateArticuloRequest {
    pub id: i64,
    pub articulo: String,
    pub cod_articulo: String,
    pub id_sub_categoria: i64,
    pub id_proveedor: i64,
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
pub fn get_all_articulos(
    user_id: i64,
    state: State<ArticuloAppState>,
) -> Result<Vec<Articulo>, AppError> {
    let service = state
        .articulo_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewArticulos)?;
    service.get_all()
}

#[tauri::command]
pub fn create_articulo(
    user_id: i64,
    request: CreateArticuloRequest,
    state: State<ArticuloAppState>,
) -> Result<Articulo, AppError> {
    let service = state
        .articulo_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::CreateArticulo)?;
    service.create(
        request.articulo,
        request.cod_articulo,
        request.id_sub_categoria,
        request.id_proveedor,
    )
}

#[tauri::command]
pub fn update_articulo(
    user_id: i64,
    request: UpdateArticuloRequest,
    state: State<ArticuloAppState>,
) -> Result<Articulo, AppError> {
    let service = state
        .articulo_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::UpdateArticulo)?;
    service.update(
        request.id,
        request.articulo,
        request.cod_articulo,
        request.id_sub_categoria,
        request.id_proveedor,
    )
}

#[tauri::command]
pub fn delete_articulo(
    user_id: i64,
    id: i64,
    state: State<ArticuloAppState>,
) -> Result<(), AppError> {
    let service = state
        .articulo_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::DeleteArticulo)?;
    service.delete(id)
}
