
pub struct Entry {
    pub id: String,
    pub name: String,
    pub place: i8,
    pub input_elo: i32,
    pub output_elo: Option<i32>
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            id: String::from("0"),
            name: String::from("Unknown"),
            place: 1,
            input_elo: 1000,
            output_elo: None
        }
    }
}