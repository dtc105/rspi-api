use chrono::NaiveDateTime;
use std::option::Option as Nullable;

#[derive(Debug)]
pub struct Store {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub currency: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub user_id: i64,
    pub store_id: i64,
    pub balance: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct Option {
    pub id: i64,
    pub store_id: i64,
    pub value: i64,
    pub title: String,
    pub description: Nullable<String>,
    pub stock: Nullable<i64>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct Transaction {
    pub id: i64,
    pub store_id: i64,
    pub user_id: i64,
    pub value: i64,
    pub discount: i64,
    pub title: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct Discount {
    pub id: i64,
    pub option_id: i64,
    pub value: i64,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub option_id: i64,
    pub count: i64,
}
