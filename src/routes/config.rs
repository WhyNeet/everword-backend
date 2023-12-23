use actix_web::web;

use super::{dict::configure as configure_dict, healthcheck::healthcheck};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(healthcheck)
            .configure(configure_dict),
    );
}
