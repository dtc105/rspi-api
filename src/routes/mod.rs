mod auth;
mod counter;
mod raspi;

use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .configure(auth::router)
            .configure(counter::router)
            .configure(raspi::router),
    );
}
