use crate::custom_error::AocError;

use hashbrown::HashSet;
use itertools::Itertools;

fn bfs(
    grid: &[&[u8]],
    start: (isize, isize),
    steps: usize,
) -> usize {
    let mut pos = HashSet::from_iter([start]);
    let mut new_pos = HashSet::new();

    for _ in 0..steps {
        new_pos.clear();
        for &(row, column) in &pos {
            for (diff_row, diff_column) in
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
            {
                let (new_row, new_column) = (row + diff_row, column + diff_column);

                let tile = grid[new_row as usize % grid.len()]
                    [new_column as usize % grid.len()];
                if tile == b'#' {
                    continue;
                }

                new_pos.insert((new_row, new_column));
            }
        }

        (pos, new_pos) = (new_pos, pos);
    }

    pos.len()
}

#[tracing::instrument]
pub fn process(
    input: &str,
    step_count: usize,
) -> miette::Result<String, AocError> {
    let grid = input
        .lines()
        .map(str::as_bytes)
        .collect::<Vec<_>>();

    let start = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(r, c)| grid[r][c] == b'S')
        .map(|(r, c)| (r as isize, c as isize))
        .unwrap();

    let res = bfs(&grid, start, step_count);

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!("16", process(input, 6)?);
        Ok(())
    }
}
