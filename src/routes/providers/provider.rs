use actix_web::{get, web, HttpResponse};
use serde_json::json;

use crate::server::error::{AppError, AppResult};

use everword_lib::dict::{provider::ProviderFactory, traits::Scrape};

#[get("/{provider}/{word}")]
pub async fn word(path: web::Path<(String, String)>) -> AppResult<HttpResponse> {
    let provider = ProviderFactory::get_provider(&path.0).ok_or(AppError::WrongProviderName)?;

    let defs = Scrape::scrape(provider, &path.1)
        .await
        .or(Err(AppError::Internal))?;

    Ok(HttpResponse::Ok().json(json!({ "defenitions": defs })))
}
