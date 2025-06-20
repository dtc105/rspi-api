use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

static RE_STRING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\-_]*$").unwrap());

#[derive(Debug, Deserialize, Validate)]
pub struct QueryParams {
    #[validate(range(min = 1))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u32>,

    #[validate(contains(pattern = "asc|desc"))]
    #[validate(regex(path = *RE_STRING))]
    pub order: Option<String>,

    #[validate(length(min = 3, max = 32))]
    #[validate(regex(path = *RE_STRING))]
    pub username: Option<String>,

    #[validate(length(min = 1, max = 2000))]
    #[validate(regex(path = *RE_STRING))]
    pub word: Option<String>,
}

pub struct SetQueryParams {
    pub page: u32,
    pub limit: u32,
    pub order: String,
    pub username: Option<String>,
    pub word: Option<String>,
}

impl From<QueryParams> for SetQueryParams {
    fn from(query: QueryParams) -> Self {
        Self {
            page: query.page.unwrap_or(1),
            limit: query.limit.unwrap_or(10),
            order: query.order.unwrap_or("desc".to_string()),
            username: query.username,
            word: query.word,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct QueryPagination {
    #[validate(range(min = 1))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 100))]
    pub limit: Option<u32>,

    #[validate(contains(pattern = "asc|desc"))]
    #[validate(regex(path = *RE_STRING))]
    pub order: Option<String>,
}

pub struct SetQueryPagination {
    pub page: u32,
    pub limit: u32,
    pub order: String,
}

impl From<QueryPagination> for SetQueryPagination {
    fn from(query: QueryPagination) -> Self {
        Self {
            page: query.page.unwrap_or(1),
            limit: query.limit.unwrap_or(10),
            order: query.order.unwrap_or("desc".to_string()),
        }
    }
}
