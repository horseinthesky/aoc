use itertools::Itertools;
use z3::ast::{Ast, Int, Real};

use crate::custom_error::AocError;

fn solve(
    hailstones: &[(f64, f64, f64, f64, f64, f64)],
) -> usize {
    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"]
            .map(|v| Real::new_const(&ctx, v));
    let zero = Int::from_i64(&ctx, 0).to_real();
    for (i, &(x, y, z, dx, dy, dz)) in
        hailstones[..3].iter().enumerate()
    {
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz]
            .map(|v| Int::from_i64(&ctx, v as _).to_real());
        let t = Real::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(
            &((&x + &dx * &t)._eq(&(&fx + &fdx * &t))),
        );
        s.assert(
            &((&y + &dy * &t)._eq(&(&fy + &fdy * &t))),
        );
        s.assert(
            &((&z + &dz * &t)._eq(&(&fz + &fdz * &t))),
        );
    }

    assert_eq!(s.check(), z3::SatResult::Sat);
    let res = s
        .get_model()
        .unwrap()
        .eval(&(&fx + &fy + &fz), true)
        .unwrap();

    res.to_string()
        .strip_suffix(".0")
        .unwrap()
        .parse()
        .unwrap()
}

#[tracing::instrument]
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
