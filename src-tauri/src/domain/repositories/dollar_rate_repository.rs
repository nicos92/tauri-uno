use crate::domain::entities::DollarRate;
use crate::infrastructure::error::AppError;

#[cfg_attr(test, mockall::automock)]
pub trait DollarRateRepository: Send + Sync {
    fn upsert(&self, rate: &DollarRate) -> Result<(), AppError>;
    fn find_all(&self) -> Result<Vec<DollarRate>, AppError>;
    fn find_by_type(&self, dollar_type: &str) -> Result<Option<DollarRate>, AppError>;
}
