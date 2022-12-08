from __future__ import annotations

import os
import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support


def compute(s: str) -> int:
    MAX = 100000

    files = {}
    dirs = {"/"}

    pwd = "/"
    in_ls = False
    for line in s.splitlines()[1:]:
        if in_ls and line.startswith("$"):
            in_ls = False
        elif in_ls and line.startswith("dir "):
            _, dirname = line.split(" ", 1)
            dirs.add(os.path.join(pwd, dirname))
            continue
        elif in_ls:
            sz, filename = line.split(" ", 1)
            files[os.path.join(pwd, filename)] = int(sz)
            continue

        if line == "$ ls":
            in_ls = True
        elif line == "$ cd ..":
            pwd = os.path.dirname(pwd) or "/"
        elif line.startswith("$ cd"):
            pwd = os.path.join(pwd, line.split(" ", 2)[-1])
        else:
            raise AssertionError(line)

    def size(dirname: str) -> int:
        sz = 0
        for k, v in files.items():
            if k.startswith(f"{dirname}/"):
                sz += v

        if sz > MAX:
            return 0
        else:
            return sz

    root = sum(files.values())
    if root > MAX:
        root = 0

    return root + sum(size(dirname) for dirname in dirs)


INPUT_S = """\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"""
EXPECTED = 95437


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
