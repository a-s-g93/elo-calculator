use super::models::{MultipleRequestData, SingleRequestData};
use actix_web::{post, web, HttpResponse, Responder};

use elo_calculator::services::calculate_elos::update_elos_for_group;

#[post("/calc/single")]
async fn calculate_elo_single(input_data: web::Json<SingleRequestData>) -> impl Responder {
    // HttpResponse::Ok().json(input_data.elo_entries.to_vec().get(0).unwrap())
    let mut entries = input_data.elo_entries.to_vec();
    let k = input_data.k;
    update_elos_for_group(entries.iter_mut().collect(), k);
    HttpResponse::Ok().json(entries)
}
#[post("/calc/multiple")]
async fn calculate_elo_multiple(input_data: web::Json<MultipleRequestData>) -> impl Responder {
    HttpResponse::Ok().json(input_data.elo_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(calculate_elo_single)
        .service(calculate_elo_multiple);
}
