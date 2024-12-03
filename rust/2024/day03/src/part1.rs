use crate::custom_error::AocError;

use regex::Regex;

#[tracing::instrument(skip(input), ret)]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    // use capture groups for `(\d+)` to ref them later as `cap[1]`
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res = re
        .captures_iter(input)
        .map(|cap| {
            let x = cap[1].parse::<i32>().unwrap();
            let y = cap[2].parse::<i32>().unwrap();
            x*y
        })
        .sum::<i32>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
