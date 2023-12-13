use crate::custom_error::AocError;

#[tracing::instrument]
fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        match &line[index..] {
            line if line.starts_with("one") => Some(1),
            line if line.starts_with("two") => Some(2),
            line if line.starts_with("three") => Some(3),
            line if line.starts_with("four") => Some(4),
            line if line.starts_with("five") => Some(5),
            line if line.starts_with("six") => Some(6),
            line if line.starts_with("seven") => Some(7),
            line if line.starts_with("eight") => Some(8),
            line if line.starts_with("nine") => Some(9),
            line => {
                line.chars().next().unwrap().to_digit(10)
            }
        }
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let output =
        input.lines().map(process_line).sum::<u32>();

    Ok(output.to_string())
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
