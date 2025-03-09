

use std::collections::HashMap;

use crate::models::entry::*;

fn calculate_elo_change_for_pair(entry_one: &Entry, entry_two: &Entry) -> (f32, f32) {
    let base: f32 = 10.0;

    let r1 = base.powf(entry_one.input_elo as f32 / 400.0);
    let r2 = base.powf(entry_two.input_elo as f32 / 400.0);

    let e1 = r1 / (r1 + r2);
    let e2 = r2 / (r1 + r2);

    let s1: f32;
    let s2: f32;

    if entry_one.place < entry_two.place {
        s1 = 1.0;
        s2 = 0.0;
    } else if entry_one.place > entry_two.place {
        s1 = 0.0;
        s2 = 1.0;
    } else {
        s1 = 0.5;
        s2 = 0.5;
    }

    (s1 - e1, s2 - e2)

}

fn calculate_elo_change_for_group(entries: Vec<&Entry>) -> HashMap<&str, f32> {
    todo!()
}

fn update_elos_for_group(entries: Vec<&mut Entry>) -> Vec<&mut Entry> {
    todo!()
}



#[cfg(test)]
mod tests {
    use super::*;

    fn create_player_1() -> Entry {
        Entry { 
            id: String::from("1"),
            input_elo: 1020,
            place: 1,
            ..Default::default()
        }
    }

    fn create_player_2() -> Entry {
        Entry { 
            id: String::from("2"),
            input_elo: 900,
            place: 2,
            ..Default::default()
        }
    }

    fn create_player_3() -> Entry {
        Entry { 
            id: String::from("3"),
            input_elo: 800,
            place: 3,
            ..Default::default()
        }
    }

    fn create_player_4() -> Entry {
        Entry { 
            id: String::from("4"),
            input_elo: 1000,
            place: 4,
            ..Default::default()
        }
    }

    fn create_player_5() -> Entry {
        Entry { 
            id: String::from("5"),
            input_elo: 1000,
            place: 5,
            ..Default::default()
        }
    }

    fn create_player_6() -> Entry {
        Entry { 
            id: String::from("6"),
            input_elo: 1400,
            place: 6,
            ..Default::default()
        }
    }

    fn create_player_7() -> Entry {
        // for draw testing against player 1
        Entry { 
            id: String::from("7"),
            input_elo: 1600,
            place: 1,
            ..Default::default()
        }
    }
   

    #[test]
    fn test_calculate_elo_change_for_pair_equal_elos() {
        let left = create_player_4();
        let right = create_player_5();

        let delta = calculate_elo_change_for_pair(&left, &right);

        assert_eq!(delta.0, 0.5);
        assert_eq!(delta.1, -0.5);
    }

    #[test]
    fn test_calculate_elo_change_for_pair_small_elo_difference() {
        let left = create_player_1();
        let right = create_player_4();

        let answer: f32 = 0.47124946;

        let delta = calculate_elo_change_for_pair(&left, &right);

        assert!(delta.0 - answer < 0.0001);
        assert!(delta.1 + answer < 0.0001);
    }


    #[test]
    fn test_calculate_elo_change_for_pair_large_elo_difference() {
        let left = create_player_3();
        let right = create_player_6();

        let answer: f32 = 0.9693466;

        let delta = calculate_elo_change_for_pair(&left, &right);

        assert!(delta.0 - answer < 0.0001);
        assert!(delta.1 + answer < 0.0001);
    }

    #[test]
    fn test_calculate_elo_change_for_tie() {
        let left = create_player_1();
        let right = create_player_7();

        let answer: f32 = 0.46573445;

        let delta = calculate_elo_change_for_pair(&left, &right);

        assert!(delta.0 - answer < 0.0001);
        assert!(delta.1 + answer < 0.0001);
    }

    #[test]
    fn test_calculate_elo_change_for_group_size_2() {}

    #[test]
    fn test_calculate_elo_change_for_group_size_3() {}

    #[test]
    fn test_calculate_elo_change_for_group_size_4() {}

    #[test]
    fn test_calculate_elo_change_for_group_size_6() {}

    #[test]
    fn test_update_elos_for_group_size_2() {}

    #[test]
    fn test_update_elos_for_group_size_3() {}

    #[test]
    fn test_update_elos_for_group_size_4() {}

    #[test]
    fn test_update_elos_for_group_size_6() {}

}