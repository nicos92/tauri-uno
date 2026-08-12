use std::sync::Mutex;
use std::time::Duration;

use tauri::{AppHandle, Emitter, State};
use tokio::sync::watch;

use crate::api::commands::permissions::check_permission;
use crate::application::services::{log_audit, DollarService};
use crate::domain::entities::{AuditAction, AuditScreen, DollarRate, PermissionCode};
use crate::infrastructure::error::AppError;

pub const DEFAULT_POLLING_INTERVAL_SECONDS: u64 = 300;
const POLLING_PAUSED: u64 = 0;

pub struct DollarAppState {
    pub dollar_service: Mutex<DollarService>,
    pub polling: Mutex<PollingState>,
}

pub struct PollingState {
    pub interval_seconds: u64,
    tx: Option<watch::Sender<u64>>,
    handle: Option<tauri::async_runtime::JoinHandle<()>>,
}

impl Default for PollingState {
    fn default() -> Self {
        Self {
            interval_seconds: DEFAULT_POLLING_INTERVAL_SECONDS,
            tx: None,
            handle: None,
        }
    }
}

impl Default for DollarAppState {
    fn default() -> Self {
        Self::new()
    }
}

impl DollarAppState {
    pub fn new() -> Self {
        Self {
            dollar_service: Mutex::new(DollarService::new()),
            polling: Mutex::new(PollingState::default()),
        }
    }

    pub fn interval_seconds(&self) -> u64 {
        self.polling
            .lock()
            .map(|p| p.interval_seconds)
            .unwrap_or(DEFAULT_POLLING_INTERVAL_SECONDS)
    }

    pub fn start_polling(&self, app: AppHandle, seconds: u64) -> Result<(), AppError> {
        let mut polling = self
            .polling
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        polling.interval_seconds = seconds;

        if let Some(tx) = &polling.tx {
            let _ = tx.send(seconds);
            return Ok(());
        }

        let (tx, rx) = watch::channel(seconds);
        let service = self
            .dollar_service
            .lock()
            .map_err(|e| AppError::Internal(e.to_string()))?
            .clone();

        polling.tx = Some(tx);
        polling.handle = Some(tauri::async_runtime::spawn(polling_loop(app, service, rx)));

        Ok(())
    }
}

async fn polling_loop(app: AppHandle, service: DollarService, mut rx: watch::Receiver<u64>) {
    loop {
        let interval = *rx.borrow();

        if interval == POLLING_PAUSED {
            if rx.changed().await.is_err() {
                return;
            }
            continue;
        }

        match service.fetch_and_persist().await {
            Ok(rates) => {
                let _ = app.emit("dollar-rates-updated", &rates);
            }
            Err(_) => {
                let _ = app.emit(
                    "dollar-rates-fetch-error",
                    "No se pudo actualizar la cotización del dólar.",
                );
            }
        }

        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}

#[tauri::command(async)]
pub fn get_latest_dollar_rates(
    user_id: i64,
    state: State<DollarAppState>,
) -> Result<Vec<DollarRate>, AppError> {
    check_permission(user_id, PermissionCode::ViewDolar)?;
    let service = state
        .dollar_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    service.get_latest()
}

#[tauri::command(async)]
pub async fn fetch_dollar_rates_manual(
    user_id: i64,
    state: State<'_, DollarAppState>,
) -> Result<Vec<DollarRate>, AppError> {
    check_permission(user_id, PermissionCode::ViewDolar)?;
    let service = state
        .dollar_service
        .lock()
        .map_err(|e| AppError::Internal(e.to_string()))?
        .clone();
    let rates = service.fetch_and_persist().await?;
    log_audit(
        user_id,
        AuditScreen::Dolar,
        AuditAction::Update,
        Some("Cotización del dólar actualizada manualmente".to_string()),
    )?;
    Ok(rates)
}

#[tauri::command(async)]
pub fn update_polling_interval(
    user_id: i64,
    seconds: u64,
    app: AppHandle,
    state: State<DollarAppState>,
) -> Result<u64, AppError> {
    check_permission(user_id, PermissionCode::ViewDolar)?;
    state.start_polling(app, seconds)?;
    Ok(seconds)
}
