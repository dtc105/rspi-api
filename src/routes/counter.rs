use crate::controllers::counter::*;
use crate::middleware::authentication::AuthenticationMiddleware;

use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/counter")
            .wrap(AuthenticationMiddleware)
            .route("", web::get().to(get_all))
            .route("/users", web::get().to(get_all_users))
            .route("/words", web::get().to(get_all_words)),
    );
}
