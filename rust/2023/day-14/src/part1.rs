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
    let map = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let result = {
        let mut map = map.clone();
        roll_north(&mut map);
        total_load(&map)
    };

    Ok(result.to_string())
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
