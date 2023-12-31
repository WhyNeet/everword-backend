use actix_web::web;

use super::{cambridge::word, discover::discover};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/providers")
            .service(discover)
            .service(web::scope("/cambridge").service(word)),
    );
}
