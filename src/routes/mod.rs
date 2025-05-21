mod auth;
mod health_checker;
mod raspi;

pub mod routes {
    use actix_web::web;

    use super::auth;
    use super::health_checker::health_checker_handler;
    use super::raspi;

    pub fn router(config: &mut web::ServiceConfig) {
        config.service(
            web::scope("/api")
                .route("", web::get().to(health_checker_handler))
                .service(auth::router())
                .service(raspi::router()),
        );
    }
}
