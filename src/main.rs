use models::entry::Entry;
use services::calculate_elos;
use std::env;
use std::process;

mod models {
    pub mod entry;
}

mod services {
    pub mod calculate_elos;
}


fn main() {
    // run_demo();

    let mut entries = match parse_inputs() {
        Ok(entries) => entries,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };
    let entry_refs = entries.iter_mut().collect();

    calculate_elos::update_elos_for_group(entry_refs, 32);

    for entry in entries {
        println!("{}: {} -> {}", entry.name, entry.input_elo, entry.output_elo.unwrap_or(0));
    }
}


fn parse_inputs() -> Result<Vec<Entry>, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err("Must provide more than 1 Elo to compute Elo rating changes".to_string());
    };
    
    let mut entries: Vec<Entry> = vec![];
    
    for i in 1..args.len() {
        let entry = match Entry::from_cli_input(String::from(i.to_string()), i as i8, args[i].clone()) {
            Ok(ent) => ent,
            Err(e) => return Err(e)
        };

        entries.push(entry);
    }

    Ok(entries)
}



/// Demo placeholder
#[allow(dead_code)]
fn run_demo() -> () {
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

    let entries = vec![&mut entry1, &mut entry2, &mut entry3];

    calculate_elos::update_elos_for_group(entries, 32);

    println!(
        "Updated {} elo: {} -> {}",
        entry1.name,
        entry1.input_elo,
        entry1.output_elo.unwrap_or(0)
    );
    println!(
        "Updated {} elo: {} -> {}",
        entry2.name,
        entry2.input_elo,
        entry2.output_elo.unwrap_or(0)
    );
    println!(
        "Updated {} elo: {} -> {}",
        entry3.name,
        entry3.input_elo,
        entry3.output_elo.unwrap_or(0)
    );
}
