
from elo_calculator import quick_calc

def test_quick_calc() -> None:
    w = 1020
    l = 900
    k = 32

    result = quick_calc(w, l, k)

    assert result[0] == 1031
    assert result[1] == 889