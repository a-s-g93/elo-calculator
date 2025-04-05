use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    #[pyo3(get, set)]
    pub id: String,
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub place: i8,
    #[pyo3(get, set)]
    pub input_elo: Option<i32>,
    #[pyo3(get, set)]
    pub output_elo: Option<i32>,
}

#[pymethods]
impl Entry {
    #[new]
    pub fn new(id: String, name: String, place: i8, input_elo: Option<i32>) -> Self {
        Self {
            id,
            name,
            place,
            input_elo,
            output_elo: None,
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Entry(id='{}', name='{}', place={}, input_elo={:?}, output_elo={:?})",
            self.id, self.name, self.place, self.input_elo, self.output_elo
        )
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            id: String::from("0"),
            name: String::from("Unknown"),
            place: 1,
            input_elo: None,
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
            name: String::from("Entry ") + &id,
            place,
            input_elo: Some(input_elo),
            ..Default::default()
        })
    }
}
