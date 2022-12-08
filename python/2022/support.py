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
