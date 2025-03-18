use elo_calculator::Entry;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct RequestData {
    pub elo_entries: Vec<Entry>,
}
