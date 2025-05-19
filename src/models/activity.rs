use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, option::Option as Nullable};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Serialize)]
pub struct Notification {
    pub id: i64,
    pub subject_id: i64,
    pub issuer_id: i64,
    pub system_id: i64,
    pub item_id: Nullable<i64>,
    pub message: String,
    pub status: String,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewNotification {
    #[validate(range(min=1))]
    pub subject_id: i64,
    #[validate(range(min=1))]
    pub issuer_id: i64,
    #[validate(range(min=1))]
    pub system_id: i64,
    #[validate(range(min=1))]
    pub item_id: Nullable<i64>,
    #[validate(length(min=1, max=256))]
    pub message: String,
    #[validate(custom(function="validate_notification_status"))]
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Interaction {
    pub id: i64,
    pub user_id: i64,
    pub system_id: i64,
    pub item_id: i64,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewInteraction {
    #[validate(range(min=1))]
    pub user_id: i64,
    #[validate(range(min=1))]
    pub system_id: i64,
    #[validate(range(min=1))]
    pub item_id: i64
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Pinned {
    pub id: i64,
    pub user_id: i64,
    pub system_id: i64,
    pub item_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewPinned {
    #[validate(range(min=1))]
    pub user_id: i64,
    #[validate(range(min=1))]
    pub system_id: i64,
    #[validate(range(min=1))]
    pub item_id: i64
}

fn validate_notification_status(status: &str) -> Result<(), ValidationError> {
    let valid_statuses = ["delivered", "seen", "accepted", "rejected", "ignored"];
    if valid_statuses.contains(&status) {
        return Ok(());
    }

    Err(ValidationError::new("value_error")
        .with_message(Cow::from("Notification status invalid.  Valid statuses: [\"delivered\", \"seen\", \"accepted\", \"rejected\", \"ignored\"]")))   
}

