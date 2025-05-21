use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "API is alive!"}))
}
