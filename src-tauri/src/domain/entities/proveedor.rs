use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proveedor {
    pub id: i64,
    pub cuit: Option<String>,
    pub proveedor: String,
    pub nombre: String,
    pub tel: Option<String>,
    pub email: Option<String>,
    pub observacion: Option<String>,
}

impl Proveedor {
    pub fn new(
        proveedor: String,
        nombre: String,
        cuit: Option<String>,
        tel: Option<String>,
        email: Option<String>,
        observacion: Option<String>,
    ) -> Self {
        Self {
            id: 0,
            proveedor,
            nombre,
            cuit,
            tel,
            email,
            observacion,
        }
    }
}
