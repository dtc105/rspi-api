use std::fs;

use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

pub fn router() -> actix_web::Scope {
    web::scope("/raspi")
        .route("", web::get().to(get_system_info))
        .route("/processes", web::get().to(get_processes))
}

#[derive(Deserialize, Serialize)]
struct SystemInfo {
    #[serde(rename = "cpuTemp")]
    cpu_temp: String,
}

async fn get_system_info() -> impl Responder {
    let temperature = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .and_then(|temp_str| match temp_str.trim().parse::<i32>() {
            Ok(temp) => Ok(format!("{:.2}", temp as f64 / 1000f64)),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        })
        .unwrap_or("N/A".to_string());

    todo!();

    HttpResponse::Ok()
}

async fn get_processes() -> impl Responder {
    HttpResponse::Ok()
}
