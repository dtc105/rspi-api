use std::option::Option as Nullable;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    pub id: i64,
    pub owner_id: i64,
    pub title: String,
    pub currency: String,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewStore {
    #[validate(range(min=1))]
    pub owner_id: i64,
    #[validate(length(min=1, max=64))]
    pub title: String,
    #[validate(length(min=1, max=32))]
    pub currency: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub user_id: i64,
    pub store_id: i64,
    pub balance: i64,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewUser {
    #[validate(range(min=1))]
    pub user_id: i64,
    #[validate(range(min=1))]
    pub store_id: i64,
    #[validate(range(min=0))]
    pub balance: i64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Option {
    pub id: i64,
    pub store_id: i64,
    pub value: i64,
    pub title: String,
    pub description: Nullable<String>,
    pub stock: Nullable<i64>,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewOption {
    #[validate(range(min=1))]
    pub store_id: i64,
    pub value: i64,
    #[validate(length(min=1, max=64))]
    pub title: String,
    #[validate(length(min=1, max=256))]
    pub description: Nullable<String>,
    #[validate(range(min=0))]
    pub stock: Nullable<i64>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub id: i64,
    pub store_id: i64,
    pub user_id: i64,
    pub value: i64,
    pub discount: i64,
    pub title: String,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewTransaction {
    #[validate(range(min=1))]
    pub store_id: i64,
    #[validate(range(min=1))]
    pub user_id: i64,
    #[validate(custom(function="non_zero"))]
    pub value: i64,
    #[validate(range(min=1))]
    pub discount: i64,
    pub title: String,
}

fn non_zero(value: i64) -> Result<(), ValidationError> {
    todo!()
}

