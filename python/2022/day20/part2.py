from __future__ import annotations

import collections
import sys
from pathlib import Path
from unittest import mock

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support


def compute(s: str) -> int:
    orig_numbers = support.parse_numbers_split(s)
    orig_numbers = [n * 811589153 for n in orig_numbers]
    numbers = collections.deque(list(enumerate(orig_numbers)))

    for _ in range(10):
        for i, num in enumerate(orig_numbers):
            idx = numbers.index((i, mock.ANY))
            numbers.rotate(-idx)
            assert numbers.popleft() == (i, num)
            numbers.rotate(-num)
            numbers.appendleft((i, num))

    idx_0 = numbers.index((mock.ANY, 0))
    return sum(
        numbers[(idx_0 + i) % len(numbers)][1]
        for i in (1000, 2000, 3000)
    )


INPUT_S = """\
1
2
-3
3
-2
0
4
"""
EXPECTED = 1623178306


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
