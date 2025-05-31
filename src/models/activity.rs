use chrono::NaiveDateTime;
use std::option::Option as Nullable;

#[derive(Debug)]
pub struct Notification {
    pub id: i64,
    pub subject_id: i64,
    pub issuer_id: i64,
    pub system_id: i64,
    pub item_id: Nullable<i64>,
    pub message: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct Interaction {
    pub id: i64,
    pub user_id: i64,
    pub system_id: i64,
    pub item_id: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct Pinned {
    pub id: i64,
    pub user_id: i64,
    pub system_id: i64,
    pub item_id: i64,
}
