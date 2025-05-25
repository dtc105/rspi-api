use crate::config::database::AppState;
use crate::models::auth::*;

use actix_web::{Error, HttpResponse, Responder, error, web};
use bcrypt::{DEFAULT_COST, hash};
use rusqlite::params;
use serde_json::json;
use validator::Validate;

pub async fn read_token(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn login() -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn register(
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
        return Ok(HttpResponse::BadRequest()
            .json(json!({"error": "BadRequest", "message": "Username taken."})));
    }

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

    Ok(HttpResponse::Ok().json(json!({
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
