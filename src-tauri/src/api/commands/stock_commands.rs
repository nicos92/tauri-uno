use std::sync::Mutex;
use tauri::State;

use crate::application::services::StockService;
use crate::domain::entities::{PermissionCode, Stock};
use crate::infrastructure::error::AppError;

pub struct StockAppState {
    pub stock_service: Mutex<StockService>,
}

impl StockAppState {
    pub fn new() -> Self {
        Self {
            stock_service: Mutex::new(StockService::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateStockRequest {
    pub id_articulo: i64,
    pub cantidad: f64,
    pub costo: f64,
    pub ganancia: f64,
}

#[derive(serde::Deserialize)]
pub struct UpdateStockRequest {
    pub id: i64,
    pub cantidad: f64,
    pub costo: f64,
    pub ganancia: f64,
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
pub fn get_all_stock(user_id: i64, state: State<StockAppState>) -> Result<Vec<Stock>, AppError> {
    let service = state
        .stock_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewStock)?;
    service.get_all()
}

#[tauri::command]
pub fn get_stock_by_id(
    user_id: i64,
    id: i64,
    state: State<StockAppState>,
) -> Result<Stock, AppError> {
    let service = state
        .stock_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewStock)?;
    service.get_by_id(id)
}

#[tauri::command]
pub fn get_stock_by_articulo(
    user_id: i64,
    id_articulo: i64,
    state: State<StockAppState>,
) -> Result<Option<Stock>, AppError> {
    let service = state
        .stock_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewStock)?;
    service.get_by_articulo(id_articulo)
}

#[tauri::command]
pub fn create_stock(
    user_id: i64,
    request: CreateStockRequest,
    state: State<StockAppState>,
) -> Result<Stock, AppError> {
    let service = state
        .stock_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::CreateStock)?;
    service.create(
        request.id_articulo,
        request.cantidad,
        request.costo,
        request.ganancia,
    )
}

#[tauri::command]
pub fn update_stock(
    user_id: i64,
    request: UpdateStockRequest,
    state: State<StockAppState>,
) -> Result<Stock, AppError> {
    let service = state
        .stock_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::UpdateStock)?;
    service.update(
        request.id,
        request.cantidad,
        request.costo,
        request.ganancia,
    )
}

#[tauri::command]
pub fn delete_stock(user_id: i64, id: i64, state: State<StockAppState>) -> Result<(), AppError> {
    let service = state
        .stock_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::DeleteStock)?;
    service.delete(id)
}

#[tauri::command]
pub fn get_precio_venta(
    user_id: i64,
    id: i64,
    state: State<StockAppState>,
) -> Result<f64, AppError> {
    let service = state
        .stock_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewStock)?;
    service.get_precio_venta(id)
}
