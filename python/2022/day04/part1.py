from __future__ import annotations

import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support


def compute(s: str) -> int:
    n = 0
    for line in s.splitlines():
        ab, cd = line.split(',')
        a_s, b_s = ab.split('-')
        c_s, d_s = cd.split('-')
        a, b = int(a_s), int(b_s)
        c, d = int(c_s), int(d_s)

        if a <= c <= d <= b:
            n += 1
        elif c <= a <= b <= d:
            n += 1

    return n


INPUT_S = """\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"""
EXPECTED = 2


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
