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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_fields() {
        let s = SubCategoria::new("Gaseosas".to_string(), 3);
        assert_eq!(s.id, 0);
        assert_eq!(s.sub_categoria, "Gaseosas");
        assert_eq!(s.id_categoria, 3);
    }

    #[test]
    fn json_round_trip() {
        let s = SubCategoria::new("Gaseosas".to_string(), 3);
        let json = serde_json::to_string(&s).unwrap();
        let back: SubCategoria = serde_json::from_str(&json).unwrap();
        assert_eq!(back.sub_categoria, "Gaseosas");
        assert_eq!(back.id_categoria, 3);
        assert_eq!(back.id, 0);
    }
}
