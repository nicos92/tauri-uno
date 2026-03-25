use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: i64,
    pub permission: String,
    pub created: DateTime<Utc>,
}

impl Permission {
    pub fn new(permission: String) -> Self {
        Self {
            id: 0,
            permission,
            created: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermission {
    pub id: i64,
    pub permission: String,
    pub created: DateTime<Utc>,
    pub assigned_at: DateTime<Utc>,
}
