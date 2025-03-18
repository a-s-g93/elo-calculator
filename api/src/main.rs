use actix_web::{get, App, HttpServer};

mod calc;
use calc::services;

// struct AppState {
//     elo_entries: Mutex<Vec<Entry>>
// }

#[get("/")]
async fn index() -> String {
    "This is a healthcheck".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let app_data = web::Data::new(AppState{
    //     elo_entries: Mutex::new(vec![])
    // });

    HttpServer::new(move || App::new().service(index).configure(services::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
