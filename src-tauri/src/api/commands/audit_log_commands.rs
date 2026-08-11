use std::sync::Mutex;
use tauri::State;

use crate::api::commands::permissions::check_permission;
use crate::application::services::AuditLogService;
use crate::domain::entities::{AuditLog, PermissionCode};
use crate::domain::repositories::{AuditLogFilter, Page};
use crate::infrastructure::error::AppError;

pub struct AuditLogAppState {
    pub audit_service: Mutex<AuditLogService>,
}

impl Default for AuditLogAppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditLogAppState {
    pub fn new() -> Self {
        Self {
            audit_service: Mutex::new(AuditLogService::new()),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct GetAuditLogsRequest {
    pub user_id: Option<i64>,
    pub screen: Option<String>,
    pub action: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[tauri::command(async)]
pub fn get_audit_logs(
    user_id: i64,
    request: GetAuditLogsRequest,
    state: State<AuditLogAppState>,
) -> Result<Page<AuditLog>, AppError> {
    let service = state
        .audit_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    check_permission(user_id, PermissionCode::ViewAuditoria)?;

    let filter = AuditLogFilter {
        user_id: request.user_id,
        screen: request.screen,
        action: request.action,
        from: request.from,
        to: request.to,
        limit: request.limit,
        offset: request.offset,
    };

    service.get_logs(&filter)
}
