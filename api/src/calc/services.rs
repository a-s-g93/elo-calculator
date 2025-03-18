use super::models::RequestData;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/calc/single")]
async fn calculate_elo_single(input_data: web::Json<RequestData>) -> impl Responder {
    HttpResponse::Ok().json(input_data.elo_entries.to_vec().get(0).unwrap())
}

#[post("/calc/multiple")]
async fn calculate_elo_multiple(input_data: web::Json<RequestData>) -> impl Responder {
    HttpResponse::Ok().json(input_data.elo_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(calculate_elo_single)
        .service(calculate_elo_multiple);
}
