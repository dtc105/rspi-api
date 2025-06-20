use crate::utils::string;
use rusqlite::{Error, Row};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DataLinks {
    #[serde(rename = "self")]
    pub own: String,
}

#[derive(Debug, Serialize)]
pub struct Data {
    pub username: String,
    pub word: String,
    pub count: u32,
    pub similarity: Option<f32>,
}

impl Data {
    pub fn from_row(
        row: &Row,
        username: &Option<String>,
        word: &Option<String>,
    ) -> Result<Self, Error> {
        let row_username: String = row.get("username")?;
        let row_word: String = row.get("word")?;
        let similarity: Option<f32>;

        if let Some(u) = username {
            similarity = Some(string::similarity(&row_username, u));
        } else if let Some(w) = word {
            similarity = Some(string::similarity(&row_word, w));
        } else {
            similarity = None;
        }

        Ok(Self {
            username: row_username,
            word: row_word,
            count: row.get("count")?,
            similarity: similarity,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,

    #[serde(rename = "totalRows")]
    pub total_rows: u32,

    #[serde(rename = "totalPages")]
    pub total_pages: u32,

    #[serde(rename = "hasNext")]
    pub has_next: bool,

    #[serde(rename = "hasPrev")]
    pub has_prev: bool,
}

#[derive(Debug, Serialize)]
pub struct Filters {
    pub username: Option<String>,
    pub word: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Sort {
    pub by: String,
    pub order: String,
}

#[derive(Debug, Serialize)]
pub struct Meta {
    pub pagination: Pagination,
    pub filters: Filters,
    pub sort: Sort,
}

#[derive(Debug, Serialize)]
pub struct PartMeta {
    pub pagination: Pagination,
    pub sort: Sort,
}

#[derive(Debug, Serialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub own: String,
    pub first: String,
    pub last: String,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub data: Vec<Data>,
    pub meta: Meta,
    pub links: Links,
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub username: String,
    pub count: u32,
}

impl UserData {
    pub fn from_row(row: &Row) -> Result<Self, Error> {
        let username: String = row.get("username")?;
        Ok(Self {
            username: username.clone(),
            count: row.get("total")?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub data: Vec<UserData>,
    pub meta: PartMeta,
    pub links: Links,
}

#[derive(Debug, Serialize)]
pub struct WordData {
    pub word: String,
    pub count: u32,
}

impl WordData {
    pub fn from_row(row: &Row) -> Result<Self, Error> {
        let word: String = row.get("word")?;
        Ok(Self {
            word: word.clone(),
            count: row.get("total")?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct WordResponse {
    pub data: Vec<WordData>,
    pub meta: PartMeta,
    pub links: Links,
}
