use actix_web::{get, web, HttpResponse};
use serde_json::json;

use crate::{
    dict::{cambridge::CambridgeScraper, traits::Scrape},
    server::error::{AppError, AppResult},
};

#[get("/{word}")]
pub async fn word(path: web::Path<String>) -> AppResult<HttpResponse> {
    let defs = CambridgeScraper::scrape(&path)
        .await
        .or(Err(AppError::Internal))?;

    Ok(HttpResponse::Ok().json(json!({ "defentions": defs })))
}
