use actix_web::web;

use super::cambridge::word;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/dict").service(web::scope("/cambridge").service(word)));
}
