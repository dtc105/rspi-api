use crate::models::auth::*;
use crate::{config::database::AppState, middleware::authentication::Claims};

use actix_web::cookie::Cookie;
use actix_web::error::ErrorBadRequest;
use actix_web::{Error, HttpResponse, Responder, error, web};
use bcrypt::{DEFAULT_COST, hash};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use rusqlite::params;
use serde_json::json;
use validator::Validate;

fn sign_token(id: &i64, role: &String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("`JWT_SECRET` must be defined in `.env`.");

    let iat = Utc::now();
    let exp = iat + Duration::days(30);

    let claims = Claims {
        sub: id.to_owned(),
        role: role.to_owned(),
        iat,
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub async fn read_token(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn login(
    body: web::Json<NewUser>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Validate body
    body.validate()
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    let conn = state
        .pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    Ok(HttpResponse::NotImplemented().finish())
}

/// Handles user registration and assigns a JWT
///
/// # Route
/// `POST /auth/register`
///
/// # Request Body
/// - `username`: The username the client is registering with (3-32 chars)
/// - `password`: The password the client is registering with (8+ chars)
///
/// # Responses
/// - `201 Created`: Returns registered user data
/// - `400 Bad Request`: If missing or invalid parameters
/// - `409 Conflict`: If the username is already taken
/// - `500 Internal Server Error`: Server sided error
///
/// # Example Request
/// `POST /auth/register`
///
/// # Example Request Body
/// ```
/// {
///     "username": "JohnDoe123",
///     "password" "password"
/// }
/// ```
///
/// # Example Response 201
/// ```
/// {
///     "id": 123,
///     "username": "JohnDoe123",
///     "role": "user"
/// }
/// ```
pub async fn register(
    body: web::Json<NewUser>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // Validate body
    body.validate()
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    // Connect to the database
    let conn = state
        .pool
        .get()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    // Check if the username already exists
    let user_exists: bool = conn
        .query_row(
            r#"
            SELECT 1
            FROM users
            WHERE username = ?1;
            "#,
            [&body.username],
            |row| row.get(0),
        )
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    if user_exists {
        return Ok(HttpResponse::Conflict()
            .json(json!({"error": "BadRequest", "message": "Username taken."})));
    }

    // Hash the password and insert it into the database
    let password_hash = hash(&body.password, DEFAULT_COST)
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    conn.execute(
        r#"
        INSERT INTO users(username, password, role)
        VALUES (?1, ?2, "user");
        "#,
        params![&body.username, password_hash],
    )
    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let token = sign_token(&conn.last_insert_rowid(), &"user".to_string())
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let cookie = Cookie::build("token", token.clone())
        .http_only(true)
        .secure(false)
        .path("/")
        .finish();

    Ok(HttpResponse::Created().cookie(cookie).json(json!({
        "id": conn.last_insert_rowid(),
        "username": &body.username,
        "role": "user"
    })))
}

pub async fn change_password() -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn change_username() -> impl Responder {
    HttpResponse::NotImplemented()
}
