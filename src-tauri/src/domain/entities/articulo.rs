use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Articulo {
    pub id: i64,
    pub articulo: String,
    pub cod_articulo: String,
    pub id_sub_categoria: i64,
    pub id_proveedor: i64,
}

impl Articulo {
    pub fn new(
        articulo: String,
        cod_articulo: String,
        id_sub_categoria: i64,
        id_proveedor: i64,
    ) -> Self {
        Self {
            id: 0,
            articulo,
            cod_articulo,
            id_sub_categoria,
            id_proveedor,
        }
    }
}
