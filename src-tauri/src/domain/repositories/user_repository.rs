use crate::domain::entities::{Permission, User, UserPermission};
use crate::infrastructure::error::AppError;

pub trait UserRepository: Send + Sync {
    fn create(&self, user: &User) -> Result<User, AppError>;
    fn find_by_id(&self, id: i64) -> Result<Option<User>, AppError>;
    fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
    fn find_all(&self) -> Result<Vec<User>, AppError>;
    fn update(&self, user: &User) -> Result<User, AppError>;
    fn delete(&self, id: i64) -> Result<(), AppError>;
    fn add_permission(&self, user_id: i64, permission_id: i64) -> Result<(), AppError>;
    fn remove_permission(&self, user_id: i64, permission_id: i64) -> Result<(), AppError>;
    fn get_user_permissions(&self, user_id: i64) -> Result<Vec<UserPermission>, AppError>;
    fn get_all_permissions(&self) -> Result<Vec<Permission>, AppError>;
    fn has_permission(&self, user_id: i64, permission_name: &str) -> Result<bool, AppError>;
    fn get_user_permissions_by_names(&self, user_id: i64) -> Result<Vec<String>, AppError>;
}
