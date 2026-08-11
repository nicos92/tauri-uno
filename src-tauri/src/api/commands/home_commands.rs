use std::sync::Mutex;
use tauri::State;

use crate::application::services::HomeService;
use crate::domain::entities::HomeStats;
use crate::infrastructure::error::AppError;

pub struct HomeStatsAppState {
    pub home_service: Mutex<HomeService>,
}

impl Default for HomeStatsAppState {
    fn default() -> Self {
        Self::new()
    }
}

impl HomeStatsAppState {
    pub fn new() -> Self {
        Self {
            home_service: Mutex::new(HomeService::new()),
        }
    }
}

#[tauri::command(async)]
pub fn get_home_stats(
    user_id: i64,
    state: State<HomeStatsAppState>,
) -> Result<HomeStats, AppError> {
    let _ = user_id;
    let service = state
        .home_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    service.get_stats()
}
