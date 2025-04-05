use actix_web::{get, App, HttpServer};

mod calc;
use calc::services;

#[get("/")]
async fn index() -> String {
    "This is a healthcheck".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(index).configure(services::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
