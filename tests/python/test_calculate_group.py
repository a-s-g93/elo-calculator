from elo_calculator import Entry, update_elos_for_group

def test_calculate_group(player_1: Entry, player_2: Entry, player_3: Entry) -> None:
    group = [player_1, player_2, player_3]
    k = 16
    result = update_elos_for_group(group, k)

    assert result[0].output_elo == 1029
    assert result[1].output_elo == 900
    assert result[2].output_elo == 791
    
