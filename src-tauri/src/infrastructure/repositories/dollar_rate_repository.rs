use rusqlite::params;

use crate::domain::entities::DollarRate;
use crate::domain::repositories::DollarRateRepository;
use crate::infrastructure::database::DB;
use crate::infrastructure::error::AppError;

pub struct SqliteDollarRateRepository;

impl Default for SqliteDollarRateRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl SqliteDollarRateRepository {
    pub fn new() -> Self {
        Self
    }
}

impl DollarRateRepository for SqliteDollarRateRepository {
    fn upsert(&self, rate: &DollarRate) -> Result<(), AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        conn.execute(
            "INSERT INTO dollar_rates (dollar_type, buy_price, sell_price, updated_at)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(dollar_type) DO UPDATE SET
                buy_price = excluded.buy_price,
                sell_price = excluded.sell_price,
                updated_at = excluded.updated_at",
            params![
                rate.dollar_type,
                rate.buy_price,
                rate.sell_price,
                rate.updated_at
            ],
        )?;

        Ok(())
    }

    fn find_all(&self) -> Result<Vec<DollarRate>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT dollar_type, buy_price, sell_price, updated_at
             FROM dollar_rates ORDER BY dollar_type",
        )?;

        let mut rates = Vec::new();
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            rates.push(row_to_rate(row)?);
        }

        Ok(rates)
    }

    fn find_by_type(&self, dollar_type: &str) -> Result<Option<DollarRate>, AppError> {
        let conn = DB.lock().map_err(|e| AppError::Internal(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT dollar_type, buy_price, sell_price, updated_at
             FROM dollar_rates WHERE dollar_type = ?1 LIMIT 1",
        )?;

        let mut rows = stmt.query(params![dollar_type])?;

        if let Some(row) = rows.next()? {
            Ok(Some(row_to_rate(row)?))
        } else {
            Ok(None)
        }
    }
}

fn row_to_rate(row: &rusqlite::Row) -> Result<DollarRate, AppError> {
    Ok(DollarRate {
        dollar_type: row.get(0)?,
        buy_price: row.get(1)?,
        sell_price: row.get(2)?,
        updated_at: row.get(3)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database::{reset_test_db, TEST_LOCK};
    use std::sync::MutexGuard;

    fn fresh_db() -> MutexGuard<'static, ()> {
        let guard = TEST_LOCK.lock().unwrap();
        reset_test_db().unwrap();
        guard
    }

    fn sample_rate(dollar_type: &str) -> DollarRate {
        DollarRate {
            dollar_type: dollar_type.to_string(),
            buy_price: 1000.0,
            sell_price: 1040.0,
            updated_at: "2025-01-15T12:30:00.000-03:00".to_string(),
        }
    }

    #[test]
    fn upsert_inserts_and_find_by_type_round_trip() {
        let _guard = fresh_db();
        let repo = SqliteDollarRateRepository::new();

        repo.upsert(&sample_rate("oficial")).unwrap();

        let found = repo.find_by_type("oficial").unwrap().unwrap();
        assert_eq!(found.dollar_type, "oficial");
        assert_eq!(found.buy_price, 1000.0);
        assert_eq!(found.sell_price, 1040.0);
        assert_eq!(found.updated_at, "2025-01-15T12:30:00.000-03:00");
    }

    #[test]
    fn upsert_overwrites_previous_value_for_same_type() {
        let _guard = fresh_db();
        let repo = SqliteDollarRateRepository::new();

        repo.upsert(&sample_rate("blue")).unwrap();

        let mut updated = sample_rate("blue");
        updated.buy_price = 1300.0;
        updated.sell_price = 1340.0;
        repo.upsert(&updated).unwrap();

        let all = repo.find_all().unwrap();
        assert_eq!(all.len(), 1);

        let found = repo.find_by_type("blue").unwrap().unwrap();
        assert_eq!(found.buy_price, 1300.0);
        assert_eq!(found.sell_price, 1340.0);
    }

    #[test]
    fn find_all_returns_both_types_ordered() {
        let _guard = fresh_db();
        let repo = SqliteDollarRateRepository::new();

        repo.upsert(&sample_rate("oficial")).unwrap();
        repo.upsert(&sample_rate("blue")).unwrap();

        let all = repo.find_all().unwrap();
        let types: Vec<&str> = all.iter().map(|r| r.dollar_type.as_str()).collect();
        assert_eq!(types, vec!["blue", "oficial"]);
    }

    #[test]
    fn find_by_type_returns_none_for_unknown_type() {
        let _guard = fresh_db();
        let repo = SqliteDollarRateRepository::new();

        assert!(repo.find_by_type("cripto").unwrap().is_none());
    }
}
