use crate::domain::entities::AuditLog;
use crate::domain::repositories::Page;
use crate::infrastructure::error::AppError;

#[derive(Debug, Clone, Default)]
pub struct AuditLogFilter {
    pub user_id: Option<i64>,
    pub screen: Option<String>,
    pub action: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[cfg_attr(test, mockall::automock)]
pub trait AuditLogRepository: Send + Sync {
    fn create(&self, log: &AuditLog) -> Result<AuditLog, AppError>;
    fn find_with_filters(&self, filter: &AuditLogFilter) -> Result<Page<AuditLog>, AppError>;
    fn get_username(&self, user_id: i64) -> Result<Option<String>, AppError>;
}
