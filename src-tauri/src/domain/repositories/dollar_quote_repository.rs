use crate::domain::entities::DollarQuote;
use crate::infrastructure::error::AppError;

#[cfg_attr(test, mockall::automock)]
pub trait DollarQuoteRepository: Send + Sync {
    fn save(&self, quote: &DollarQuote) -> Result<DollarQuote, AppError>;
    fn find_all(&self) -> Result<Vec<DollarQuote>, AppError>;
    fn delete_by_id(&self, id: i64) -> Result<(), AppError>;
}
