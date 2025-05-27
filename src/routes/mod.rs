mod auth;
mod counter;
mod raspi;

pub mod routes {
    use super::auth;
    use super::counter;
    use super::raspi;

    use actix_web::web;

    pub fn router(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("")
                .configure(auth::router)
                //.service(counter::router())
                .configure(raspi::router),
        );
    }
}
