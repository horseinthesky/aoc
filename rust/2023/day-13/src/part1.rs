use crate::custom_error::AocError;

fn find_col(grid: &[&[u8]], limit: usize) -> Option<usize> {
    (0..grid[0].len() - 1).find(|&c| {
        let incorrect = (0..=c.min(grid[0].len() - c - 2))
            .map(|dc| {
                let a = c - dc;
                let b = c + 1 + dc;
                (0..grid.len())
                    .filter(|&r| grid[r][a] != grid[r][b])
                    .count()
            })
            .sum::<usize>();
        incorrect == limit
    })
}

fn find_row(grid: &[&[u8]], limit: usize) -> Option<usize> {
    (0..grid.len() - 1).find(|&r| {
        let incorrect = (0..=r.min(grid.len() - r - 2))
            .map(|dr| {
                let a = r - dr;
                let b = r + 1 + dr;
                (0..grid[0].len())
                    .filter(|&c| grid[a][c] != grid[b][c])
                    .count()
            })
            .sum::<usize>();
        incorrect == limit
    })
}

fn solve(grids: &[Vec<&[u8]>], limit: usize) -> usize {
    grids
        .iter()
        .map(|grid| {
            find_row(grid, limit)
                .map(|r| (r + 1) * 100)
                .or_else(|| {
                    find_col(grid, limit).map(|c| c + 1)
                })
                .unwrap()
        })
        .sum()
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let grids = input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.as_bytes())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(solve(&grids, 0).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!("405", process(input)?);
        Ok(())
    }
}
