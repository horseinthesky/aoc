use itertools::Itertools;
use tracing::info;

use crate::custom_error::AocError;

fn solve(
    hailstones: &[(f64, f64, f64, f64, f64, f64)],
) -> usize {
    hailstones
        .iter()
        .tuple_combinations()
        .filter(
            |(
                &(x1, y1, _, dx1, dy1, _),
                &(x2, y2, _, dx2, dy2, _),
            )| {
                let m1 = dy1 / dx1;
                let m2 = dy2 / dx2;
                if (m2 - m1).abs() <= f64::EPSILON {
                    return false;
                }

                let x = (m1 * x1 - m2 * x2 + y2 - y1)
                    / (m1 - m2);
                let y = (m1 * m2 * (x2 - x1) + m2 * y1
                    - m1 * y2)
                    / (m2 - m1);
                if dx1.signum() != (x - x1).signum()
                    || dx2.signum() != (x - x2).signum()
                {
                    return false;
                }

                [x, y].iter().all(|v| {
                    (200000000000000.0..=400000000000000.0)
                        .contains(v)
                })
            },
        )
        .count()
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let hailstones = input
        .lines()
        .map(|l| {
            l.split(['@', ','])
                .map(|w| w.trim().parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();

    info!(?hailstones);

    Ok(solve(&hailstones).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}

//
