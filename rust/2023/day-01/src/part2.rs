use crate::custom_error::AocError;

const STR_DIGITS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six",
    b"seven", b"eight", b"nine",
];

fn digit_sum(w: &[u8], part_two: bool) -> usize {
    let mut digits =
        (0..w.len()).filter_map(|i| match w[i] {
            b'0'..=b'9' => Some((w[i] - b'0') as usize),
            _ if part_two => STR_DIGITS
                .iter()
                .enumerate()
                .find_map(|(di, d)| {
                    w[i..].starts_with(d).then_some(di + 1)
                }),
            _ => None,
        });
    let a = digits.next().unwrap();
    let b = digits.last().unwrap_or(a);
    a * 10 + b
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut res = 0;
    for l in input.lines() {
        res += digit_sum(l.as_bytes(), true);
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
