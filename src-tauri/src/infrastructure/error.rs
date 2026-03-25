use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

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

    #[error("Internal error: {0}")]
    Internal(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
