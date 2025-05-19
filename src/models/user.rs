
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub role: String,
    pub created_at: chrono::Date(chrono::Utc),
}
