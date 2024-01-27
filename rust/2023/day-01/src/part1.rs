use crate::custom_error::AocError;

fn digit_sum(w: &[u8]) -> usize {
    let mut digits =
        (0..w.len()).filter_map(|i| match w[i] {
            b'0'..=b'9' => Some((w[i] - b'0') as usize),
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
        res += digit_sum(l.as_bytes());
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
