use std::sync::Arc;

use crate::domain::entities::{AuditAction, AuditLog, AuditScreen};
use crate::domain::repositories::{AuditLogFilter, AuditLogRepository, Page};
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteAuditLogRepository;

pub struct AuditLogService {
    repository: Arc<SqliteAuditLogRepository>,
}

impl AuditLogService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(SqliteAuditLogRepository::new()),
        }
    }

    pub fn log(
        &self,
        user_id: i64,
        screen: AuditScreen,
        action: AuditAction,
        detail: Option<String>,
    ) -> Result<AuditLog, AppError> {
        let username = lookup_username(user_id)?;
        let log = AuditLog::new(user_id, username, screen, action, detail);
        self.repository.create(&log)
    }

    pub fn get_logs(&self, filter: &AuditLogFilter) -> Result<Page<AuditLog>, AppError> {
        self.repository.find_with_filters(filter)
    }
}

fn lookup_username(user_id: i64) -> Result<String, AppError> {
    let conn = crate::infrastructure::database::DB
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let username: Option<String> = conn
        .query_row(
            "SELECT username FROM users WHERE id = ?1",
            rusqlite::params![user_id],
            |row| row.get(0),
        )
        .ok();

    Ok(username.unwrap_or_default())
}

pub fn log_audit(
    user_id: i64,
    screen: AuditScreen,
    action: AuditAction,
    detail: Option<String>,
) -> Result<AuditLog, AppError> {
    let service = AuditLogService::new();
    service.log(user_id, screen, action, detail)
}
