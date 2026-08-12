use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DollarRate {
    pub dollar_type: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub updated_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_fields_round_trip_through_json() {
        let rate = DollarRate {
            dollar_type: "blue".to_string(),
            buy_price: 1200.5,
            sell_price: 1240.75,
            updated_at: "2025-01-15T12:30:00.000-03:00".to_string(),
        };

        let json = serde_json::to_string(&rate).unwrap();
        let back: DollarRate = serde_json::from_str(&json).unwrap();

        assert_eq!(back, rate);
    }

    #[test]
    fn deserializes_api_shaped_payload() {
        let json = r#"{ "dollar_type": "oficial", "buy_price": 1000.0, "sell_price": 1040.0, "updated_at": "2025-01-15T12:30:00.000-03:00" }"#;

        let rate: DollarRate = serde_json::from_str(json).unwrap();

        assert_eq!(rate.dollar_type, "oficial");
        assert_eq!(rate.buy_price, 1000.0);
        assert_eq!(rate.sell_price, 1040.0);
    }
}
