fn is_valid(target: usize, nums: &[usize], n: usize) -> bool {
    if nums.is_empty() {
        return n == target;
    }

    is_valid(target, &nums[1..], n + nums[0])
        || is_valid(target, &nums[1..], n * nums[0])
}

#[tracing::instrument(skip(input), ret)]
pub fn process(input: &str) -> miette::Result<String> {
    let ops = input.lines().map(|l| {
        let (test_value, rest) = l.split_once(": ").unwrap();
        let nums = rest
            .split(' ')
            .map(|w| w.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        (
            test_value.parse::<usize>().unwrap(),
            nums,
        )
    });

    let mut res = 0;
    for (t, nums) in ops {
        if !is_valid(t, &nums[1..], nums[0]) {
            continue;
        }
        res += t;
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("3749", process(input)?);
        Ok(())
    }
}
