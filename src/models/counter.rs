use rusqlite::{Error, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Count {
    pub username: String,
    pub word: String,
    pub count: i64,
}

impl Count {
    pub fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(Count {
            username: row.get("username")?,
            word: row.get("word")?,
            count: row.get("count")?,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Summary {
    pub total: i64,
    pub rows: i64,
}

impl Summary {
    pub fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(Summary {
            total: row.get("total")?,
            rows: row.get("rows")?,
        })
    }
}
