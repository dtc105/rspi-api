use crate::config::database::AppState;

use actix_web::{HttpResponse, Responder, web};

pub async fn read_token(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn login() -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn register() -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn change_password() -> impl Responder {
    HttpResponse::NotImplemented()
}

pub async fn change_username() -> impl Responder {
    HttpResponse::NotImplemented()
}
