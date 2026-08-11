use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub id: i64,
    pub id_articulo: i64,
    pub cantidad: f64,
    pub costo: f64,
    pub ganancia: f64,
}

impl Stock {
    pub fn new(id_articulo: i64, cantidad: f64, costo: f64, ganancia: f64) -> Self {
        Self {
            id: 0,
            id_articulo,
            cantidad,
            costo,
            ganancia,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_fields() {
        let s = Stock::new(12, 50.5, 100.0, 30.0);
        assert_eq!(s.id, 0);
        assert_eq!(s.id_articulo, 12);
        assert_eq!(s.cantidad, 50.5);
        assert_eq!(s.costo, 100.0);
        assert_eq!(s.ganancia, 30.0);
    }

    #[test]
    fn new_accepts_zero_and_negative_cantidad() {
        let s = Stock::new(12, 0.0, 0.0, 0.0);
        assert_eq!(s.cantidad, 0.0);
        assert_eq!(s.costo, 0.0);
        assert_eq!(s.ganancia, 0.0);
    }

    #[test]
    fn json_round_trip() {
        let s = Stock::new(12, 50.5, 100.0, 30.0);
        let json = serde_json::to_string(&s).unwrap();
        let back: Stock = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id_articulo, 12);
        assert_eq!(back.cantidad, 50.5);
        assert_eq!(back.costo, 100.0);
        assert_eq!(back.ganancia, 30.0);
        assert_eq!(back.id, 0);
    }
}
