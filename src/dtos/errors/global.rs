use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Generic {
    pub error: String,
    pub message: String,
}
