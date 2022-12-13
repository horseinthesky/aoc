from __future__ import annotations

import heapq
import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support

MAPPED = {
    "S": "a",
    "E": "z",
}


def compute(s: str) -> int:
    coords = {}
    start = end = None
    for y, line in enumerate(s.splitlines()):
        for x, c in enumerate(line):
            coords[(y, x)] = c
            if c == "S":
                start = (y, x)
            elif c == "E":
                end = (y, x)

    assert start is not None
    assert end is not None

    seen = set()
    todo = [(0, start)]

    while todo:
        cost, pos = heapq.heappop(todo)

        if pos == end:
            return cost

        if pos in seen:
            continue

        seen.add(pos)

        for cand in support.adjacent_4(*pos):
            if cand in coords:
                current_c = MAPPED.get(coords[pos], coords[pos])
                cand_c = MAPPED.get(coords[cand], coords[cand])
                if ord(cand_c) - ord(current_c) <= 1:
                    heapq.heappush(todo, (cost + 1, cand))

    raise AssertionError("wat")


INPUT_S = """\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"""
EXPECTED = 31


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
