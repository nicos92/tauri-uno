pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

use api::commands::{
    add_permission_to_user, anular_venta, create_articulo, create_categoria, create_permission,
    create_proveedor, create_stock, create_sub_categoria, create_user, create_venta,
    delete_articulo, delete_categoria, delete_proveedor, delete_stock, delete_sub_categoria,
    delete_user, get_all_articulos, get_all_categorias, get_all_permissions, get_all_proveedores,
    get_all_stock, get_all_sub_categorias, get_all_users, get_all_ventas, get_audit_logs,
    get_precio_venta, get_proveedor_by_id, get_stock_by_articulo, get_stock_by_id,
    get_sub_categorias_by_categoria, get_user_permissions, get_venta_by_id, login,
    remove_permission_from_user, update_articulo, update_categoria, update_proveedor, update_stock,
    update_sub_categoria, update_user, AppState, ArticuloAppState, AuditLogAppState,
    CategoriaAppState, ProveedorAppState, StockAppState, SubCategoriaAppState, VentaAppState,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = &infrastructure::database::DB;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .manage(AuditLogAppState::new())
        .manage(ProveedorAppState::new())
        .manage(CategoriaAppState::new())
        .manage(SubCategoriaAppState::new())
        .manage(ArticuloAppState::new())
        .manage(StockAppState::new())
        .manage(VentaAppState::new())
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
            delete_categoria,
            get_all_sub_categorias,
            get_sub_categorias_by_categoria,
            create_sub_categoria,
            update_sub_categoria,
            delete_sub_categoria,
            get_all_articulos,
            create_articulo,
            update_articulo,
            delete_articulo,
            get_all_stock,
            get_stock_by_id,
            get_stock_by_articulo,
            create_stock,
            update_stock,
            delete_stock,
            get_precio_venta,
            get_audit_logs,
            create_venta,
            get_all_ventas,
            get_venta_by_id,
            anular_venta
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
