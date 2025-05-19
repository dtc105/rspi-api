use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
pub struct System {
    pub id: i64,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewSystem {
    #[validate(length(min=1, max=128))]
    pub name: String
}