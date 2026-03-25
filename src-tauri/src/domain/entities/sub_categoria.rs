use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubCategoria {
    pub id: i64,
    pub sub_categoria: String,
    pub id_categoria: i64,
}

impl SubCategoria {
    pub fn new(sub_categoria: String, id_categoria: i64) -> Self {
        Self {
            id: 0,
            sub_categoria,
            id_categoria,
        }
    }
}
