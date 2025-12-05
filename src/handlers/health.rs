use actix_web::{HttpResponse, Responder, get};
use serde_json::json;

#[get("/api/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Backend server is running",
        "models": ["flash", "pro"]
    }))
}
