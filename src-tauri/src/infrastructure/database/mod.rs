use directories::ProjectDirs;
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

pub static DB: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = init_database().expect("Failed to initialize database");
    Mutex::new(conn)
});

pub fn get_db_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "nicos92", "tauri-uno") {
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir).ok();
        data_dir.join("app.db")
    } else {
        PathBuf::from("app.db")
    }
}

const PERMISSIONS: &[&str] = &[
    "crear_usuario",
    "modificar_usuario",
    "eliminar_usuario",
    "asignar_permiso_a_usuario",
    "quitar_permiso_a_usuario",
    "ver_usuarios",
    "ver_permisos",
    "crear_permiso",
];

pub fn init_database() -> Result<Connection, rusqlite::Error> {
    let db_path = get_db_path();
    let conn = Connection::open(&db_path)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            modified_at TEXT NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            permission TEXT NOT NULL UNIQUE,
            created TEXT NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS user_permissions (
            user_id INTEGER NOT NULL,
            permission_id INTEGER NOT NULL,
            PRIMARY KEY (user_id, permission_id),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
        );
        ",
    )?;

    seed_permissions(&conn)?;
    seed_admin_user(&conn)?;

    Ok(conn)
}

fn seed_permissions(conn: &Connection) -> Result<(), rusqlite::Error> {
    let now = chrono::Utc::now().to_rfc3339();

    for permission in PERMISSIONS {
        conn.execute(
            "INSERT OR IGNORE INTO permissions (permission, created) VALUES (?1, ?2)",
            rusqlite::params![permission, now],
        )?;
    }

    Ok(())
}

fn seed_admin_user(conn: &Connection) -> Result<(), rusqlite::Error> {
    let username = "admin";

    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?1)",
        [username],
        |row| row.get(0),
    )?;

    if !exists {
        let bcrypt_hash =
            bcrypt::hash("admin123", bcrypt::DEFAULT_COST).expect("Failed to hash password");
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO users (username, password, active, created_at, modified_at) VALUES (?1, ?2, 1, ?3, ?3)",
            rusqlite::params![username, bcrypt_hash, now],
        )?;
    }

    let admin_id: i64 = conn.query_row(
        "SELECT id FROM users WHERE username = ?1",
        [username],
        |row| row.get(0),
    )?;

    let mut stmt = conn.prepare("SELECT id FROM permissions")?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let perm_id: i64 = row.get(0)?;
        conn.execute(
            "INSERT OR IGNORE INTO user_permissions (user_id, permission_id) VALUES (?1, ?2)",
            rusqlite::params![admin_id, perm_id],
        )?;
    }

    Ok(())
}
