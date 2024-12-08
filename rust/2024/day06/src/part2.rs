use itertools::Itertools;

fn walk(
    map: &[Vec<u8>],
    mut r: usize,
    mut c: usize,
    return_squares: bool,
) -> Option<Vec<(usize, usize)>> {
    let mut seen = vec![vec![[false; 4]; map[0].len()]; map.len()];
    let mut d = 0;

    loop {
        if seen[r][c][d] {
            return None;
        }

        seen[r][c][d] = true;

        // Directions
        let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let (dr, dc) = directions[d];

        // New coordinates
        let next_row = r as isize + dr;
        let next_column = c as isize + dc;

        // Check new coordinates are within the map bounds
        if next_row < 0
            || next_row >= map.len() as isize
            || next_column < 0
            || next_column >= map[0].len() as isize
        {
            if !return_squares {
                return Some(Vec::new());
            }

            let visited = (0..map.len())
                .flat_map(|r| (0..map[0].len()).map(move |c| (r, c)))
                .filter(|&(r, c)| seen[r][c].iter().any(|&b| b))
                .collect();
            return Some(visited);
        }

        // Convert back to usize
        let (rr, cc) = (next_row as usize, next_column as usize);

        if map[rr][cc] == b'#' {
            d = (d + 1) % 4; // Turn right
        } else {
            r = rr;
            c = cc;
        }
    }
}

#[tracing::instrument(skip(input), ret)]
pub fn process(input: &str) -> miette::Result<String> {
    let mut map = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let (start_row, start_column) = (0..map.len())
        .cartesian_product(0..map[0].len())
        .find(|&(row, column)| map[row][column] == b'^')
        .unwrap();

    let part1 = walk(&map, start_row, start_column, true).unwrap();
    let res = part1
        .iter()
        .filter(|&&(row, column)| {
            map[row][column] = b'#';
            let ok = walk(&map, start_row, start_column, false).is_none();
            map[row][column] = b'.';
            ok
        })
        .count();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
