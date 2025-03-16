//! Elo rating calculation service module
//!
//! This module provides functionality for calculating and updating Elo ratings
//! for a group of players based on their performance.

use crate::models::entry::Entry;
use std::collections::HashMap;

/// Updates Elo ratings for a group of players based on their places.
///
/// # Arguments
/// * `entries` - A vector of mutable references to Entry structs containing player data
/// * `k` - The K-factor used in Elo calculations
///
/// # Returns
/// A vector of mutable references to the updated Entry structs
pub fn update_elos_for_group(mut entries: Vec<&mut Entry>, k: i32) -> Vec<&mut Entry> {
    // First collect all the data we need for calculation
    let computation_inputs: Vec<(String, i32, i8)> = entries
        .iter()
        .map(|e| (e.id.clone(), e.input_elo, e.place))
        .collect();

    // Calculate Elo changes
    let elo_changes = calculator::calculate_elo_change_for_group(computation_inputs, k);

    // Update each entry's Elo rating
    for entry in entries.iter_mut() {
        if let Some(&change) = elo_changes.get(&entry.id) {
            entry.output_elo = Some(entry.input_elo + change);
        }
    }

    entries
}

/// Private module containing the Elo calculation implementation
mod calculator {
    use super::*;

    /// Calculate Elo change between two players
    #[allow(dead_code)]
    pub(crate) fn calculate_elo_change_for_pair(
        entry_one: (i32, i8),
        entry_two: (i32, i8),
    ) -> (f32, f32) {
        let base: f32 = 10.0;

        let r1 = base.powf(entry_one.0 as f32 / 400.0);
        let r2 = base.powf(entry_two.0 as f32 / 400.0);

        let e1 = r1 / (r1 + r2);
        let e2 = r2 / (r1 + r2);

        let s1: f32;
        let s2: f32;

        if entry_one.1 < entry_two.1 {
            s1 = 1.0;
            s2 = 0.0;
        } else if entry_one.1 > entry_two.1 {
            s1 = 0.0;
            s2 = 1.0;
        } else {
            s1 = 0.5;
            s2 = 0.5;
        }

        (s1 - e1, s2 - e2)
    }

    /// Calculate Elo changes for a group of players
    #[allow(dead_code)]
    pub(crate) fn calculate_elo_change_for_group(
        entries: Vec<(String, i32, i8)>,
        k: i32,
    ) -> HashMap<String, i32> {
        let mut r_map: HashMap<String, f32> = entries.iter().map(|e| (e.0.clone(), 0.0)).collect();
        let id_list: Vec<String> = entries.iter().map(|e| e.0.clone()).collect();
        let group_size = id_list.len();

        for i in 0..group_size {
            let i_id = id_list.get(i).expect("Entry.id should never be None");

            for j in i..group_size {
                let j_id = id_list.get(j).expect("Entry.id should never be None");
                let entry_i = entries.get(i).expect("Entry should not be None");
                let entry_j = entries.get(j).expect("Entry should not be None");
                let temp_r_ij =
                    calculate_elo_change_for_pair((entry_i.1, entry_i.2), (entry_j.1, entry_j.2));

                r_map.insert(i_id.clone(), r_map.get(i_id).unwrap() + temp_r_ij.0);
                r_map.insert(j_id.clone(), r_map.get(j_id).unwrap() + temp_r_ij.1);
            }
        }

        r_map
            .iter()
            .map(|(id, r)| (id.clone(), (k as f32 * *r).round() as i32))
            .collect()
    }
}

// End of calculator module

#[cfg(test)]
mod tests {
    use super::calculator::{calculate_elo_change_for_group, calculate_elo_change_for_pair};
    use super::*;

    fn create_player_1_struct() -> Entry {
        Entry {
            id: String::from("1"),
            input_elo: 1020,
            place: 1,
            ..Default::default()
        }
    }

    fn create_player_1_tuple_with_id() -> (String, i32, i8) {
        (String::from("1"), 1020, 1)
    }

    fn create_player_1_tuple_no_id() -> (i32, i8) {
        (1020, 1)
    }

    fn create_player_2_struct() -> Entry {
        Entry {
            id: String::from("2"),
            input_elo: 900,
            place: 2,
            ..Default::default()
        }
    }

    fn create_player_2_tuple_with_id() -> (String, i32, i8) {
        (String::from("2"), 900, 2)
    }

    fn create_player_3_struct() -> Entry {
        Entry {
            id: String::from("3"),
            input_elo: 800,
            place: 3,
            ..Default::default()
        }
    }

    fn create_player_3_tuple_with_id() -> (String, i32, i8) {
        (String::from("3"), 800, 3)
    }

    fn create_player_3_tuple_no_id() -> (i32, i8) {
        (800, 3)
    }

    fn create_player_4_struct() -> Entry {
        Entry {
            id: String::from("4"),
            input_elo: 1000,
            place: 4,
            ..Default::default()
        }
    }

    fn create_player_4_tuple_with_id() -> (String, i32, i8) {
        (String::from("4"), 1000, 4)
    }

