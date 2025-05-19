use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Count {
    pub id: i64,
    pub username: String,
    pub word: String,
    pub count: i64
}   