use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &[u8],
) -> miette::Result<String, AocError> {
    let width =
        input.iter().position(|&b| b == b'\n').unwrap();
    let start =
        input.iter().position(|&b| b == b'S').unwrap();

    let (mut pos, mut dir) = {
        if matches!(
            input[start - width - 1],
            b'|' | b'7' | b'F'
        ) {
            (start - width - 1, Dir::Up)
        } else if matches!(
            input[start + width + 1],
            b'|' | b'L' | b'J'
        ) {
            (start + width + 1, Dir::Down)
        } else {
            (start - 1, Dir::Left)
        }
    };

    let final_pos = format!(
        "{}",
        (1 + std::iter::repeat(())
            .position(|_| unsafe {
                match (input.get_unchecked(pos), dir) {
                    (b'|', Dir::Down) => pos += width + 1,
                    (b'|', Dir::Up) => pos -= width + 1,
                    (b'-', Dir::Left) => pos -= 1,
                    (b'-', Dir::Right) => pos += 1,
                    (b'L', Dir::Down) | (b'F', Dir::Up) => {
                        pos += 1;
                        dir = Dir::Right;
                    }
                    (b'L', Dir::Left)
                    | (b'J', Dir::Right) => {
                        pos -= width + 1;
                        dir = Dir::Up;
                    }
                    (b'7', Dir::Up) | (b'J', Dir::Down) => {
                        pos -= 1;
                        dir = Dir::Left;
                    }
                    (b'7', Dir::Right)
                    | (b'F', Dir::Left) => {
                        pos += width + 1;
                        dir = Dir::Down;
                    }
                    (b'S', _) => return true,
                    (_, _) => unreachable!(),
                }
                false
            })
            .unwrap())
            / 2
    );

    Ok(final_pos)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        "4"
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        "8"
    )]
    #[test_log::test]
    fn test_process(
        #[case] input: &str,
        #[case] output: &str,
    ) -> miette::Result<()> {
        assert_eq!(output, process(input.as_bytes())?);
        Ok(())
    }
}
