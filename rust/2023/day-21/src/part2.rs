use hashbrown::HashSet;
use itertools::Itertools;
use tracing::info;

use crate::custom_error::AocError;

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

fn find_polynomial(
    grid: &[&[u8]],
    start: (isize, isize),
    n: usize,
) -> usize {
    let n1 = bfs(grid, start, n % grid.len());
    let n2 = bfs(grid, start, n % grid.len() + grid.len());
    let n3 = bfs(
        grid,
        start,
        n % grid.len() + grid.len() * 2,
    );

    let n = n / grid.len();
    let [a, b, c] = [n1, n2 - n1, n3 - n2];

    a + b * n + (n * (n - 1) / 2) * (c - b)
}

#[tracing::instrument(skip(input))]
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

    let res = find_polynomial(&grid, start, step_count);

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6536)]
    #[case(500, 167004)]
    #[case(1000, 668697)]
    #[case(5000, 16733044)]
    fn test_process(
        #[case] step_count: usize,
        #[case] expected_tiles: usize,
    ) -> miette::Result<()> {
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
        assert_eq!(
            expected_tiles.to_string(),
            process(input, step_count)?
        );

        Ok(())
    }
}
