use actix_web::{App, HttpServer};

use resources::application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(application::index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

mod resources;
