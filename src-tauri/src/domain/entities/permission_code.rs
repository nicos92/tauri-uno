use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionCode {
    CreateUser,
    UpdateUser,
    DeleteUser,
    AssignPermission,
    RemovePermission,
    ViewUsers,
    ViewPermissions,
    CreatePermission,
}

impl PermissionCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionCode::CreateUser => "crear_usuario",
            PermissionCode::UpdateUser => "modificar_usuario",
            PermissionCode::DeleteUser => "eliminar_usuario",
            PermissionCode::AssignPermission => "asignar_permiso_a_usuario",
            PermissionCode::RemovePermission => "quitar_permiso_a_usuario",
            PermissionCode::ViewUsers => "ver_usuarios",
            PermissionCode::ViewPermissions => "ver_permisos",
            PermissionCode::CreatePermission => "crear_permiso",
        }
    }

    pub fn all() -> Vec<PermissionCode> {
        vec![
            PermissionCode::CreateUser,
            PermissionCode::UpdateUser,
            PermissionCode::DeleteUser,
            PermissionCode::AssignPermission,
            PermissionCode::RemovePermission,
            PermissionCode::ViewUsers,
            PermissionCode::ViewPermissions,
            PermissionCode::CreatePermission,
        ]
    }
}
