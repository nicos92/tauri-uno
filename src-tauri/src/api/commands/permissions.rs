use crate::domain::entities::PermissionCode;
use crate::infrastructure::error::AppError;

pub fn check_permission(user_id: i64, permission: PermissionCode) -> Result<(), AppError> {
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
