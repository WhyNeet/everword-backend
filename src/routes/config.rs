use actix_web::web;

use super::{healthcheck::healthcheck, providers::configure as configure_providers};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(healthcheck)
            .configure(configure_providers),
    );
}
