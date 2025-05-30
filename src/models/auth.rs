use chrono::{DateTime, NaiveDateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        let created_at_str: String = row.get("created_at")?;
        let naive_datetime: NaiveDateTime =
            NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S").map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    4,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

        let created_at: DateTime<Utc> =
            DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc);

        Ok(User {
            id: row.get("id")?,
            username: row.get("username")?,
            password: row.get("password")?,
            role: row.get("role")?,
            created_at,
        })
    }
}

static RE_USERNAME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\-_]*$").unwrap());

static RE_PASSWORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\-_!@#$%^&*()]*$").unwrap());

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 3, max = 32))]
    #[validate(regex(path = *RE_USERNAME))]
    pub username: String,
    #[validate(length(min = 6, max = 1024))]
    #[validate(regex(path = *RE_PASSWORD))]
    pub password: String,
}
