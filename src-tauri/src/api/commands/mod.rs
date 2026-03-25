use std::sync::Mutex;
use tauri::State;

use crate::application::services::UserService;
use crate::domain::entities::{Permission, User};
use crate::infrastructure::error::AppError;

pub struct AppState {
    pub user_service: Mutex<UserService>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            user_service: Mutex::new(UserService::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateUserRequest {
    pub id: i64,
    pub username: String,
    pub active: bool,
}

#[derive(serde::Deserialize)]
pub struct AddPermissionRequest {
    pub user_id: i64,
    pub permission_id: i64,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub active: bool,
    pub created_at: String,
    pub modified_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            active: user.active,
            created_at: user.created_at.to_rfc3339(),
            modified_at: user.modified_at.to_rfc3339(),
        }
    }
}

#[tauri::command]
pub fn login(request: LoginRequest, state: State<AppState>) -> Result<UserResponse, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let user = service.login(request.username, request.password)?;
    Ok(user.into())
}

#[tauri::command]
pub fn create_user(
    request: CreateUserRequest,
    state: State<AppState>,
) -> Result<UserResponse, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let user = service.create_user(request.username, request.password)?;
    Ok(user.into())
}

#[tauri::command]
pub fn get_all_users(state: State<AppState>) -> Result<Vec<UserResponse>, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let users = service.get_all_users()?;
    Ok(users.into_iter().map(|u| u.into()).collect())
}

#[tauri::command]
pub fn update_user(
    request: UpdateUserRequest,
    state: State<AppState>,
) -> Result<UserResponse, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let user = service.update_user(request.id, request.username, request.active)?;
    Ok(user.into())
}

#[tauri::command]
pub fn delete_user(id: i64, state: State<AppState>) -> Result<(), AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    service.delete_user(id)
}

#[tauri::command]
pub fn add_permission_to_user(
    request: AddPermissionRequest,
    state: State<AppState>,
) -> Result<(), AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    service.add_permission_to_user(request.user_id, request.permission_id)
}

#[tauri::command]
pub fn remove_permission_from_user(
    request: AddPermissionRequest,
    state: State<AppState>,
) -> Result<(), AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    service.remove_permission_from_user(request.user_id, request.permission_id)
}

#[tauri::command]
pub fn get_user_permissions(
    user_id: i64,
    state: State<AppState>,
) -> Result<Vec<Permission>, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    service.get_user_permissions(user_id)
}

#[tauri::command]
pub fn get_all_permissions(state: State<AppState>) -> Result<Vec<Permission>, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    service.get_all_permissions()
}

#[tauri::command]
pub fn create_permission(name: String, state: State<AppState>) -> Result<Permission, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    service.create_permission(name)
}
