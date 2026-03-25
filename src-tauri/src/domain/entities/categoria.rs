use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Categoria {
    pub id: i64,
    pub categoria: String,
}

impl Categoria {
    pub fn new(categoria: String) -> Self {
        Self { id: 0, categoria }
    }
}
