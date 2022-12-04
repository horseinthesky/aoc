from __future__ import annotations

import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support

trans = {
    "A": "R",
    "B": "P",
    "C": "S",
    "X": "R",
    "Y": "P",
    "Z": "S",
}

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


def compute(s: str) -> int:
    for k, v in trans.items():
        s = s.replace(k, v)

    n = 0

    for line in s.splitlines():
        their_move, our_move = line.split()
        if their_move == our_move:
            n += 3
        elif win[their_move] != our_move:
            n += 6

        n += shape[our_move]

    return n


INPUT_S = """\
A Y
B X
C Z
"""
EXPECTED = 15


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
