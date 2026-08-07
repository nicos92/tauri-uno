use rusqlite::ffi::ErrorCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Foreign key constraint violation")]
    ForeignKeyConstraint,

    #[error("Duplicate value violates uniqueness constraint")]
    DuplicateValue,

    #[error("User not found")]
    UserNotFound,

    #[error("Permission not found")]
    PermissionNotFound,

    #[error("Username already exists")]
    UsernameExists,

    #[error("Permission already exists")]
    PermissionExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User is inactive")]
    UserInactive,

    #[error("Hashing error: {0}")]
    Hashing(String),

    #[error("Permission already assigned to user")]
    PermissionAlreadyAssigned,

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Proveedor not found")]
    ProveedorNotFound,

    #[error("CUIT already exists")]
    DuplicateCuit,

    #[error("Categoria not found")]
    CategoriaNotFound,

    #[error("Categoria already exists")]
    CategoriaExists,

    #[error("Categoria has sub-categorias")]
    CategoriaHasSubCategorias,

    #[error("Sub categoria not found")]
    SubCategoriaNotFound,

    #[error("Sub categoria already exists")]
    SubCategoriaExists,

    #[error("Sub categoria has articulos")]
    SubCategoriaHasArticulos,

    #[error("Articulo not found")]
    ArticuloNotFound,

    #[error("Codigo de articulo already exists")]
    CodArticuloExists,

    #[error("Stock not found")]
    StockNotFound,

    #[error("Stock already exists for this articulo")]
    StockExistsForArticulo,

    #[error("Proveedor has articulos")]
    ProveedorHasArticulos,

    #[error("Cannot delete the current user")]
    CannotDeleteSelf,

    #[error("Cannot delete the admin user")]
    CannotDeleteAdmin,

    #[error("Venta not found")]
    VentaNotFound,

    #[error("Venta already anulada")]
    VentaAlreadyAnulada,

    #[error("Insufficient stock")]
    InsufficientStock,

    #[error("Articulo has no stock")]
    ArticuloWithoutStock,

    #[error("Descuento inválido")]
    DescuentoInvalido,

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        match error {
            rusqlite::Error::SqliteFailure(ffi_error, msg) => {
                let extended_code = ffi_error.extended_code;

                if ffi_error.code == ErrorCode::ConstraintViolation {
                    match extended_code {
                        787 => AppError::ForeignKeyConstraint,
                        2067 => AppError::DuplicateValue,
                        _ => AppError::Database(
                            msg.unwrap_or_else(|| ffi_error.to_string()),
                        ),
                    }
                } else {
                    AppError::Database(msg.unwrap_or_else(|| ffi_error.to_string()))
                }
            }
            other => AppError::Database(other.to_string()),
        }
    }
}

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Database(_) => "database_error",
            AppError::ForeignKeyConstraint => "foreign_key_constraint",
            AppError::DuplicateValue => "duplicate_value",
            AppError::UserNotFound => "user_not_found",
            AppError::PermissionNotFound => "permission_not_found",
            AppError::UsernameExists => "username_exists",
            AppError::PermissionExists => "permission_exists",
            AppError::InvalidCredentials => "invalid_credentials",
            AppError::UserInactive => "user_inactive",
            AppError::Hashing(_) => "hashing_error",
            AppError::PermissionAlreadyAssigned => "permission_already_assigned",
            AppError::PermissionDenied => "permission_denied",
            AppError::ProveedorNotFound => "proveedor_not_found",
            AppError::DuplicateCuit => "duplicate_cuit",
            AppError::CategoriaNotFound => "categoria_not_found",
            AppError::CategoriaExists => "categoria_exists",
            AppError::CategoriaHasSubCategorias => "categoria_has_sub_categorias",
            AppError::SubCategoriaNotFound => "sub_categoria_not_found",
            AppError::SubCategoriaExists => "sub_categoria_exists",
            AppError::SubCategoriaHasArticulos => "sub_categoria_has_articulos",
            AppError::ArticuloNotFound => "articulo_not_found",
            AppError::CodArticuloExists => "cod_articulo_exists",
            AppError::StockNotFound => "stock_not_found",
            AppError::StockExistsForArticulo => "stock_exists_for_articulo",
            AppError::ProveedorHasArticulos => "proveedor_has_articulos",
            AppError::CannotDeleteSelf => "cannot_delete_self",
            AppError::CannotDeleteAdmin => "cannot_delete_admin",
            AppError::VentaNotFound => "venta_not_found",
            AppError::VentaAlreadyAnulada => "venta_already_anulada",
            AppError::InsufficientStock => "insufficient_stock",
            AppError::ArticuloWithoutStock => "articulo_without_stock",
            AppError::DescuentoInvalido => "descuento_invalido",
            AppError::Internal(_) => "internal_error",
        }
    }

    pub fn user_message(&self) -> String {
        match self {
            AppError::Database(_) => {
                "Ocurrió un error en la base de datos. Intente nuevamente.".to_string()
            }
            AppError::ForeignKeyConstraint => {
                "No se puede eliminar porque otros registros hacen referencia a este elemento."
                    .to_string()
            }
            AppError::DuplicateValue => {
                "Ya existe un registro con ese valor único.".to_string()
            }
            AppError::UserNotFound => "El usuario no existe.".to_string(),
            AppError::PermissionNotFound => "El permiso no existe.".to_string(),
            AppError::UsernameExists => "El nombre de usuario ya existe.".to_string(),
            AppError::PermissionExists => "El permiso ya existe.".to_string(),
            AppError::InvalidCredentials => {
                "Usuario o contraseña incorrectos.".to_string()
            }
            AppError::UserInactive => "El usuario está inactivo.".to_string(),
            AppError::Hashing(_) => {
                "Ocurrió un error al procesar la contraseña.".to_string()
            }
            AppError::PermissionAlreadyAssigned => {
                "El permiso ya está asignado a este usuario.".to_string()
            }
            AppError::PermissionDenied => {
                "No tiene permisos para realizar esta acción.".to_string()
            }
            AppError::ProveedorNotFound => "El proveedor no existe.".to_string(),
            AppError::DuplicateCuit => "El CUIT ya está registrado.".to_string(),
            AppError::CategoriaNotFound => "La categoría no existe.".to_string(),
            AppError::CategoriaExists => "La categoría ya existe.".to_string(),
            AppError::CategoriaHasSubCategorias => {
                "No se puede eliminar la categoría porque tiene sub categorías asociadas."
                    .to_string()
            }
            AppError::SubCategoriaNotFound => "La sub categoría no existe.".to_string(),
            AppError::SubCategoriaExists => "La sub categoría ya existe.".to_string(),
            AppError::SubCategoriaHasArticulos => {
                "No se puede eliminar la sub categoría porque tiene artículos asociados."
                    .to_string()
            }
            AppError::ArticuloNotFound => "El artículo no existe.".to_string(),
            AppError::CodArticuloExists => {
                "El código de artículo ya está registrado.".to_string()
            }
            AppError::StockNotFound => "El stock no existe.".to_string(),
            AppError::StockExistsForArticulo => {
                "El artículo ya tiene stock registrado.".to_string()
            }
            AppError::ProveedorHasArticulos => {
                "No se puede eliminar el proveedor porque tiene artículos asociados."
                    .to_string()
            }
            AppError::CannotDeleteSelf => {
                "No se puede eliminar a sí mismo.".to_string()
            }
            AppError::CannotDeleteAdmin => {
                "No se puede eliminar el usuario administrador.".to_string()
            }
            AppError::VentaNotFound => "La venta no existe.".to_string(),
            AppError::VentaAlreadyAnulada => "La venta ya fue anulada.".to_string(),
            AppError::InsufficientStock => {
                "Stock insuficiente para uno de los artículos.".to_string()
            }
            AppError::ArticuloWithoutStock => {
                "Uno de los artículos no tiene stock registrado.".to_string()
            }
            AppError::DescuentoInvalido => {
                "El descuento debe estar entre 0 y 100.".to_string()
            }
            AppError::Internal(_) => {
                "Ocurrió un error inesperado. Intente nuevamente.".to_string()
            }
        }
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("AppError", 2)?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("message", &self.user_message())?;
        state.end()
    }
}
