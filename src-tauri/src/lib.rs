pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

use api::commands::{
    add_permission_to_user, create_categoria, create_permission, create_proveedor, create_user,
    delete_categoria, delete_proveedor, delete_user, get_all_categorias, get_all_permissions,
    get_all_proveedores, get_all_users, get_proveedor_by_id, get_user_permissions, login,
    remove_permission_from_user, update_categoria, update_proveedor, update_user, AppState,
    CategoriaAppState, ProveedorAppState,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = &infrastructure::database::DB;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .manage(ProveedorAppState::new())
        .manage(CategoriaAppState::new())
        .invoke_handler(tauri::generate_handler![
            login,
            create_user,
            get_all_users,
            update_user,
            delete_user,
            add_permission_to_user,
            remove_permission_from_user,
            get_user_permissions,
            get_all_permissions,
            create_permission,
            get_all_proveedores,
            get_proveedor_by_id,
            create_proveedor,
            update_proveedor,
            delete_proveedor,
            get_all_categorias,
            create_categoria,
            update_categoria,
            delete_categoria
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
