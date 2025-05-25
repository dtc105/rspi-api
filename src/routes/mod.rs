mod auth;
mod counter;
mod raspi;

pub mod routes {
    use actix_web::web;

    use super::auth;
    use super::counter;
    use super::raspi;

    pub fn router(config: &mut web::ServiceConfig) {
        config.service(
            web::scope("")
                .service(auth::router())
                .service(counter::router())
                .service(raspi::router()),
        );
    }
}
