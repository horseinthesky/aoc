from __future__ import annotations

import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support


def compute(s: str) -> int:
    first, rest = s.split('\n\n')

    lastline_len = len(first.splitlines()[-1].rstrip())
    stacks: list[list[str]]
    stacks = [[] for _ in range((lastline_len + 2) // 4)]

    for line in first.splitlines():
        for i, c in enumerate(line[1::4]):
            if not c.isspace():
                stacks[i].append(c)

    for stack in stacks:
        stack.reverse()

    for instr in rest.splitlines():
        _, n_s, _, f_s, _, d_s = instr.split()
        n, f, d = int(n_s), int(f_s), int(d_s)

        victims = stacks[f - 1][-n:]
        del stacks[f - 1][-n:]

        stacks[d - 1].extend(victims)

    return ''.join(stack[-1] if stack else '' for stack in stacks)


INPUT_S = """\
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"""
EXPECTED = "MCD"


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
