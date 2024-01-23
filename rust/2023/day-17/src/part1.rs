use std::collections::{BinaryHeap, HashMap};

use crate::custom_error::AocError;

fn dijkstra(
    grid: &[&[u8]],
    minstep: isize,
    maxstep: isize,
) -> i64 {
    let mut dists = HashMap::new();
    let mut q =
        BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);

    while let Some((cost, (r, c, d))) = q.pop() {
        if (r, c) == (grid.len() - 1, grid[0].len() - 1) {
            return -cost;
        }

        if dists.get(&(r, c, d)).is_some_and(|&c| -cost > c)
        {
            continue;
        }

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if d == (dr, dc) || d == (-dr, -dc) {
                continue;
            }

            let mut next_cost = -cost;
            for dist in 1..=maxstep {
                let rr = (r as isize + dr * dist) as usize;
                let cc = (c as isize + dc * dist) as usize;
                if rr >= grid.len() || cc >= grid[0].len() {
                    break;
                }

                next_cost += (grid[rr][cc] - b'0') as i64;
                if dist < minstep {
                    continue;
                }

                let key = (rr, cc, (dr, dc));
                if next_cost
                    < *dists.get(&key).unwrap_or(&i64::MAX)
                {
                    dists.insert(key, next_cost);
                    q.push((-next_cost, key));
                }
            }
        }
    }

    unreachable!()
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let grid = input
        .lines()
        .map(str::as_bytes)
        .collect::<Vec<_>>();

    let res = dijkstra(&grid, 1, 3);
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
        assert_eq!("102", process(input)?);
        Ok(())
    }
}
