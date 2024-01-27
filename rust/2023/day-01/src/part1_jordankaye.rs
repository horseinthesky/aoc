use crate::custom_error::AocError;
use tracing::info;

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

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut result = 0u32;
    let mut state = State::default();
    let mut chars = input.trim().chars();
    while let Some(ch) = chars.next() {
        if let Some(digit) = ch.to_digit(10) {
            state = state.update(digit);
        } else if ch == '\n' {
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
