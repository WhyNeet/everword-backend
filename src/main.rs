use actix_web::{App, HttpServer};
use backend::routes::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configure))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
