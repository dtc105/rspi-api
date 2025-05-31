use rusqlite::{Error, Row};

#[derive(Debug)]
pub struct Count {
    pub id: i64,
    pub username: String,
    pub word: String,
    pub count: i64,
}

impl Count {
    fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(Count {
            id: row.get("id")?,
            username: row.get("username")?,
            word: row.get("word")?,
            count: row.get("count")?,
        })
    }
}
