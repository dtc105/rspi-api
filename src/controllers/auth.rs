use crate::models::auth::*;
use crate::{config::database::AppState, middleware::authentication::Claims};

use actix_web::{Error, HttpResponse, Responder, cookie::Cookie, error, web};
use actix_web::{HttpMessage, HttpRequest};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use rusqlite::{OptionalExtension, params};
use serde_json::json;
use validator::Validate;

fn sign_token(id: &i64, role: &String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret: String =
        std::env::var("JWT_SECRET").expect("`JWT_SECRET` must be defined in `.env`.");

    let iat: i64 = Utc::now().timestamp();
    let exp: i64 = iat + Duration::days(30).num_seconds();

    let claims: Claims = Claims {
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

/// Reads the users token
///
/// # Route
/// `GET /auth/token`
///
/// # Responses
/// - `200 Ok`: Returns user data
/// - `401 Unauthorized`: If there is no token
/// - `500 Internal Server Error`: Server sided error
///
/// # Example Request
/// `GET /auth/login`
///
/// # Example Response 200
/// ```
/// {
///     "id": 123,
///     "role": "user"
/// }
/// ```
pub async fn read_token(req: HttpRequest) -> Result<HttpResponse, Error> {
    let ext = req.extensions();
    let claims = ext
        .get::<Claims>()
        .ok_or_else(|| error::ErrorUnauthorized("No token."))?;

    Ok(HttpResponse::Ok().json(json!({
        "id": &claims.sub,
        "role": &claims.role
    })))
}

/// Handles user login and assigns a JWT
///
/// # Route
/// `POST /auth/login`
///
/// # Request Body
/// - `username`: The username the client is logging in with (3-32 chars)
/// - `password`: The password the client is logging in with (6+ chars)
///
/// # Responses
/// - `200 Ok`: Returns user data
/// - `400 Bad Request`: If missing or invalid parameters
/// - `401 Conflict`: If the username or password is incorrect
/// - `500 Internal Server Error`: Server sided error
///
/// # Example Request
/// `POST /auth/login`
///
/// # Example Request Body
/// ```
/// {
///     "username": "JohnDoe123",
///     "password" "password"
/// }
/// ```
///
/// # Example Response 200
/// ```
/// {
///     "id": 123,
///     "username": "JohnDoe123",
///     "role": "user"
/// }
/// ```
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

    let query: Option<User> = conn
        .query_row(
            r#"
            SELECT *
            FROM users
            WHERE username = ?1;
            "#,
            [&body.username],
            |row| User::from_row(row),
        )
        .optional()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    match query {
        Some(user) => {
            // Check the password
            let is_password_correct: bool = verify(&body.password, &user.password)
                .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

            if !is_password_correct {
                return Ok(HttpResponse::Unauthorized().json(json!({
                    "error": "Unauthorized",
                    "message": "Incorrect username or password."
                })));
            }

            // Sign a token
            let token: String = sign_token(&user.id, &user.role)
                .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

            let cookie = Cookie::build("Authorization", token.clone())
                .http_only(true)
                .secure(false)
                .path("/")
                .finish();

            return Ok(HttpResponse::Ok().cookie(cookie).json(json!({
                "id": &user.id,
                "username": &user.username,
                "role": &user.role
            })));
        }
        None => {
            return Ok(HttpResponse::NotFound().json(json!({
                "error": "NotFound",
                "message": "User not found."
            })));
        }
    };
}

/// Handles user registration and assigns a JWT
///
/// # Route
/// `POST /auth/register`
///
/// # Request Body
/// - `username`: The username the client is registering with (3-32 chars)
/// - `password`: The password the client is registering with (6+ chars)
///
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
            |row| row.get::<usize, i64>(0),
        )
        .optional()
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?
        .is_some();

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

    let cookie = Cookie::build("Authorization", token.clone())
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
