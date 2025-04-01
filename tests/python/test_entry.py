from elo_calculator import Entry

def test_entry_init_no_input_elo() -> None:
    Entry(id="1", name="DK", place=1)

def test_entry_init_with_input_elo() -> None:
    Entry(id="1", name="DK", place=1, input_elo=1000)