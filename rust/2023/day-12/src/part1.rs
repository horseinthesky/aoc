use std::collections::HashMap;

use crate::custom_error::AocError;

fn possible_ways(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    s: &[u8],
    within: Option<usize>,
    remaining: &[usize],
) -> usize {
    if s.is_empty() {
        return match (within, remaining.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining[0] => 1,
            _ => 0,
        };
    }
    if within.is_some() && remaining.is_empty() {
        return 0;
    }

    let key = (
        s.len(),
        within.unwrap_or(0),
        remaining.len(),
    );
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let ways = match (s[0], within) {
        (b'.', Some(x)) if x != remaining[0] => 0,
        (b'.', Some(_)) => possible_ways(
            cache,
            &s[1..],
            None,
            &remaining[1..],
        ),
        (b'.', None) => {
            possible_ways(cache, &s[1..], None, remaining)
        }
        (b'#', Some(_)) => possible_ways(
            cache,
            &s[1..],
            within.map(|x| x + 1),
            remaining,
        ),
        (b'#', None) => possible_ways(
            cache,
            &s[1..],
            Some(1),
            remaining,
        ),
        (b'?', Some(x)) => {
            let mut ans = possible_ways(
                cache,
                &s[1..],
                within.map(|x| x + 1),
                remaining,
            );
            if x == remaining[0] {
                ans += possible_ways(
                    cache,
                    &s[1..],
                    None,
                    &remaining[1..],
                )
            }
            ans
        }
        (b'?', None) => {
            possible_ways(
                cache,
                &s[1..],
                Some(1),
                remaining,
            ) + possible_ways(
                cache,
                &s[1..],
                None,
                remaining,
            )
        }
        _ => unreachable!(),
    };

    cache.insert(key, ways);
    ways
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut cache = HashMap::new();

    let mut res = 0;

    input.lines().for_each(|l| {
        let (vents, rest) = l.split_once(" ").unwrap();
        let nums = rest
            .split(',')
            .map(|w| w.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        cache.clear();

        res += possible_ways(
            &mut cache,
            vents.as_bytes(),
            None,
            &nums,
        );
    });

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
