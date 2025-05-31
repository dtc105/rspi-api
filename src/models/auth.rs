use chrono::NaiveDateTime;
use rusqlite::{Error, Row, types::Type};

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn from_row(row: &Row) -> Result<Self, Error> {
        let created_at_str: String = row.get("created_at")?;
        let created_at: NaiveDateTime =
            NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
                .map_err(|e| Error::FromSqlConversionFailure(4, Type::Text, Box::new(e)))?;

        Ok(User {
            id: row.get("id")?,
            username: row.get("username")?,
            password: row.get("password")?,
            role: row.get("role")?,
            created_at,
        })
    }
}
