use crate::custom_error::AocError;

use radix_heap::RadixHeapMap;
use rustc_hash::FxHashMap;

pub struct City {
    heat_loss: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl City {
    pub fn len(&self) -> usize {
        self.heat_loss.len()
    }
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug,
)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
struct Step {
    idx: usize,
    dir: Direction,
    steps: u8,
}

impl Step {
    pub fn try_create(
        city: &City,
        from_row: usize,
        from_col: usize,
        from_dir: Direction,
        steps: u8,
        to_dir: Direction,
    ) -> Option<Self> {
        if from_dir == to_dir && steps == 3
            || Self::out_of_bounds(
                city, from_row, from_col, to_dir,
            )
        {
            None
        } else {
            Self::make_step(
                city, from_row, from_col, from_dir, to_dir,
                steps,
            )
        }
    }

    pub fn try_create_ultra(
        city: &City,
        from_row: usize,
        from_col: usize,
        from_dir: Direction,
        steps: u8,
        to_dir: Direction,
    ) -> Option<Self> {
        if from_dir == to_dir && steps == 10
            || from_dir != to_dir && steps < 4
            || Self::out_of_bounds(
                city, from_row, from_col, to_dir,
            )
        {
            None
        } else {
            Self::make_step(
                city, from_row, from_col, from_dir, to_dir,
                steps,
            )
        }
    }

    fn out_of_bounds(
        city: &City,
        from_row: usize,
        from_col: usize,
        to_dir: Direction,
    ) -> bool {
        to_dir == Direction::Up && from_row == 0
            || to_dir == Direction::Left && from_col == 0
            || to_dir == Direction::Down
                && from_row == city.rows - 1
            || to_dir == Direction::Right
                && from_col == city.cols - 1
    }

    fn make_step(
        city: &City,
        from_row: usize,
        from_col: usize,
        from_dir: Direction,
        dir: Direction,
        steps: u8,
    ) -> Option<Self> {
        let steps =
            if from_dir == dir { steps + 1 } else { 1 };
        let idx = match dir {
            Direction::Right => {
                from_row * city.cols + from_col + 1
            }
            Direction::Down => {
                (from_row + 1) * city.cols + from_col
            }
            Direction::Left => {
                from_row * city.cols + from_col - 1
            }
            Direction::Up => {
                (from_row - 1) * city.cols + from_col
            }
        };
        Some(Self { idx, dir, steps })
    }

    pub fn to_pending(&self) -> PendingStep {
        PendingStep {
            idx: self.idx,
            steps: self.steps,
            dir: if self.dir == Direction::Right
                || self.dir == Direction::Left
            {
                PendingDirection::Horiz
            } else {
                PendingDirection::Vert
            },
        }
    }
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug,
)]
enum PendingDirection {
    Horiz,
    Vert,
}

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug,
)]
struct PendingStep {
    idx: usize,
    dir: PendingDirection,
    steps: u8,
}

struct Traversal<'a> {
    city: &'a City,
    cache: FxHashMap<PendingStep, u32>,
    pending: RadixHeapMap<std::cmp::Reverse<u32>, Step>,
}

impl<'a> Traversal<'a> {
    pub fn new(city: &'a City) -> Self {
        Traversal {
            city,
            cache: FxHashMap::default(),
            pending: RadixHeapMap::new(),
        }
    }

    pub fn minimize(&mut self, ultra: bool) -> u32 {
        let target_idx = self.city.len() - 1;
        let mut curr = Step {
            idx: 0,
            dir: Direction::Right,
            steps: 0,
        };
        self.cache.insert(curr.to_pending(), 0);
        let down = Step {
            idx: 0,
            dir: Direction::Down,
            steps: 0,
        };
        self.cache.insert(down.to_pending(), 0);
        self.pending.push(std::cmp::Reverse(0), down);

        let mut heat = std::cmp::Reverse(0);
        while curr.idx != target_idx
            || (ultra && curr.steps < 4)
        {
            for neighbor in self.neighbors(&curr, ultra) {
                if let Some(step) = neighbor {
                    self.maybe_update_pending(&curr, step);
                }
            }
            (heat, curr) = self.pending.pop().unwrap();
        }

        heat.0
    }

    fn neighbors(
        &self,
        s: &Step,
        ultra: bool,
    ) -> [Option<Step>; 3] {
        let col = s.idx % self.city.cols;
        let row = s.idx / self.city.cols;

        if ultra {
            match s.dir {
                Direction::Right => [
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Right,
                    ),
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Up,
                    ),
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Down,
                    ),
                ],
                Direction::Down => [
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Down,
                    ),
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Right,
                    ),
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Left,
                    ),
                ],
                Direction::Left => [
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Left,
                    ),
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Down,
                    ),
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Up,
                    ),
                ],
                Direction::Up => [
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Up,
                    ),
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Left,
                    ),
                    Step::try_create_ultra(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Right,
                    ),
                ],
            }
        } else {
            match s.dir {
                Direction::Right => [
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Right,
                    ),
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Up,
                    ),
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Down,
                    ),
                ],
                Direction::Down => [
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Down,
                    ),
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Right,
                    ),
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Left,
                    ),
                ],
                Direction::Left => [
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Left,
                    ),
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Down,
                    ),
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Up,
                    ),
                ],
                Direction::Up => [
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Up,
                    ),
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Left,
                    ),
                    Step::try_create(
                        self.city,
                        row,
                        col,
                        s.dir,
                        s.steps,
                        Direction::Right,
                    ),
                ],
            }
        }
    }

    fn maybe_update_pending(
        &mut self,
        curr: &Step,
        next: Step,
    ) {
        let curr_heat =
            self.cache.get(&curr.to_pending()).unwrap();
        let heat = curr_heat
            + self.city.heat_loss[next.idx] as u32;
        let next_cache = self.cache.get(&next.to_pending());
        let next_heat = next_cache.unwrap_or(&u32::MAX);

        if heat < *next_heat {
            if next_cache.is_none() {
                self.cache.insert(next.to_pending(), heat);
            } else {
                *self
                    .cache
                    .get_mut(&next.to_pending())
                    .unwrap() = heat;
            }
            self.pending
                .push(std::cmp::Reverse(heat), next);
        }
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let cols = input.find('\n').unwrap();
    let heat_loss = input
        .as_bytes()
        .iter()
        .filter(|b| **b != b'\n')
        .map(|b| b - b'0')
        .collect::<Vec<u8>>();

    let rows = heat_loss.len() / cols;

    let city = City {
        heat_loss,
        rows,
        cols,
    };

    let mut traversal = Traversal::new(&city);

    let res = traversal.minimize(true);
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!("94", process(input)?);
        Ok(())
    }
}
