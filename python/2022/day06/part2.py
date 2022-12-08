from __future__ import annotations

import collections
import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support


def compute(s: str) -> int:
    POSSIBLE_MARKET_FIRST_CHAR_POSITION = 14

    chars = collections.deque(maxlen=POSSIBLE_MARKET_FIRST_CHAR_POSITION)

    for i, c in enumerate(s, start=1):
        chars.append(c)

        if i < POSSIBLE_MARKET_FIRST_CHAR_POSITION:
            continue

        if len(set(chars)) == POSSIBLE_MARKET_FIRST_CHAR_POSITION:
            return i


INPUT_S = """\
mjqjpqmgbljsphdztnvjfqwrcgsmlb
"""
EXPECTED = 19


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
