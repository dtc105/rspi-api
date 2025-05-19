use actix_web::{HttpResponse, Responder, get};
use serde_json::json;

#[get("/api")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "API is alive!";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}
