use actix_web::{HttpResponse, Responder, web};

use crate::config::database::AppState;

pub fn router() -> actix_web::Scope {
    web::scope("/auth")
        .route("/token", web::get().to(read_token))
        .route("/login", web::post().to(login))
        .route("/register", web::post().to(register))
        .route("/password", web::patch().to(change_password))
        .route("/username", web::patch().to(change_username))
}

async fn read_token(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn login() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn register() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn change_password() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn change_username() -> impl Responder {
    HttpResponse::NotImplemented()
}
