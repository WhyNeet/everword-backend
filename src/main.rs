use actix_web::{middleware::Logger, App, HttpServer};
use backend::{
    routes::configure,
    server::{cors::cors, logging},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging::init().expect("failed to initialize logging");

    HttpServer::new(|| {
        App::new()
            .configure(configure)
            .wrap(cors())
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
