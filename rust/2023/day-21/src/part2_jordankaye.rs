use polyfit_rs::polyfit_rs::polyfit;
use std::collections::HashSet;

use crate::custom_error::AocError;

pub struct Garden {
    steps: Vec<u8>,
    start: usize,
    rows: usize,
    cols: usize,
}

pub fn generate(input: &str) -> Garden {
    let mut steps = Vec::new();
    let mut start = 0;
    let cols = input.find('\n').unwrap();
    for (idx, c) in
        input.chars().filter(|c| *c != '\n').enumerate()
    {
        steps.push((c != '#') as u8);
        if c == 'S' {
            start = idx;
        }
    }
    let rows = steps.len() / cols;

    Garden {
        steps,
        start,
        rows,
        cols,
    }
}

fn infinite_neighbors(
    steps: &[u8],
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> [Option<(usize, usize)>; 4] {
    let mut res = [None; 4];
    let step_col = col % cols;
    let step_row = row % rows;

    let upidx = if step_row > 0 {
        (step_row - 1) * cols + step_col
    } else {
        (rows - 1) * cols + step_col
    };
    if steps[upidx] == 1 {
        let r = if row == 0 {
            usize::MAX - (usize::MAX % rows) - 1
        } else {
            row - 1
        };
        res[0] = Some((r, col));
    }

    let leftidx = if step_col > 0 {
        step_row * cols + step_col - 1
    } else {
        step_row * cols + cols - 1
    };
    if steps[leftidx] == 1 {
        let c = if col == 0 {
            usize::MAX - (usize::MAX % cols) - 1
        } else {
            col - 1
        };
        res[1] = Some((row, c));
    }

    let downidx = if step_row < rows - 1 {
        (step_row + 1) * cols + step_col
    } else {
        step_col
    };
    if steps[downidx] == 1 {
        res[2] = Some((row + 1, col));
    }

    let rightidx = if step_col < cols - 1 {
        step_row * cols + step_col + 1
    } else {
        step_row * cols
    };
    if steps[rightidx] == 1 {
        res[3] = Some((row, col + 1));
    }

    res
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
    step_count: usize,
) -> miette::Result<String, AocError> {
    let garden = generate(input);
    let mut possibilities = [0f32, 0f32];
    let mut working = vec![];
    let r = garden.start / garden.cols;
    let re = r * 10000 - 1;
    let c = garden.start % garden.cols;
    let ce = c * 10000 - 1;
    let mut frontier = vec![(
        re - re % garden.rows + r,
        ce - ce % garden.cols + c,
    )];
    let mut seen = HashSet::new();
    let mut ys = [0f32; 3];
    for step in 0..327 {
        std::mem::swap(&mut working, &mut frontier);
        for (row, col) in working.drain(0..) {
            for neighbor in infinite_neighbors(
                &garden.steps,
                row,
                col,
                garden.rows,
                garden.cols,
            ) {
                if neighbor.is_none() {
                    continue;
                }

                let coords = neighbor.unwrap();
                if seen.contains(&coords) {
                    continue;
                }
                seen.insert(coords);
                frontier.push(coords);
                possibilities[step % 2] += 1f32;
            }
        }
        if step == 64 {
            ys[0] = possibilities[step % 2];
        } else if step == 195 {
            ys[1] = possibilities[step % 2];
        }
    }
    ys[2] = possibilities[0];
    let ps = polyfit(&[0f32, 1f32, 2f32], &ys, 2).unwrap();

    Ok((ps[0].round() as usize
        + ps[1].round() as usize * 202300
        + ps[2].round() as usize * 202300usize.pow(2))
    .to_string())
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
