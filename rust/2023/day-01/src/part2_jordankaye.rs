use crate::custom_error::AocError;

enum State {
    Empty,
    SingleDigit(u32),
    DoubleDigit(u32, u32),
}

impl Default for State {
    fn default() -> Self {
        Self::Empty
    }
}

impl State {
    pub fn update(self, val: u32) -> Self {
        match self {
            Self::Empty => Self::SingleDigit(val),
            Self::SingleDigit(a) => {
                Self::DoubleDigit(a, val)
            }
            Self::DoubleDigit(a, _) => {
                Self::DoubleDigit(a, val)
            }
        }
    }

    pub fn extract(self) -> u32 {
        match self {
            Self::SingleDigit(a) => a * 10 + a,
            Self::DoubleDigit(a, b) => a * 10 + b,
            _ => panic!("Invalid input, attempted to extract from empty state"),
        }
    }
}

const WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven",
    "eight", "nine",
];

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut result = 0u32;
    let mut state = State::default();
    let mut chars = input.trim().chars();
    let mut ch: Option<char>;
    while {
        let mut digit = 1;
        for word in WORDS {
            if chars.as_str().starts_with(word) {
                state = state.update(digit);
                for _ in 0..word.len() - 2 {
                    chars.next();
                }
                break;
            }
            digit += 1;
        }
        ch = chars.next();
        ch.is_some()
    } {
        if let Some(digit) = ch.unwrap().to_digit(10) {
            state = state.update(digit);
        } else if ch.unwrap() == '\n' {
            result += state.extract();
            state = State::default();
        }
    }
    result += state.extract();

    Ok(result.to_string())
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
