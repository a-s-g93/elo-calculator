//! Elo rating calculation service module
//!
//! This module provides functionality for calculating and updating Elo ratings
//! for a group of players based on their performance.

use calculator::update_event_input_elos_from_previous_event;

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
    let computation_inputs: Vec<(&str, i32, i8)> = entries
        .iter()
        .map(|e| (e.id.as_str(), e.input_elo.unwrap(), e.place))
        .collect();

    // Calculate Elo changes
    let elo_changes = calculator::calculate_elo_change_for_group(computation_inputs, k);

    // Collect the changes first to avoid borrowing conflicts
    let entry_changes: Vec<(usize, i32)> = entries
        .iter()
        .enumerate()
        .filter_map(|(idx, entry)| {
            elo_changes
                .get(entry.id.as_str())
                .map(|&change| (idx, change))
        })
        .collect();

    // Now apply the changes separately
    for (idx, change) in entry_changes {
        let entry = &mut entries[idx];
        entry.output_elo = Some(entry.input_elo.unwrap() + change);
    }

    entries
}

pub fn update_elos_for_sequence(mut groups: Vec<Vec<&mut Entry>>, k: i32) -> Vec<Vec<&mut Entry>> {
    let mut elo_hash = HashMap::<String, i32>::new();

    // Process each group one at a time
    for i in 0..groups.len() {
        // Extract the current group
        let mut current_group = Vec::new();
        std::mem::swap(&mut current_group, &mut groups[i]);

        // Update entries from previous event
        let group_with_updated_inputs =
            update_event_input_elos_from_previous_event(current_group, &elo_hash);

        // Update Elo ratings for this group
        let group_with_updated_outputs = update_elos_for_group(group_with_updated_inputs, k);

        let mut new_group = Vec::new();

        // Store the updated Elo values in our hash map
        for entry in group_with_updated_outputs {
            if let Some(new_elo) = entry.output_elo {
                elo_hash.insert(entry.id.clone(), new_elo);
            }
            new_group.push(entry);
        }

        // Put the updated group back
        groups[i] = new_group;
    }

    groups
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
        entries: Vec<(&str, i32, i8)>,
        k: i32,
    ) -> HashMap<&str, i32> {
        let mut r_map: HashMap<&str, f32> = entries.iter().map(|e| (e.0, 0.0)).collect();
        let id_list: Vec<&str> = entries.iter().map(|e| e.0).collect();
        let group_size = id_list.len();

        for i in 0..group_size {
            let i_id = id_list.get(i).expect("Entry.id should never be None");

            for j in i..group_size {
                let j_id = id_list.get(j).expect("Entry.id should never be None");
                let entry_i = entries.get(i).expect("Entry should not be None");
                let entry_j = entries.get(j).expect("Entry should not be None");
                let temp_r_ij =
                    calculate_elo_change_for_pair((entry_i.1, entry_i.2), (entry_j.1, entry_j.2));

                r_map.insert(i_id, r_map.get(i_id).unwrap() + temp_r_ij.0);
                r_map.insert(j_id, r_map.get(j_id).unwrap() + temp_r_ij.1);
            }
        }

        r_map
            .iter()
            .map(|(id, r)| (*id, (k as f32 * *r).round() as i32))
            .collect()
    }

    #[allow(dead_code)]
    pub(crate) fn update_event_input_elos_from_previous_event<'a>(
        mut entries: Vec<&'a mut Entry>,
        elo_hash: &HashMap<String, i32>,
    ) -> Vec<&'a mut Entry> {
        for entry in entries.iter_mut() {
            if let Some(update) = elo_hash.get(entry.id.as_str()) {
                entry.input_elo = Some(*update);
            }
        }

        entries
    }
}

// End of calculator module

#[cfg(test)]
mod tests {
    use super::calculator::{
        calculate_elo_change_for_group, calculate_elo_change_for_pair,
        update_event_input_elos_from_previous_event,
    };
    use super::*;

    fn create_player_1_struct() -> Entry {
        Entry {
            id: String::from("1"),
            input_elo: Some(1020),
            place: 1,
            ..Default::default()
        }
    }

    fn create_player_1_tuple_with_id() -> (&'static str, i32, i8) {
        ("1", 1020, 1)
    }

    fn create_player_1_tuple_no_id() -> (i32, i8) {
        (1020, 1)
    }

    fn create_player_2_struct() -> Entry {
        Entry {
            id: String::from("2"),
            input_elo: Some(900),
            place: 2,
            ..Default::default()
        }
    }

    fn create_player_2_tuple_with_id() -> (&'static str, i32, i8) {
        ("2", 900, 2)
    }

    fn create_player_3_struct() -> Entry {
        Entry {
            id: String::from("3"),
            input_elo: Some(800),
            place: 3,
            ..Default::default()
        }
    }

    fn create_player_3_tuple_with_id() -> (&'static str, i32, i8) {
        ("3", 800, 3)
    }

    fn create_player_3_tuple_no_id() -> (i32, i8) {
        (800, 3)
    }

    fn create_player_4_struct() -> Entry {
        Entry {
            id: String::from("4"),
            input_elo: Some(1000),
            place: 4,
            ..Default::default()
        }
    }

    fn create_player_4_tuple_with_id() -> (&'static str, i32, i8) {
        ("4", 1000, 4)
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

    #[test]
    fn test_update_elos_for_sequence() {
        let mut player1 = create_player_1_struct();
        let mut player2 = create_player_2_struct();
        let mut player3 = create_player_3_struct();
        let mut player1_second_event = create_player_1_struct();

        let result = update_elos_for_sequence(
            vec![
                vec![&mut player1, &mut player2],
                vec![&mut player1_second_event, &mut player3],
            ],
            16,
        );

        assert_eq!(result[0][0].output_elo, Some(1025));
        assert_eq!(result[0][1].output_elo, Some(895));
        assert_eq!(result[1][0].output_elo, Some(1028));
        assert_eq!(result[1][1].output_elo, Some(797));
    }

    #[test]
    fn test_update_event_input_elos_from_previous_event() {
        let mut player1 = create_player_1_struct();
        let mut player2 = create_player_2_struct();

        let mut elo_hash = HashMap::new();
        elo_hash.insert(String::from("1"), 123);
        elo_hash.insert(String::from("2"), 456);

        let result = update_event_input_elos_from_previous_event(
            vec![&mut player1, &mut player2],
            &elo_hash,
        );

        assert_eq!(result[0].input_elo.unwrap(), 123);
        assert_eq!(result[1].input_elo.unwrap(), 456);
    }
}
