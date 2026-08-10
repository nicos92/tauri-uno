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

pub trait AuditLogRepository: Send + Sync {
    fn create(&self, log: &AuditLog) -> Result<AuditLog, AppError>;
    fn find_with_filters(&self, filter: &AuditLogFilter) -> Result<Page<AuditLog>, AppError>;
}
