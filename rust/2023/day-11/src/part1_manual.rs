use crate::custom_error::AocError;

const INC: usize = 1;

#[inline(always)]
fn dist(counts: &[usize]) -> usize {
    let (mut gaps, mut sum, mut items, mut dist) = (0, 0, 0, 0);
    for (i, count) in counts.iter().enumerate() {
        if *count > 0 {
            let expanded = i + INC * gaps;
            dist += count * (items * expanded - sum);
            sum += count * expanded;
            items += count;
        } else {
            gaps += 1;
        }
    }
    dist
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &[u8],
) -> miette::Result<String, AocError> {
    let size = input.iter().position(|&b| b == b'\n').unwrap();
    let (mut xx, mut yy) = (vec![0; size], vec![0; size]);

    input.iter()
        .enumerate()
        .filter(|(_, &b)| b == b'#')
        .for_each(|(pos, _)| {
            xx[pos % (size + 1)] += 1;
            yy[pos / (size + 1)] += 1;
        });

    Ok(format!("{}", dist(&xx) + dist(&yy)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", process(input.as_bytes())?);
        Ok(())
    }
}
