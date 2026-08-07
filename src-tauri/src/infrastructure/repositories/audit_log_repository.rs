use rusqlite::params;

use crate::domain::entities::AuditLog;
use crate::domain::repositories::{AuditLogFilter, AuditLogRepository};
use crate::infrastructure::database::DB;
use crate::infrastructure::error::AppError;

pub struct SqliteAuditLogRepository;

impl SqliteAuditLogRepository {
    pub fn new() -> Self {
        Self
    }
}

impl AuditLogRepository for SqliteAuditLogRepository {
    fn create(&self, log: &AuditLog) -> Result<AuditLog, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO audit_logs (user_id, username, screen, action, detail, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                log.user_id,
                log.username,
                log.screen,
                log.action,
                log.detail,
                log.created_at.to_rfc3339(),
            ],
        )?;

        let id = conn.last_insert_rowid();
        Ok(AuditLog {
            id,
            ..log.clone()
        })
    }

    fn find_with_filters(&self, filter: &AuditLogFilter) -> Result<Vec<AuditLog>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut sql = String::from(
            "SELECT id, user_id, username, screen, action, detail, created_at
             FROM audit_logs WHERE 1 = 1",
        );
        let mut conditions: Vec<String> = Vec::new();

        if let Some(user_id) = filter.user_id {
            conditions.push(format!("user_id = {}", user_id));
        }
        if let Some(screen) = &filter.screen {
            conditions.push(format!("screen = '{}'", screen.replace('\'', "''")));
        }
        if let Some(action) = &filter.action {
            conditions.push(format!("action = '{}'", action.replace('\'', "''")));
        }
        if let Some(from) = &filter.from {
            conditions.push(format!(
                "datetime(created_at) >= datetime('{}')",
                from.replace('\'', "''")
            ));
        }
        if let Some(to) = &filter.to {
            conditions.push(format!(
                "datetime(created_at) <= datetime('{}')",
                to.replace('\'', "''")
            ));
        }

        if !conditions.is_empty() {
            sql.push_str(" AND ");
            sql.push_str(&conditions.join(" AND "));
        }

        sql.push_str(" ORDER BY created_at DESC, id DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit.max(1)));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset.max(0)));
        }

        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query([])?;

        let mut logs = Vec::new();
        while let Some(row) = rows.next()? {
            logs.push(row_to_audit_log(row)?);
        }

        Ok(logs)
    }
}

fn row_to_audit_log(row: &rusqlite::Row) -> Result<AuditLog, AppError> {
    let created_at_str: String = row.get(6)?;
    let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(AuditLog {
        id: row.get(0)?,
        user_id: row.get(1)?,
        username: row.get(2)?,
        screen: row.get(3)?,
        action: row.get(4)?,
        detail: row.get(5)?,
        created_at,
    })
}
