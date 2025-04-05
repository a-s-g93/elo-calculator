use ::elo_calculator as elo_lib;
use elo_lib::models::Entry;
use pyo3::prelude::*;


/// Update elos for a single group.
///
/// Parameters
/// ----------
/// winner_elo : int
///     The winner elo score
/// loser_elo : int
///     The loser elo score
/// k : int
///     The k value to use in elo calculation
///
/// Returns
/// -------
/// Tuple[int, int]
///     A tuple containing the updated winner score and loser score respectively
#[pyfunction]
#[pyo3(text_signature = "(winner_elo, loser_elo, k, /)")]
fn quick_calc(winner_elo: i32, loser_elo: i32, k: i32) -> PyResult<(i32, i32)> {
    let result = elo_lib::services::calculate_elos::quick_calc(winner_elo, loser_elo, k);

    Ok(result)
}

/// Update elos for a single group.
///
/// Parameters
/// ----------
/// entries : List[Entry]
///     A list of entry objects that have valid `input_elo` fields
/// k : int
///     The k value to use in elo calculation
///
/// Returns
/// -------
/// List[Entry]
///     The list of updated entries with populated `output_elo` fields
#[pyfunction]
#[pyo3(text_signature = "(entries, k, /)")]
fn update_elos_for_group(mut entries: Vec<Entry>, k: i32) -> PyResult<Vec<Entry>> {
    let entry_refs: Vec<&mut Entry> = entries.iter_mut().collect();

    elo_lib::services::calculate_elos::update_elos_for_group(entry_refs, k);

    Ok(entries)
}

/// Update elos for a sequence of match groups.
///
/// This function processes a series of matches in sequence, where each match
/// contains multiple entries/players. Elo ratings are updated progressively,
/// meaning the output elo of one match becomes the input elo for the next match.
///
/// Parameters
/// ----------
/// group_sequence : List[List[Entry]]
///     A sequence of match groups, where each group is a list of entries with valid `input_elo` fields
/// k : int
///     The k value to use in elo calculation
///
/// Returns
/// -------
/// List[List[Entry]]
///     The sequence of match groups with updated entries and populated `output_elo` fields
#[pyfunction]
#[pyo3(text_signature = "(group_sequence, k, /)")]
fn update_elos_for_sequence(
    mut group_sequence: Vec<Vec<Entry>>,
    k: i32,
) -> PyResult<Vec<Vec<Entry>>> {
    let seq_refs: Vec<Vec<&mut Entry>> = group_sequence
        .iter_mut()
        .map(|group| group.iter_mut().collect())
        .collect();

    elo_lib::services::calculate_elos::update_elos_for_sequence(seq_refs, k);

    Ok(group_sequence)
}

/// Elo Calculator Python Module
///
/// A Python module for calculating Elo ratings for players or teams in games.
///
/// This module provides functions to update Elo ratings for either single matches
/// or sequences of matches. It handles both 1v1 and multiplayer scenarios.
///
/// Example
/// -------
/// ```python
/// import elo_calculator
///
/// # Create entries for a match
/// player1 = elo_calculator.Entry("1", "Alice", 1, 1500)
/// player2 = elo_calculator.Entry("2", "Bob", 2, 1400)
///
/// # Update Elo ratings with k-factor of 32
/// result = elo_calculator.update_elos_for_group([player1, player2], 32)
/// print(f"{result[0].name}: {result[0].output_elo}")
/// print(f"{result[1].name}: {result[1].output_elo}")
/// ```
#[pymodule]
fn elo_calculator(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(quick_calc, m)?)?;
    m.add_function(wrap_pyfunction!(update_elos_for_group, m)?)?;
    m.add_function(wrap_pyfunction!(update_elos_for_sequence, m)?)?;
    m.add_class::<Entry>()?;
    Ok(())
}
