use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipoVenta {
    pub id: i64,
    pub nombre: String,
    pub hacia_donde: Option<String>,
    pub created_at: String,
}

impl TipoVenta {
    pub fn new(nombre: String, hacia_donde: Option<String>) -> Self {
        Self {
            id: 0,
            nombre,
            hacia_donde,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
