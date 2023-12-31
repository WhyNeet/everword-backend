use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/discover")]
pub async fn discover() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "providers": ["Cambridge Dictionary"] }))
}
