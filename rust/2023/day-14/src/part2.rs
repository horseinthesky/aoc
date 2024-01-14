use std::collections::HashMap;
use itertools::Itertools;

use crate::custom_error::AocError;

fn roll_north(map: &mut Vec<Vec<u8>>) {
    for r in 0..map.len() - 1 {
        for c in 0..map[0].len() {
            if map[r + 1][c] != b'O' {
                continue;
            }
            for rr in (0..=r).rev() {
                if map[rr][c] != b'.' {
                    break;
                }
                map[rr][c] = b'O';
                map[rr + 1][c] = b'.';
            }
        }
    }
}

fn rotate(
    map: &mut Vec<Vec<u8>>,
    tmpmap: &mut Vec<Vec<u8>>,
) {
    for (r, c) in
        (0..map.len()).cartesian_product(0..map[0].len())
    {
        tmpmap[c][map.len() - 1 - r] = map[r][c];
    }
    std::mem::swap(map, tmpmap);
}

fn total_load(map: &Vec<Vec<u8>>) -> usize {
    (0..map.len())
        .map(|r| {
            (map.len() - r)
                * map[r]
                    .iter()
                    .filter(|&&t| t == b'O')
                    .count()
        })
        .sum()
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut map = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut tmpmap = vec![vec![0; map.len()]; map[0].len()];
    let mut seen = HashMap::new();
    for i in 1..1000000000 {
        for _ in 0..4 {
            roll_north(&mut map);
            rotate(&mut map, &mut tmpmap);
        }

        if let Some(seen_at) = seen.insert(map.clone(), i) {
            if (1000000000 - i) % (i - seen_at) == 0 {
                break;
            }
        }
    }

    Ok(total_load(&map).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("136", process(input)?);
        Ok(())
    }
}
