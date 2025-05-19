use std::{borrow::Cow, option::Option as Nullable};
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
    #[validate(length(min=1, max=64))]
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Discount {
    pub id: i64,
    pub option_id: i64,
    pub value: i64,
    pub expires_at: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewDiscount {
    #[validate(range(min=1))]
    pub option_id: i64,
    #[validate(range(min=1, max=100))]
    pub value: i64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub option_id: i64,
    pub count: i64
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewItem {
    #[validate(range(min=1))]
    pub user_id: i64,
    #[validate(range(min=1))]
    pub option_id: i64,
    #[validate(range(min=1))]
    pub count: i64
}

fn non_zero(value: i64) -> Result<(), ValidationError> {
    if value == 0i64 {
        return Err(ValidationError::new("value_error")
            .with_message(Cow::from("Value may not be 0.")));
    }

    Ok(())
}