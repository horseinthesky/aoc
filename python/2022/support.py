from __future__ import annotations

import enum
import contextlib
import sys
import time
from typing import Generator


def parse_numbers_split(s: str) -> list[int]:
    return [int(x) for x in s.split()]


def parse_coords_int(s: str) -> dict[tuple[int, int], int]:
    coords = {}
    for y, line in enumerate(s.splitlines()):
        for x, c in enumerate(line):
            coords[(x, y)] = int(c)
    return coords


@contextlib.contextmanager
def timing(name: str = "") -> Generator[None, None, None]:
    before = time.time()
    try:
        yield
    finally:
        after = time.time()
        t = (after - before) * 1000
        unit = "ms"
        if t < 100:
            t *= 1000
            unit = "μs"
        if name:
            name = f" ({name})"
        print(f"> {int(t)} {unit}{name}", file=sys.stderr, flush=True)


class Direction4(enum.Enum):
    UP = (0, -1)
    RIGHT = (1, 0)
    DOWN = (0, 1)
    LEFT = (-1, 0)

    def __init__(self, x: int, y: int) -> None:
        self.x, self.y = x, y

    @property
    def _vals(self) -> tuple[Direction4, ...]:
        return tuple(type(self).__members__.values())

    @property
    def cw(self) -> Direction4:
        vals = self._vals
        return vals[(vals.index(self) + 1) % len(vals)]

    @property
    def ccw(self) -> Direction4:
        vals = self._vals
        return vals[(vals.index(self) - 1) % len(vals)]

    @property
    def opposite(self) -> Direction4:
        vals = self._vals
        return vals[(vals.index(self) + 2) % len(vals)]

    def apply(self, x: int, y: int, *, n: int = 1) -> tuple[int, int]:
        return self.x * n + x, self.y * n + y


def format_coords_hash(coords: set[tuple[int, int]]) -> str:
    min_x = min(x for x, _ in coords)
    max_x = max(x for x, _ in coords)
    min_y = min(y for _, y in coords)
    max_y = max(y for _, y in coords)
    return '\n'.join(
        ''.join(
            '#' if (x, y) in coords else ' '
            for x in range(min_x, max_x + 1)
        )
        for y in range(min_y, max_y + 1)
    )
