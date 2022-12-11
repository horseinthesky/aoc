from __future__ import annotations

import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support


D = {
    'R': support.Direction4.RIGHT,
    'U': support.Direction4.UP,
    'L': support.Direction4.LEFT,
    'D': support.Direction4.DOWN,
}


def compute(s: str) -> int:
    head = tail = (0, 0)
    seen = {tail}

    for line in s.splitlines():
        dir_s, n_s = line.split()
        move = D[dir_s]
        n = int(n_s)

        for _ in range(n):
            head = move.apply(*head)
            if abs(head[0] - tail[0]) >= 2 or abs(head[1] - tail[1]) >= 2:
                tail = move.opposite.apply(*head)
                seen.add(tail)

    return len(seen)


INPUT_S = """\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"""
EXPECTED = 13


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
