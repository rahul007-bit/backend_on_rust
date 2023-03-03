use actix_web::HttpResponse;
use serde_json::json;

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "status": "404",
        "message": "Not found"
    }))
}
