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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_id_zero_and_name() {
        let c = Categoria::new("Bebidas".to_string());
        assert_eq!(c.id, 0);
        assert_eq!(c.categoria, "Bebidas");
    }

    #[test]
    fn json_round_trip() {
        let c = Categoria::new("Almacén".to_string());
        let json = serde_json::to_string(&c).unwrap();
        let back: Categoria = serde_json::from_str(&json).unwrap();
        assert_eq!(back.categoria, "Almacén");
        assert_eq!(back.id, 0);
    }
}
