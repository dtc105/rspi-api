use crate::dtos::responses::raspi::*;

use actix_web::{HttpResponse, Responder};
use std::fs;

pub async fn get_system_info() -> impl Responder {
    // Get the current cpu temp and parse it
    let cpu_temp = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .ok()
        .and_then(|temp_str| temp_str.trim().parse::<f32>().ok().map(|f| f / 1000f32))
        .map(FloatOrString::Float)
        .unwrap_or(FloatOrString::String("N/A".to_string()));

    // Get the up time and idle time
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

    HttpResponse::Ok().json(SystemInfo {
        cpu_temp,
        system_uptime: SystemUptime { up_time, idle_time },
    })
}
