use crate::models::auth::User;

use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: i64,
    pub username: String,
    pub role: String,
}

impl From<User> for Response {
    fn from(user: User) -> Self {
        Response {
            id: user.id,
            username: user.username,
            role: user.role,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TimestampResponse {
    pub id: i64,
    pub username: String,
    pub role: String,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl From<User> for TimestampResponse {
    fn from(user: User) -> Self {
        TimestampResponse {
            id: user.id,
            username: user.username,
            role: user.role,
            created_at: user.created_at,
        }
    }
}
