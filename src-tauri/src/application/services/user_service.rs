use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;

use crate::domain::entities::{Permission, User};
use crate::domain::repositories::UserRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteUserRepository;

pub struct UserService {
    repository: Arc<SqliteUserRepository>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(SqliteUserRepository::new()),
        }
    }

    pub fn create_user(&self, username: String, password: String) -> Result<User, AppError> {
        let existing = self.repository.find_by_username(&username)?;
        if existing.is_some() {
            return Err(AppError::UsernameExists);
        }

        let hashed_password =
            hash(&password, DEFAULT_COST).map_err(|e| AppError::Hashing(e.to_string()))?;

        let user = User::new(username, hashed_password);
        self.repository.create(&user)
    }

    pub fn login(&self, username: String, password: String) -> Result<User, AppError> {
        let user = self
            .repository
            .find_by_username(&username)?
            .ok_or(AppError::InvalidCredentials)?;

        if !user.active {
            return Err(AppError::UserInactive);
        }

        let is_valid =
            verify(&password, &user.password).map_err(|e| AppError::Hashing(e.to_string()))?;

        if !is_valid {
            return Err(AppError::InvalidCredentials);
        }

        Ok(user)
    }

    pub fn get_user(&self, id: i64) -> Result<User, AppError> {
        self.repository
            .find_by_id(id)?
            .ok_or(AppError::UserNotFound)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        self.repository.find_all()
    }

    pub fn update_user(&self, id: i64, username: String, active: bool) -> Result<User, AppError> {
        let mut user = self
            .repository
            .find_by_id(id)?
            .ok_or(AppError::UserNotFound)?;

        let existing = self.repository.find_by_username(&username)?;
        if existing.is_some() && existing.unwrap().id != id {
            return Err(AppError::UsernameExists);
        }

        user.username = username;
        user.active = active;

        self.repository.update(&user)
    }

    pub fn delete_user(&self, id: i64) -> Result<(), AppError> {
        self.repository.delete(id)
    }

    pub fn add_permission_to_user(&self, user_id: i64, permission_id: i64) -> Result<(), AppError> {
        let _user = self
            .repository
            .find_by_id(user_id)?
            .ok_or(AppError::UserNotFound)?;

        let permissions = self.repository.get_all_permissions()?;
        if !permissions.iter().any(|p| p.id == permission_id) {
            return Err(AppError::PermissionNotFound);
        }

        self.repository.add_permission(user_id, permission_id)
    }

    pub fn remove_permission_from_user(
        &self,
        user_id: i64,
        permission_id: i64,
    ) -> Result<(), AppError> {
        self.repository.remove_permission(user_id, permission_id)
    }

    pub fn get_user_permissions(&self, user_id: i64) -> Result<Vec<Permission>, AppError> {
        self.repository.get_user_permissions(user_id)
    }

    pub fn get_all_permissions(&self) -> Result<Vec<Permission>, AppError> {
        self.repository.get_all_permissions()
    }

    pub fn create_permission(&self, permission_name: String) -> Result<Permission, AppError> {
        let permissions = self.repository.get_all_permissions()?;
        if permissions.iter().any(|p| p.permission == permission_name) {
            return Err(AppError::PermissionExists);
        }

        let permission = Permission::new(permission_name);

        let conn = crate::infrastructure::database::DB
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO permissions (permission, created) VALUES (?1, ?2)",
            rusqlite::params![permission.permission, permission.created.to_rfc3339()],
        )?;

        let id = conn.last_insert_rowid();
        Ok(Permission { id, ..permission })
    }
}
