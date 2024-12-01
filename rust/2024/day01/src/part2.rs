use hashbrown::HashMap;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        left.push(nums[0]);
        right.push(nums[1]);
    }

    let mut counter: HashMap<&i32, i32> = HashMap::new();

    for right_num in right.iter() {
        let entry = counter.entry(right_num).or_default();
        *entry += 1;
    }

    let res = left
        .iter()
        .map(|num| {
            let entry = counter.entry(num).or_default();
            *num * *entry
        })
        .sum::<i32>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
