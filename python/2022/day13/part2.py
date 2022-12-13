from __future__ import annotations

import sys
import itertools
import functools
import ast
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support


ListLike = int | list['ListLike']


def compare(lhs: ListLike, rhs: ListLike) -> int:
    if isinstance(lhs, int) and not isinstance(rhs, int):
        lhs = [lhs]
    elif not isinstance(lhs, int) and isinstance(rhs, int):
        rhs = [rhs]

    if isinstance(lhs, int) and isinstance(rhs, int):
        return lhs - rhs

    if isinstance(lhs, list) and isinstance(rhs, list):
        for a, b in itertools.zip_longest(lhs, rhs):
            if a is None:
                return -1

            if b is None:
                return 1

            compared = compare(a, b)
            if compared != 0:
                return compared
        else:
            return 0

    raise AssertionError('unreachable')


def compute(s: str) -> int:
    s = s.replace('\n\n', '\n')
    lists = [ast.literal_eval(line) for line in s.splitlines()]

    lists.extend(([[2]], [[6]]))
    lists.sort(key=functools.cmp_to_key(compare))

    return (lists.index([[2]]) + 1) * (lists.index([[6]]) + 1)


INPUT_S = """\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"""
EXPECTED = 140


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
