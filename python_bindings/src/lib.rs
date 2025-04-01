use ::elo_calculator as elo_lib;
use elo_lib::models::Entry;
use pyo3::prelude::*;

#[pyfunction]
fn update_elos_for_group(mut entries: Vec<Entry>, k: i32) -> PyResult<Vec<Entry>> {
    // Convert entries to mutable references for the library function
    let entry_refs: Vec<&mut Entry> = entries.iter_mut().collect();

    elo_lib::services::calculate_elos::update_elos_for_group(entry_refs, k);

    Ok(entries)
}

#[pyfunction]
fn update_elos_for_sequence(
    mut entry_sequence: Vec<Vec<Entry>>,
    k: i32,
) -> PyResult<Vec<Vec<Entry>>> {
    // Convert nested vectors of entries to nested vectors of mutable references
    let seq_refs: Vec<Vec<&mut Entry>> = entry_sequence
        .iter_mut()
        .map(|group| group.iter_mut().collect())
        .collect();

    elo_lib::services::calculate_elos::update_elos_for_sequence(seq_refs, k);

    Ok(entry_sequence)
}

#[pymodule]
fn elo_calculator(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(update_elos_for_group, m)?)?;
    m.add_function(wrap_pyfunction!(update_elos_for_sequence, m)?)?;
    m.add_class::<Entry>()?;
    Ok(())
}
