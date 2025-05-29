use crate::controllers::auth::*;
use crate::middleware::authentication::AuthenticationMiddleware;

use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .service(
                web::scope("")
                    .wrap(AuthenticationMiddleware)
                    .route("/token", web::get().to(read_token))
                    .route("/password", web::patch().to(change_password))
                    .route("/username", web::patch().to(change_username)),
            ),
    );
}
