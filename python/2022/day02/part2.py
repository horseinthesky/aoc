from __future__ import annotations

import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support

shape = {
    "R": 1,
    "P": 2,
    "S": 3,
}

win = {
    "R": "S",
    "P": "R",
    "S": "P",
}

lose = {v: k for k, v in win.items()}

trans = {
    "A": "R",
    "B": "P",
    "C": "S",
}


def compute(s: str) -> int:
    for k, v in trans.items():
        s = s.replace(k, v)

    n = 0

    for line in s.splitlines():
        their_move, must_end_with = line.split()
        if must_end_with == "X":  # lose
            n += shape[win[their_move]]
        elif must_end_with == "Y":  # draw
            n += shape[their_move] + 3
        else:  # win
            n += shape[lose[their_move]] + 6

    return n


INPUT_S = """\
A Y
B X
C Z
"""
EXPECTED = 12


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test(input_s: str, expected: int) -> None:
    assert compute(input_s) == expected


def main() -> int:
    with support.timing():
        content = (Path(__file__).parent / "input.txt").read_text()
        print(compute(content))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
