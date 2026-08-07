use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cierre {
    pub id: i64,
    pub fecha: String,
    pub dia: i64,
    pub mes: i64,
    pub anio: i64,
    pub total_costo: f64,
    pub total_ganancia: f64,
    pub total_venta: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CierreTipo {
    pub id_tipo_venta: i64,
    pub tipo_venta: String,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CierreWithTipos {
    #[serde(flatten)]
    pub cierre: Cierre,
    pub tipos: Vec<CierreTipo>,
}
