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

    seed_admin_user(&conn)?;

    Ok(conn)
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

    Ok(())
}
