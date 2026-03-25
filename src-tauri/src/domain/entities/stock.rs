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
