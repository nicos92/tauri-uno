use std::sync::Arc;
use std::time::Duration;

use serde::Deserialize;

use crate::domain::entities::DollarRate;
use crate::domain::repositories::DollarRateRepository;
use crate::infrastructure::error::AppError;
use crate::infrastructure::repositories::SqliteDollarRateRepository;

const DOLAR_API_URL: &str = "https://dolarapi.com/v1/dolares";
const HTTP_TIMEOUT_SECONDS: u64 = 10;

#[derive(Clone)]
pub struct DollarService {
    repository: Arc<dyn DollarRateRepository>,
    client: Arc<DollarHttpClient>,
}

impl Default for DollarService {
    fn default() -> Self {
        Self::new()
    }
}

impl DollarService {
    pub fn new() -> Self {
        Self::with_repository_and_client(
            Arc::new(SqliteDollarRateRepository::new()),
            Arc::new(DollarHttpClient::new()),
        )
    }

    pub fn with_repository(
        repository: Arc<dyn DollarRateRepository>,
        base_url: &str,
    ) -> Self {
        Self::with_repository_and_client(
            repository,
            Arc::new(DollarHttpClient::with_base(base_url)),
        )
    }

    pub fn with_repository_and_client(
        repository: Arc<dyn DollarRateRepository>,
        client: Arc<DollarHttpClient>,
    ) -> Self {
        Self { repository, client }
    }

    pub async fn fetch_from_api(&self) -> Result<Vec<DollarRate>, AppError> {
        self.client.fetch_rates().await
    }

    pub async fn fetch_and_persist(&self) -> Result<Vec<DollarRate>, AppError> {
        let rates = self.client.fetch_rates().await?;

        for rate in &rates {
            self.repository.upsert(rate)?;
        }

        Ok(rates)
    }

    pub fn get_latest(&self) -> Result<Vec<DollarRate>, AppError> {
        self.repository.find_all()
    }

    pub fn get_by_type(&self, dollar_type: &str) -> Result<Option<DollarRate>, AppError> {
        self.repository.find_by_type(dollar_type)
    }
}

#[derive(Clone)]
pub struct DollarHttpClient {
    inner: reqwest::Client,
    base_url: Arc<str>,
}

impl Default for DollarHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl DollarHttpClient {
    pub fn new() -> Self {
        Self::with_base(DOLAR_API_URL)
    }

    pub fn with_base(base_url: &str) -> Self {
        Self {
            inner: reqwest::Client::new(),
            base_url: Arc::from(base_url),
        }
    }

    pub async fn fetch_rates(&self) -> Result<Vec<DollarRate>, AppError> {
        let response = self
            .inner
            .get(self.base_url.as_ref())
            .timeout(Duration::from_secs(HTTP_TIMEOUT_SECONDS))
            .send()
            .await
            .map_err(|e| AppError::DollarFetchError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AppError::DollarFetchError(format!(
                "La API respondió con estado {}",
                response.status()
            )));
        }

        let body = response
            .text()
            .await
            .map_err(|e| AppError::DollarFetchError(e.to_string()))?;

        parse_api_response(&body)
    }
}

#[derive(Deserialize)]
struct ApiDollarRate {
    casa: String,
    compra: f64,
    venta: f64,
    #[serde(rename = "fechaActualizacion")]
    fecha_actualizacion: String,
}

pub fn parse_api_response(body: &str) -> Result<Vec<DollarRate>, AppError> {
    let api_rates: Vec<ApiDollarRate> = serde_json::from_str(body).map_err(|e| {
        AppError::DollarFetchError(format!("Respuesta inválida de la API: {e}"))
    })?;

    let mut rates: Vec<DollarRate> = api_rates
        .into_iter()
        .filter(|rate| matches!(rate.casa.as_str(), "oficial" | "blue"))
        .map(|rate| DollarRate {
            dollar_type: rate.casa,
            buy_price: rate.compra,
            sell_price: rate.venta,
            updated_at: rate.fecha_actualizacion,
        })
        .collect();

    rates.sort_by(|a, b| a.dollar_type.cmp(&b.dollar_type));

    Ok(rates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::dollar_rate_repository::MockDollarRateRepository;
    use mockall::predicate::*;

    const UNREACHABLE_URL: &str = "http://127.0.0.1:9/dolares";

    #[test]
    fn parse_filters_and_maps_only_oficial_and_blue() {
        let body = r#"[
            { "moneda": "Oficial", "casa": "oficial", "nombre": "Dólar oficial", "compra": 1000.0, "venta": 1040.0, "fechaActualizacion": "2025-01-15T12:30:00.000-03:00" },
            { "moneda": "Blue", "casa": "blue", "nombre": "Dólar blue", "compra": 1200.0, "venta": 1240.0, "fechaActualizacion": "2025-01-15T12:30:00.000-03:00" },
            { "moneda": "Bolsa", "casa": "bolsa", "nombre": "Dólar bolsa", "compra": 1100.0, "venta": 1140.0, "fechaActualizacion": "2025-01-15T12:30:00.000-03:00" }
        ]"#;

        let rates = parse_api_response(body).unwrap();

        assert_eq!(rates.len(), 2);
        assert_eq!(rates[0].dollar_type, "blue");
        assert_eq!(rates[0].buy_price, 1200.0);
        assert_eq!(rates[0].sell_price, 1240.0);
        assert_eq!(rates[1].dollar_type, "oficial");
        assert_eq!(rates[1].buy_price, 1000.0);
        assert_eq!(rates[1].updated_at, "2025-01-15T12:30:00.000-03:00");
    }

    #[test]
    fn parse_rejects_invalid_payload() {
        let err = parse_api_response("not json").unwrap_err();
        assert!(matches!(err, AppError::DollarFetchError(_)));
    }

    #[test]
    fn fetch_from_api_fails_with_connection_error() {
        let client = DollarHttpClient::with_base(UNREACHABLE_URL);
        let service = DollarService::with_repository_and_client(
            Arc::new(MockDollarRateRepository::new()),
            Arc::new(client),
        );

        let result = tauri::async_runtime::block_on(service.fetch_from_api());
        assert!(matches!(result, Err(AppError::DollarFetchError(_))));
    }

    #[test]
    fn fetch_and_persist_propagates_fetch_error_without_persisting() {
        let mut repo = MockDollarRateRepository::new();
        repo.expect_upsert().never();
        let service = DollarService::with_repository(Arc::new(repo), UNREACHABLE_URL);

        let result = tauri::async_runtime::block_on(service.fetch_and_persist());
        assert!(matches!(result, Err(AppError::DollarFetchError(_))));
    }

    #[test]
    fn get_latest_delegates_to_repository() {
        let mut repo = MockDollarRateRepository::new();
        let rates = vec![DollarRate {
            dollar_type: "oficial".to_string(),
            buy_price: 1000.0,
            sell_price: 1040.0,
            updated_at: "2025-01-15T12:30:00.000-03:00".to_string(),
        }];
        repo.expect_find_all().return_once(move || Ok(rates));
        let service = DollarService::with_repository_and_client(
            Arc::new(repo),
            Arc::new(DollarHttpClient::new()),
        );

        let result = service.get_latest().unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].dollar_type, "oficial");
    }

    #[test]
    fn get_by_type_delegates_to_repository() {
        let mut repo = MockDollarRateRepository::new();
        repo.expect_find_by_type()
            .with(eq("blue"))
            .return_once(|_| Ok(None));
        let service = DollarService::with_repository_and_client(
            Arc::new(repo),
            Arc::new(DollarHttpClient::new()),
        );

        assert!(service.get_by_type("blue").unwrap().is_none());
    }
}
