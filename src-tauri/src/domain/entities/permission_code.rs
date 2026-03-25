use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionCode {
    ViewUsers,
    CreateUser,
    UpdateUser,
    DeleteUser,
    ViewPermissions,
    AssignPermission,
    RemovePermission,
    ViewProveedores,
    CreateProveedor,
    UpdateProveedor,
    DeleteProveedor,
    ViewCategorias,
    CreateCategoria,
    UpdateCategoria,
    DeleteCategoria,
    ViewSubCategorias,
    CreateSubCategoria,
    UpdateSubCategoria,
    DeleteSubCategoria,
    ViewArticulos,
    CreateArticulo,
    UpdateArticulo,
    DeleteArticulo,
}

impl PermissionCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionCode::ViewUsers => "ver_usuarios",
            PermissionCode::CreateUser => "crear_usuario",
            PermissionCode::UpdateUser => "modificar_usuario",
            PermissionCode::DeleteUser => "eliminar_usuario",
            PermissionCode::ViewPermissions => "ver_permisos",
            PermissionCode::AssignPermission => "asignar_permiso_a_usuario",
            PermissionCode::RemovePermission => "quitar_permiso_a_usuario",
            PermissionCode::ViewProveedores => "ver_proveedor",
            PermissionCode::CreateProveedor => "crear_proveedor",
            PermissionCode::UpdateProveedor => "modificar_proveedor",
            PermissionCode::DeleteProveedor => "eliminar_proveedor",
            PermissionCode::ViewCategorias => "ver_categorias",
            PermissionCode::CreateCategoria => "crear_categorias",
            PermissionCode::UpdateCategoria => "modificar_categorias",
            PermissionCode::DeleteCategoria => "eliminar_categorias",
            PermissionCode::ViewSubCategorias => "ver_sub_categorias",
            PermissionCode::CreateSubCategoria => "crear_sub_categorias",
            PermissionCode::UpdateSubCategoria => "modificar_sub_categorias",
            PermissionCode::DeleteSubCategoria => "eliminar_sub_categorias",
            PermissionCode::ViewArticulos => "ver_articulos",
            PermissionCode::CreateArticulo => "crear_articulos",
            PermissionCode::UpdateArticulo => "modificar_articulos",
            PermissionCode::DeleteArticulo => "eliminar_articulos",
        }
    }

    pub fn all() -> Vec<PermissionCode> {
        vec![
            PermissionCode::ViewUsers,
            PermissionCode::CreateUser,
            PermissionCode::UpdateUser,
            PermissionCode::DeleteUser,
            PermissionCode::ViewPermissions,
            PermissionCode::AssignPermission,
            PermissionCode::RemovePermission,
            PermissionCode::ViewProveedores,
            PermissionCode::CreateProveedor,
            PermissionCode::UpdateProveedor,
            PermissionCode::DeleteProveedor,
            PermissionCode::ViewCategorias,
            PermissionCode::CreateCategoria,
            PermissionCode::UpdateCategoria,
            PermissionCode::DeleteCategoria,
            PermissionCode::ViewSubCategorias,
            PermissionCode::CreateSubCategoria,
            PermissionCode::UpdateSubCategoria,
            PermissionCode::DeleteSubCategoria,
            PermissionCode::ViewArticulos,
            PermissionCode::CreateArticulo,
            PermissionCode::UpdateArticulo,
            PermissionCode::DeleteArticulo,
        ]
    }
}
