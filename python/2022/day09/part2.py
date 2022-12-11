from __future__ import annotations

import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support

D = {
    "R": support.Direction4.RIGHT,
    "U": support.Direction4.UP,
    "L": support.Direction4.LEFT,
    "D": support.Direction4.DOWN,
}


def fixup(head: tuple[int, int], tail: tuple[int, int]) -> tuple[int, int]:
    hx, hy = head
    tx, ty = tail

    if abs(hy - ty) == 2 and abs(hx - tx) == 2:
        return ((hx + tx) // 2, (hy + ty) // 2)

    if abs(hy - ty) == 2:
        return (hx, (ty + hy) // 2)

    if abs(hx - tx) == 2:
        return ((tx + hx) // 2, hy)

    return tail


def compute(s: str) -> int:
    positions = [(0, 0)] * 10
    seen = {positions[0]}

    for line in s.splitlines():
        dir_s, n_s = line.split()
        move = D[dir_s]
        n = int(n_s)

        for _ in range(n):
            positions[0] = move.apply(*positions[0])

            prev = positions[0]
            for i in range(1, len(positions)):
                positions[i] = fixup(prev, positions[i])
                prev = positions[i]

            seen.add(positions[-1])

    return len(seen)


INPUT_S = """\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"""
EXPECTED = 36


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
