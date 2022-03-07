use actix_web::{App, HttpServer};

use resources::application;

mod resources;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(application::create_application))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}