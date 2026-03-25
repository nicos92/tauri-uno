pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

use api::commands::{
    add_permission_to_user, create_permission, create_user, delete_user, get_all_permissions,
    get_all_users, get_user_permissions, login, remove_permission_from_user, update_user, AppState,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = &infrastructure::database::DB;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
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
            create_permission
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
