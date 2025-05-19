use actix_web::{HttpResponse, Responder, get};
use serde_json::json;

pub fn router() {
    todo!()
}

#[get("/api")]
async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "API is alive!"}))
}
