use actix_web::{App, HttpServer};
use backend::{routes::configure, server::cors::cors};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configure).wrap(cors()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
