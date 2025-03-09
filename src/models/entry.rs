
struct Entry {
    id: String,
    name: String,
    place: i8,
    current_elo: i32,
    updated_elo: Option<i32>
}

impl Default for Entry {
    fn default() -> Self {
        Self {
            id: "0",
            name: String::from("Unknown"),
            place: 1,
            current_elo: 1000,
            updated_elo: None
        }
    }
}