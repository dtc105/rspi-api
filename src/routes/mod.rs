mod auth;
mod raspi;

pub mod routes {
    use actix_web::web;

    use super::auth;
    use super::raspi;

    pub fn router(config: &mut web::ServiceConfig) {
        config.service(
            web::scope("/api")
                .service(auth::router())
                .service(raspi::router()),
        );
    }
}
