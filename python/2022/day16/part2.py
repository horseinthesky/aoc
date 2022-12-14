from __future__ import annotations

import re
import sys
from pathlib import Path

import itertools
import collections
import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support

RATE = re.compile(r"rate=(\d+);")
VALVES = re.compile(r"to valves? (.*)$")


def compute(s: str) -> int:
    edges = {}
    rates = {}

    for line in s.splitlines():
        _, name, *_ = line.split()
        match_valves = VALVES.search(line)
        assert match_valves is not None
        targets = match_valves[1].split(', ')
        match = RATE.search(line)
        assert match is not None
        edges[name] = targets
        rates[name] = int(match[1])

    weights = {}
    positive_rates = frozenset(k for k, v in rates.items() if v)
    meaningful_edges = ['AA', *positive_rates]
    for a, b in itertools.combinations(meaningful_edges, r=2):
        todo_bfs: collections.deque[tuple[str, ...]]
        todo_bfs = collections.deque([(a,)])
        while todo_bfs:
            path = todo_bfs.popleft()
            if path[-1] == b:
                break
            else:
                todo_bfs.extend(
                    (*path, n) for n in edges[path[-1]]
                    if n not in path
                )
        weights[(a, b)] = len(path)
        weights[(b, a)] = len(path)

    # time to total
    best: dict[frozenset[str], int] = {}
    todo: list[tuple[int, int, str, frozenset[str]]]
    todo = [(0, 0, 'AA', frozenset())]
    while todo:
        score, time, current, seen = todo.pop()

        best[seen] = max(best.get(seen, score), score)

        for p in positive_rates - seen:
            needed_time = time + weights[(current, p)]
            if needed_time < 26:
                todo.append((
                    score + (26 - needed_time) * rates[p],
                    needed_time,
                    p,
                    seen | {p},
                ))

    return max(
        v1 + v2
        for (k1, v1), (k2, v2) in itertools.combinations(best.items(), r=2)
        if not k1 & k2
    )


INPUT_S = """\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"""
EXPECTED = 1707


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
