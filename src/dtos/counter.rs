use crate::models::counter as models;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}
fn default_limit() -> u32 {
    25
}

#[derive(Debug, Deserialize, Serialize)]
pub struct All {
    pub page: u32,
    pub limit: u32,
    pub total: i64,
    pub rows: i64,
    pub data: Vec<models::Count>,
}
