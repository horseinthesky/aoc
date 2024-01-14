use crate::custom_error::AocError;

fn roll_north(map: &mut Vec<Vec<u8>>) {
    // Rock can roll as many times as there are rows in them map minus one
    for row in 0..map.len() - 1 {
        // Rotation needs to be done as many time as there are positions in the row
        for char in 0..map[0].len() {
            // For each char in the row we check if there is a rock on this position in the next row
            // If not we check next char
            if map[row + 1][char] != b'O' {
                continue;
            }

            // If there is a rock we need to roll it all the way north
            // We use rev() to roll the rock north
            for reversed_row in (0..=row).rev() {
                // If position if current row is NOT empty we stop rolling
                if map[reversed_row][char] != b'.' {
                    break;
                }

                // Roll the rock from next row to current one
                // And free position on the next row after that
                map[reversed_row][char] = b'O';
                map[reversed_row + 1][char] = b'.';
            }
        }
    }
}

fn total_load(map: &Vec<Vec<u8>>) -> usize {
    (0..map.len())
        .map(|row| {
            (map.len() - row)
                * map[row]
                    .iter()
                    .filter(|&&char| char == b'O')
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
        .map(|line| line.as_bytes().to_vec())
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
