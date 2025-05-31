use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

static RE_USERNAME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\-_]*$").unwrap());

static RE_PASSWORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9\-_!@#$%^&*()]*$").unwrap());

#[derive(Debug, Deserialize, Validate)]
pub struct Credentials {
    #[validate(length(min = 3, max = 32))]
    #[validate(regex(path = *RE_USERNAME))]
    pub username: String,
    #[validate(length(min = 6, max = 128))]
    #[validate(regex(path = *RE_PASSWORD))]
    pub password: String,
}
