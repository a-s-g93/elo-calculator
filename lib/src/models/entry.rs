use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub place: i8,
    pub input_elo: i32,
    pub output_elo: Option<i32>,
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            id: String::from("0"),
            name: String::from("Unknown"),
            place: 1,
            input_elo: 1000,
            output_elo: None,
        }
    }
}

impl Entry {
    pub fn from_cli_input(id: String, place: i8, input_elo: String) -> Result<Self, String> {
        if place < 1 {
            return Err("Place must be greater than 0".to_string());
        }

        let input_elo: i32 = match input_elo.parse() {
            Ok(elo) => elo,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Self {
            id: id.clone(),
            name: String::from(String::from("Entry ") + &id),
            place,
            input_elo,
            ..Default::default()
        })
    }
}
