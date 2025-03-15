use models::entry::Entry;
use services::calculate_elos;

mod models {
    pub mod entry;
}

mod services {
    pub mod calculate_elos;
}

fn main() {
    // Create individual mutable Entry variables
    let mut entry1 = Entry { 
        id: String::from("1"),
        input_elo: 1020,
        name: String::from("Donkey Kong"),
        place: 1,
        ..Default::default()
    };
    let mut entry2 = Entry { 
        id: String::from("2"),
        input_elo: 1100,
        name: String::from("Mario"),
        place: 2,
        ..Default::default()
    };
    let mut entry3 = Entry { 
        id: String::from("3"),
        name: String::from("Toad"),
        input_elo: 800,
        place: 3,
        ..Default::default()
    };

    // Create a vector of mutable references
    let entries = vec![&mut entry1, &mut entry2, &mut entry3];
    
    // Pass the vector of mutable references to the function
    calculate_elos::update_elos_for_group(entries, 32);
    
    // You can also access the updated entries directly
    println!("Updated {} elo: {} -> {}", entry1.name, entry1.input_elo, entry1.output_elo.unwrap_or(0));
    println!("Updated {} elo: {} -> {}", entry2.name, entry2.input_elo, entry2.output_elo.unwrap_or(0));
    println!("Updated {} elo: {} -> {}", entry3.name, entry3.input_elo, entry3.output_elo.unwrap_or(0));
}
