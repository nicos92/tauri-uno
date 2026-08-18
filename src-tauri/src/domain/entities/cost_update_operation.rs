use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CostUpdateEstado {
    #[serde(rename = "aplicada")]
    Aplicada,
    #[serde(rename = "deshecha")]
    Deshecha,
}

impl CostUpdateEstado {
    pub fn as_str(&self) -> &'static str {
        match self {
            CostUpdateEstado::Aplicada => "aplicada",
            CostUpdateEstado::Deshecha => "deshecha",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "aplicada" => Some(CostUpdateEstado::Aplicada),
            "deshecha" => Some(CostUpdateEstado::Deshecha),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostUpdateOperation {
    pub id: i64,
    pub user_id: i64,
    pub porcentaje: f64,
    pub filtro_categoria: Option<i64>,
    pub filtro_sub_categoria: Option<i64>,
    pub filtro_proveedor: Option<i64>,
    pub affected_count: i64,
    pub estado: CostUpdateEstado,
    pub created_at: String,
    pub undone_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estado_as_str_roundtrip() {
        assert_eq!(CostUpdateEstado::Aplicada.as_str(), "aplicada");
        assert_eq!(CostUpdateEstado::Deshecha.as_str(), "deshecha");
        assert_eq!(
            CostUpdateEstado::parse("aplicada"),
            Some(CostUpdateEstado::Aplicada)
        );
        assert_eq!(
            CostUpdateEstado::parse("deshecha"),
            Some(CostUpdateEstado::Deshecha)
        );
        assert_eq!(CostUpdateEstado::parse("invalido"), None);
    }

    #[test]
    fn json_round_trip() {
        let op = CostUpdateOperation {
            id: 1,
            user_id: 5,
            porcentaje: 20.0,
            filtro_categoria: Some(3),
            filtro_sub_categoria: None,
            filtro_proveedor: None,
            affected_count: 10,
            estado: CostUpdateEstado::Aplicada,
            created_at: "2025-01-01T12:00:00".to_string(),
            undone_at: None,
        };
        let json = serde_json::to_string(&op).unwrap();
        let back: CostUpdateOperation = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, 1);
        assert_eq!(back.porcentaje, 20.0);
        assert_eq!(back.estado, CostUpdateEstado::Aplicada);
        assert!(back.undone_at.is_none());
    }

    #[test]
    fn estado_serializes_to_json_string() {
        let aplicada = serde_json::to_string(&CostUpdateEstado::Aplicada).unwrap();
        assert_eq!(aplicada, "\"aplicada\"");
        let deshecha = serde_json::to_string(&CostUpdateEstado::Deshecha).unwrap();
        assert_eq!(deshecha, "\"deshecha\"");
    }
}
