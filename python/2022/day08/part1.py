from __future__ import annotations

import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support


def compute(s: str) -> int:
    coords = support.parse_coords_int(s)
    visible = set()

    ymin, xmin = min(coords)
    ymax, xmax = max(coords)

    for y in range(ymin, ymax + 1):
        # down
        val = coords[(y, xmin)]
        visible.add((y, xmin))
        for x in range(xmin + 1, xmax + 1):
            cand = (y, x)
            if coords[cand] > val:
                visible.add(cand)
                val = coords[cand]

        # up
        val = coords[(y, xmax)]
        visible.add((y, xmax))
        for x in range(xmax, -1, -1):
            cand = (y, x)
            if coords[cand] > val:
                visible.add(cand)
                val = coords[cand]

    for x in range(xmin, xmax + 1):
        # right
        val = coords[(ymin, x)]
        visible.add((ymin, x))
        for y in range(ymin + 1, ymax + 1):
            cand = (y, x)
            if coords[cand] > val:
                visible.add(cand)
                val = coords[cand]

        # left
        val = coords[(ymax, x)]
        visible.add((ymax, x))
        for y in range(ymax, -1, -1):
            cand = (y, x)
            if coords[cand] > val:
                visible.add(cand)
                val = coords[cand]

    return len(visible)


INPUT_S = """\
30373
25512
65332
33549
35390
"""
EXPECTED = 21


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
