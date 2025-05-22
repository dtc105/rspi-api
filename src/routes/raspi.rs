use crate::controllers::raspi::*;

use actix_web::web;

pub fn router() -> actix_web::Scope {
    web::scope("").route("", web::get().to(get_system_info))
}
