use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub _id: String,
    pub username: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub created_at: u64,
}