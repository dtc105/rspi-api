use crate::controllers::auth::*;

use actix_web::web;

pub fn router() -> actix_web::Scope {
    web::scope("/auth")
        .route("/token", web::get().to(read_token))
        .route("/login", web::post().to(login))
        .route("/register", web::post().to(register))
        .route("/password", web::patch().to(change_password))
        .route("/username", web::patch().to(change_username))
}
