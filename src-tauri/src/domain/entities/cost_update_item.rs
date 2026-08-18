use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostUpdateItem {
    pub id: i64,
    pub operation_id: i64,
    pub id_stock: i64,
    pub costo_anterior: f64,
    pub costo_nuevo: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_round_trip() {
        let item = CostUpdateItem {
            id: 1,
            operation_id: 10,
            id_stock: 55,
            costo_anterior: 1000.0,
            costo_nuevo: 1200.0,
        };
        let json = serde_json::to_string(&item).unwrap();
        let back: CostUpdateItem = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, 1);
        assert_eq!(back.operation_id, 10);
        assert_eq!(back.id_stock, 55);
        assert!((back.costo_anterior - 1000.0).abs() < 0.01);
        assert!((back.costo_nuevo - 1200.0).abs() < 0.01);
    }
}
