from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import NamedTuple

import pytest

sys.path.append(str(Path(__file__).parent.parent))
import support

reg = re.compile(
    r"^Sensor at x=(-?\d+), y=(-?\d+): "
    r"closest beacon is at x=(-?\d+), y=(-?\d+)$",
)


class Sensor(NamedTuple):
    x: int
    y: int
    beacon_x: int
    beacon_y: int

    @property
    def distance(self) -> int:
        return abs(self.x - self.beacon_x) + abs(self.y - self.beacon_y)


def compute(s: str, y: int = 2000000) -> int:
    beacons = set()
    coords = set()

    for line in s.splitlines():
        match = reg.match(line)
        assert match is not None
        sensor = Sensor(
            int(match[1]),
            int(match[2]),
            int(match[3]),
            int(match[4]),
        )
        beacons.add((sensor.beacon_x, sensor.beacon_y))
        dist = sensor.distance

        right_term = dist - abs(y - sensor.y)
        for x in range(sensor.x - right_term, sensor.x + right_term + 1):
            coords.add((x, y))

    return len(coords - beacons)


INPUT_S = """\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"""
EXPECTED = 26


@pytest.mark.parametrize(
    ("input_s", "expected"),
    ((INPUT_S, EXPECTED),),
)
def test(input_s: str, expected: int) -> None:
    assert compute(input_s, y=10) == expected


def main() -> int:
    with support.timing():
        content = (Path(__file__).parent / "input.txt").read_text()
        print(compute(content))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
