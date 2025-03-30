use super::models::{MultipleRequestData, SingleRequestData};
use actix_web::{post, web, HttpResponse, Responder};

use elo_calculator::{
    services::calculate_elos::{update_elos_for_group, update_elos_for_sequence},
    Entry,
};

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
    let mut sequence = input_data.elo_entry_sequence.to_vec();
    let k = input_data.k;

    // Convert sequence of owned entries to sequence of mutable references
    let mut sequence_refs: Vec<Vec<&mut Entry>> = Vec::new();

    // Extract mutable references for each group
    for group in &mut sequence {
        let group_refs: Vec<&mut Entry> = group.iter_mut().collect();
        sequence_refs.push(group_refs);
    }

    // Process the sequence with mutable references
    update_elos_for_sequence(sequence_refs, k);

    // Return the updated sequence of entries
    HttpResponse::Ok().json(sequence)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(calculate_elo_single)
        .service(calculate_elo_multiple);
}
