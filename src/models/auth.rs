use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 8, max = 1024))]
    pub password: String,
}

fn validate_role(role: &str) -> Result<(), ValidationError> {
    if ["admin", "moderator", "user"].contains(&role) {
        return Ok(());
    }

    Err(ValidationError::new("value_error").with_message(Cow::from(
        "Role invalid.  Valid roles: [\"admin\", \"moderator\", \"user\"]",
    )))
}