    fn create_player_4_tuple_no_id() -> (i32, i8) {
        (1000, 4)
    }

    fn create_player_5_tuple_no_id() -> (i32, i8) {
        (1000, 5)
    }

    fn create_player_6_tuple_no_id() -> (i32, i8) {
        (1400, 6)
    }

    fn create_player_7_tuple_no_id() -> (i32, i8) {
        (1600, 1)
    }

    #[test]
    fn test_calculate_elo_change_for_pair_equal_elos() {
        let left = create_player_4_tuple_no_id();
        let right = create_player_5_tuple_no_id();

        let delta = calculate_elo_change_for_pair(left, right);

        assert_eq!(delta.0, 0.5);
        assert_eq!(delta.1, -0.5);
    }

    #[test]
    fn test_calculate_elo_change_for_pair_small_elo_difference() {
        let left = create_player_1_tuple_no_id();
        let right = create_player_4_tuple_no_id();

        let answer: f32 = 0.47124946;

        let delta = calculator::calculate_elo_change_for_pair(left, right);

        assert!(delta.0 - answer < 0.0001);
        assert!(delta.1 + answer < 0.0001);
    }

    #[test]
    fn test_calculate_elo_change_for_pair_large_elo_difference() {
        let left = create_player_3_tuple_no_id();
        let right = create_player_6_tuple_no_id();

        let answer: f32 = 0.9693466;

        let delta = calculate_elo_change_for_pair(left, right);

        assert!(delta.0 - answer < 0.0001);
        assert!(delta.1 + answer < 0.0001);
    }

    #[test]
    fn test_calculate_elo_change_for_tie() {
        let left = create_player_1_tuple_no_id();
        let right = create_player_7_tuple_no_id();

        let answer: f32 = 0.46573445;

        let delta = calculate_elo_change_for_pair(left, right);

        assert!(delta.0 - answer < 0.0001);
        assert!(delta.1 + answer < 0.0001);
    }

    #[test]
    fn test_calculate_elo_change_for_group_size_2_k_equals_32() {
        let player_1 = create_player_1_tuple_with_id();
        let player_2 = create_player_2_tuple_with_id();

        let result = calculate_elo_change_for_group(vec![player_1, player_2], 32);

        assert_eq!(*result.get("1").unwrap(), 11);
        assert_eq!(*result.get("2").unwrap(), -11);
    }

    #[test]
    fn test_calculate_elo_change_for_group_size_3_k_equals_16() {
        let player_1 = create_player_1_tuple_with_id();
        let player_2 = create_player_2_tuple_with_id();
        let player_3 = create_player_3_tuple_with_id();

        let result = calculate_elo_change_for_group(vec![player_1, player_2, player_3], 16);

        assert_eq!(*result.get("1").unwrap(), 9);
        assert_eq!(*result.get("2").unwrap(), 0);
        assert_eq!(*result.get("3").unwrap(), -9);
    }

    #[test]
    fn test_calculate_elo_change_for_group_size_4_k_equals_8() {
        let player_1 = create_player_1_tuple_with_id();
        let player_2 = create_player_2_tuple_with_id();
        let player_3 = create_player_3_tuple_with_id();
        let player_4 = create_player_4_tuple_with_id();

        let result =
            calculate_elo_change_for_group(vec![player_1, player_2, player_3, player_4], 8);

        assert_eq!(*result.get("1").unwrap(), 8);
        assert_eq!(*result.get("2").unwrap(), 5);
        assert_eq!(*result.get("3").unwrap(), 1);
        assert_eq!(*result.get("4").unwrap(), -15);
    }

    #[test]
    fn test_update_elos_for_group_size_2() {
        let mut player1 = create_player_1_struct();
        let mut player2 = create_player_2_struct();

        let result = update_elos_for_group(vec![&mut player1, &mut player2], 32);

        assert_eq!(result[0].output_elo, Some(1031));
        assert_eq!(result[1].output_elo, Some(889));
    }

    #[test]
    fn test_update_elos_for_group_size_3() {
        let mut player1 = create_player_1_struct();
        let mut player2 = create_player_2_struct();
        let mut player3 = create_player_3_struct();

        let result = update_elos_for_group(vec![&mut player1, &mut player2, &mut player3], 16);

        assert_eq!(result[0].output_elo, Some(1029));
        assert_eq!(result[1].output_elo, Some(900));
        assert_eq!(result[2].output_elo, Some(791));
    }

    #[test]
    fn test_update_elos_for_group_size_4() {
        let mut player1 = create_player_1_struct();
        let mut player2 = create_player_2_struct();
        let mut player3 = create_player_3_struct();
        let mut player4 = create_player_4_struct();

        let result = update_elos_for_group(
            vec![&mut player1, &mut player2, &mut player3, &mut player4],
            8,
        );

        assert_eq!(result[0].output_elo, Some(1028));
        assert_eq!(result[1].output_elo, Some(905));
        assert_eq!(result[2].output_elo, Some(801));
        assert_eq!(result[3].output_elo, Some(985));
    }
}
