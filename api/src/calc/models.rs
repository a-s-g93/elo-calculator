use elo_calculator::Entry;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct SingleRequestData {
    pub elo_entries: Vec<Entry>,
    pub k: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MultipleRequestData {
    pub elo_entry_sequence: Vec<Vec<Entry>>,
    pub k: i32,
}
