use crate::custom_error::AocError;

use regex::Regex;

#[tracing::instrument(skip(input), ret)]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let r = Regex::new(r"(mul\(\d+,\d+\)|do(n't)?\(\))").unwrap();

    let mut enabled = true;
    let mut res = 0;

    for m in r.find_iter(input) {
        match m.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            mul => {
                if !enabled {
                    continue;
                }

                let (x, y) = mul[4..mul.len() - 1].split_once(',').unwrap();
                res +=
                    x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap();
            }
        }
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
