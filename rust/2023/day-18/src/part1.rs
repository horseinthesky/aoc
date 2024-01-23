use crate::custom_error::AocError;

fn calc_area(
    instructions: impl Iterator<Item = (u8, isize)>,
) -> isize {
    let (mut r, mut c, mut area) = (0, 0, 0);
    for (direction, amount_of_cubes) in instructions {
        let (rr, cc) = (r, c);
        match direction {
            b'R' => c += amount_of_cubes,
            b'D' => r += amount_of_cubes,
            b'L' => c -= amount_of_cubes,
            b'U' => r -= amount_of_cubes,
            _ => unreachable!(),
        }

        area += (c + cc) * (r - rr) + amount_of_cubes;
    }

    area / 2 + 1
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let iterator_over_direction_and_amount_of_lava_cubes =
        input.lines().map(|l| {
            let (n, _) = l[2..].split_once(' ').unwrap();
            (l.as_bytes()[0], n.parse().unwrap())
        });

    Ok(calc_area(
        iterator_over_direction_and_amount_of_lava_cubes,
    )
    .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!("62", process(input)?);
        Ok(())
    }
}
