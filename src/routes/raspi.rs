use std::fs;

use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

pub fn router() -> actix_web::Scope {
    web::scope("").route("", web::get().to(get_system_info))
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum FloatOrString {
    Float(f32),
    String(String),
}

#[derive(Deserialize, Serialize)]
struct SystemUptime {
    #[serde(rename = "upTime")]
    up_time: FloatOrString,
    #[serde(rename = "idle_time")]
    idle_time: FloatOrString,
}

#[derive(Deserialize, Serialize)]
struct SystemInfo {
    #[serde(rename = "cpuTemp")]
    cpu_temp: FloatOrString,
    #[serde(rename = "systemUptime")]
    system_uptime: SystemUptime,
}

async fn get_system_info() -> impl Responder {
    let cpu_temp = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .ok()
        .and_then(|temp_str| temp_str.trim().parse::<f32>().ok().map(|f| f / 1000f32))
        .map(FloatOrString::Float)
        .unwrap_or(FloatOrString::String("N/A".to_string()));

    let uptime_data = fs::read_to_string("/proc/uptime").unwrap_or_default();
    let mut uptime_part = uptime_data.trim().split_whitespace();

    let up_time = uptime_part
        .next()
        .and_then(|time| time.trim().parse::<f32>().ok())
        .map(FloatOrString::Float)
        .unwrap_or(FloatOrString::String("N/A".to_string()));

    let idle_time = uptime_part
        .next()
        .and_then(|time| time.trim().parse::<f32>().ok())
        .map(FloatOrString::Float)
        .unwrap_or(FloatOrString::String("N/A".to_string()));

    let system_info = SystemInfo {
        cpu_temp,
        system_uptime: SystemUptime { up_time, idle_time },
    };

    HttpResponse::Ok().json(system_info)
}
