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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_fields() {
        let a = Articulo::new("Coca Cola 1.5L".to_string(), "CC1500".to_string(), 4, 7);
        assert_eq!(a.id, 0);
        assert_eq!(a.articulo, "Coca Cola 1.5L");
        assert_eq!(a.cod_articulo, "CC1500");
        assert_eq!(a.id_sub_categoria, 4);
        assert_eq!(a.id_proveedor, 7);
    }

    #[test]
    fn json_round_trip() {
        let a = Articulo::new("Coca Cola 1.5L".to_string(), "CC1500".to_string(), 4, 7);
        let json = serde_json::to_string(&a).unwrap();
        let back: Articulo = serde_json::from_str(&json).unwrap();
        assert_eq!(back.articulo, a.articulo);
        assert_eq!(back.cod_articulo, a.cod_articulo);
        assert_eq!(back.id_sub_categoria, 4);
        assert_eq!(back.id_proveedor, 7);
        assert_eq!(back.id, 0);
    }
}
