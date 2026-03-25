use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            username,
            password,
            active: true,
            created_at: now,
            modified_at: now,
        }
    }
}
