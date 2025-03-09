

use models::entry::*;

fn calculate_elo_change_for_pair(entry_one: &Entry, entry_two: &Entry) -> (f32, f32) {

}

fn calculate_elo_change_for_group(entries: Vec<&Entry>) -> HashMap<&str, f32> {

}

fn update_elos_for_group(entries: Vec<&mut Entry>) -> Vec<&mut Entry> {

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_elo_change_for_pair_equal_elos() {}

    #[test]
    fn test_calculate_elo_change_for_pair_small_elo_difference() {}

    #[test]
    fn test_calculate_elo_change_for_pair_mid_elo_difference() {}

    #[test]
    fn test_calculate_elo_change_for_pair_large_elo_difference() {}

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