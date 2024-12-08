use hashbrown::{HashMap, HashSet};

#[tracing::instrument(skip(input), ret)]
pub fn process(input: &str) -> miette::Result<String> {
    let (rules_part, updates_part) = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::<usize, HashSet<usize>>::new();
    for r in rules_part.lines() {
        let (before, after) = r.split_once("|").unwrap();
        rules
            .entry(after.parse().unwrap())
            .or_default()
            .insert(before.parse().unwrap());
    }

    let updates = updates_part.lines().map(|line| {
        line.split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut res = 0;
    for mut u in updates {
        if u.is_sorted_by(|a, b| {
            rules.get(b).map_or(false, |set| set.contains(a))
        }) {
            continue;
        }

        u.sort_by(|a, b| {
            let a_in_b = rules.get(b).map_or(false, |set| set.contains(a));
            let b_in_a = rules.get(a).map_or(false, |set| set.contains(b));
            a_in_b.cmp(&b_in_a)
        });

        res += u[u.len() / 2];
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
