use crate::controllers::raspi::*;

use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(get_system_info));
}
