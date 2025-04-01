import pytest

from elo_calculator import Entry

@pytest.fixture
def player_1() -> Entry:
    return Entry(id="1", name="DK", input_elo=1020, place=1)

@pytest.fixture
def player_2() -> Entry:
    return Entry(id="2", name="Mario", input_elo=900, place=2)

@pytest.fixture
def player_3() -> Entry:
    return Entry(id="3", name="Toad", input_elo=800, place=3)

@pytest.fixture
def player_1_round_2() -> Entry:
    return Entry(id="1", name="DK", place=1)