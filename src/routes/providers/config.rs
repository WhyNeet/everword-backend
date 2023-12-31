use actix_web::web;

use super::{discover::discover, provider::word};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/providers").service(discover).service(word));
}
