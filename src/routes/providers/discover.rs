use actix_web::{get, HttpResponse};
use everword_lib::dict::provider::ProviderFactory;
use serde_json::json;

#[get("/discover")]
pub async fn discover() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "providers": ProviderFactory::get_all() }))
}
