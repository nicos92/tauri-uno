use chrono::{DateTime, Utc};
use rusqlite::params;

use crate::domain::entities::{Permission, User};
use crate::domain::repositories::UserRepository;
use crate::infrastructure::database::DB;
use crate::infrastructure::error::AppError;

pub struct SqliteUserRepository;

impl SqliteUserRepository {
    pub fn new() -> Self {
        Self
    }
}

impl UserRepository for SqliteUserRepository {
    fn create(&self, user: &User) -> Result<User, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO users (username, password, active, created_at, modified_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                user.username,
                user.password,
                user.active as i32,
                user.created_at.to_rfc3339(),
                user.modified_at.to_rfc3339()
            ],
        )?;

        let id = conn.last_insert_rowid();
        Ok(User { id, ..user.clone() })
    }

    fn find_by_id(&self, id: i64) -> Result<Option<User>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, username, password, active, created_at, modified_at FROM users WHERE id = ?1"
        )?;

        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_user(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, username, password, active, created_at, modified_at FROM users WHERE username = ?1"
        )?;

        let mut rows = stmt.query(params![username])?;

        if let Some(row) = rows.next()? {
            Ok(Some(self.row_to_user(row)?))
        } else {
            Ok(None)
        }
    }

    fn find_all(&self) -> Result<Vec<User>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn
            .prepare("SELECT id, username, password, active, created_at, modified_at FROM users")?;

        let mut users = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            users.push(self.row_to_user(row)?);
        }

        Ok(users)
    }

    fn update(&self, user: &User) -> Result<User, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "UPDATE users SET username = ?1, password = ?2, active = ?3, modified_at = ?4 WHERE id = ?5",
            params![
                user.username,
                user.password,
                user.active as i32,
                Utc::now().to_rfc3339(),
                user.id
            ],
        )?;

        Ok(user.clone())
    }

    fn delete(&self, id: i64) -> Result<(), AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;
        conn.execute(
            "DELETE FROM user_permissions WHERE user_id = ?1",
            params![id],
        )?;
        conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
        Ok(())
    }

    fn add_permission(&self, user_id: i64, permission_id: i64) -> Result<(), AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT OR IGNORE INTO user_permissions (user_id, permission_id) VALUES (?1, ?2)",
            params![user_id, permission_id],
        )?;

        Ok(())
    }

    fn remove_permission(&self, user_id: i64, permission_id: i64) -> Result<(), AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "DELETE FROM user_permissions WHERE user_id = ?1 AND permission_id = ?2",
            params![user_id, permission_id],
        )?;

        Ok(())
    }

    fn get_user_permissions(&self, user_id: i64) -> Result<Vec<Permission>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT p.id, p.permission, p.created 
             FROM permissions p 
             INNER JOIN user_permissions up ON p.id = up.permission_id 
             WHERE up.user_id = ?1",
        )?;

        let mut permissions = Vec::new();
        let mut rows = stmt.query(params![user_id])?;

        while let Some(row) = rows.next()? {
            permissions.push(Permission {
                id: row.get(0)?,
                permission: row.get(1)?,
                created: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            });
        }

        Ok(permissions)
    }

    fn get_all_permissions(&self) -> Result<Vec<Permission>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare("SELECT id, permission, created FROM permissions")?;

        let mut permissions = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            permissions.push(Permission {
                id: row.get(0)?,
                permission: row.get(1)?,
                created: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            });
        }

        Ok(permissions)
    }
}

impl SqliteUserRepository {
    fn row_to_user(&self, row: &rusqlite::Row) -> Result<User, AppError> {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            active: row.get::<_, i32>(3)? != 0,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            modified_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        })
    }
}
