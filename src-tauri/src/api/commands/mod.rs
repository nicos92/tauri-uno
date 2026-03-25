use std::sync::Mutex;
use tauri::State;

use crate::application::services::UserService;
use crate::domain::entities::{Permission, PermissionCode, User, UserPermission};
use crate::infrastructure::error::AppError;

pub mod articulo_commands;
pub mod categoria_commands;
pub mod proveedor_commands;
pub mod sub_categoria_commands;

pub use articulo_commands::{
    create_articulo, delete_articulo, get_all_articulos, update_articulo, ArticuloAppState,
};
pub use categoria_commands::{
    create_categoria, delete_categoria, get_all_categorias, update_categoria, CategoriaAppState,
};
pub use proveedor_commands::{
    create_proveedor, delete_proveedor, get_all_proveedores, get_proveedor_by_id, update_proveedor,
    ProveedorAppState,
};
pub use sub_categoria_commands::{
    create_sub_categoria, delete_sub_categoria, get_all_sub_categorias,
    get_sub_categorias_by_categoria, update_sub_categoria, SubCategoriaAppState,
};

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
    pub permissions: Vec<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            active: user.active,
            created_at: user.created_at.to_rfc3339(),
            modified_at: user.modified_at.to_rfc3339(),
            permissions: Vec::new(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub permissions: Vec<String>,
}

fn check_permission(
    service: &UserService,
    user_id: i64,
    permission: PermissionCode,
) -> Result<(), AppError> {
    let has_perm = service
        .has_permission(user_id, permission.as_str())
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if !has_perm {
        return Err(AppError::PermissionDenied);
    }
    Ok(())
}

#[tauri::command]
pub fn login(request: LoginRequest, state: State<AppState>) -> Result<LoginResponse, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let user = service.login(request.username, request.password)?;
    let permissions = service
        .get_user_permissions_by_names(user.id)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(LoginResponse {
        user: user.into(),
        permissions,
    })
}

#[tauri::command]
pub fn create_user(
    user_id: i64,
    request: CreateUserRequest,
    state: State<AppState>,
) -> Result<UserResponse, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&service, user_id, PermissionCode::CreateUser)?;
    let user = service.create_user(request.username, request.password)?;
    Ok(user.into())
}

#[tauri::command]
pub fn get_all_users(user_id: i64, state: State<AppState>) -> Result<Vec<UserResponse>, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&service, user_id, PermissionCode::ViewUsers)?;
    let users = service.get_all_users()?;
    Ok(users.into_iter().map(|u| u.into()).collect())
}

#[tauri::command]
pub fn update_user(
    user_id: i64,
    request: UpdateUserRequest,
    state: State<AppState>,
) -> Result<UserResponse, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&service, user_id, PermissionCode::UpdateUser)?;
    let user = service.update_user(request.id, request.username, request.active)?;
    Ok(user.into())
}

#[tauri::command]
pub fn delete_user(user_id: i64, id: i64, state: State<AppState>) -> Result<(), AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&service, user_id, PermissionCode::DeleteUser)?;
    service.delete_user(id)
}

#[tauri::command]
pub fn add_permission_to_user(
    user_id: i64,
    request: AddPermissionRequest,
    state: State<AppState>,
) -> Result<(), AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&service, user_id, PermissionCode::AssignPermission)?;
    service.add_permission_to_user(request.user_id, request.permission_id)
}

#[tauri::command]
pub fn remove_permission_from_user(
    user_id: i64,
    request: AddPermissionRequest,
    state: State<AppState>,
) -> Result<(), AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&service, user_id, PermissionCode::RemovePermission)?;
    service.remove_permission_from_user(request.user_id, request.permission_id)
}

#[tauri::command]
pub fn get_user_permissions(
    user_id: i64,
    target_user_id: i64,
    state: State<AppState>,
) -> Result<Vec<UserPermission>, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&service, user_id, PermissionCode::ViewPermissions)?;
    service.get_user_permissions(target_user_id)
}

#[tauri::command]
pub fn get_all_permissions(
    user_id: i64,
    state: State<AppState>,
) -> Result<Vec<Permission>, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&service, user_id, PermissionCode::ViewPermissions)?;
    service.get_all_permissions()
}

#[tauri::command]
pub fn create_permission(
    user_id: i64,
    name: String,
    state: State<AppState>,
) -> Result<Permission, AppError> {
    let service = state
        .user_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(&service, user_id, PermissionCode::CreateCategoria)?;
    service.create_permission(name)
}
