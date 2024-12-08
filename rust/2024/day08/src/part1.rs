use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[tracing::instrument(skip(input), ret)]
pub fn process(input: &str) -> miette::Result<String> {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut nodes = HashMap::<_, Vec<_>>::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != b'.' {
                nodes
                    .entry(grid[i][j])
                    .or_default()
                    .push((i as isize, j as isize));
            }
        }
    }

    let mut res = HashSet::new();
    for v in nodes.values() {
        for (&a, &b) in v.iter().tuple_combinations() {
            for ((r1, c1), (r2, c2)) in [(a, b), (b, a)] {
                let (dr, dc) = (r2 - r1, c2 - c1);
                let c = c2 + dc;
                let r = r2 + dr;

                if (0..grid.len() as isize).contains(&r)
                    && (0..grid[0].len() as isize).contains(&c)
                {
                    res.insert((r, c));
                }
            }
        }
    }

    Ok(res.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
