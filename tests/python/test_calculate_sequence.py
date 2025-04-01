from elo_calculator import Entry, update_elos_for_sequence

def test_calculate_sequence(player_1: Entry, player_2: Entry, player_3: Entry, player_1_round_2: Entry) -> None:
    group_1 = [player_1, player_2]
    group_2 = [player_1_round_2, player_3]

    k = 16
    result = update_elos_for_sequence([group_1, group_2], k)

    assert result[0][0].output_elo == 1025
    assert result[0][1].output_elo == 895
    assert result[1][0].output_elo == 1028
    assert result[1][1].output_elo == 797