use actix_web::web;

pub fn router() -> actix_web::Scope {
    web::scope("/counter")
}
