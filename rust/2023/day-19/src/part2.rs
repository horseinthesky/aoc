use crate::custom_error::AocError;
use std::collections::HashMap;

type WorkFlows<'a> = HashMap<
    &'a str,
    (
        Vec<(char, bool, usize, &'a str)>,
        &'a str,
    ),
>;

fn count_accepted(
    workflows: &WorkFlows<'_>,
    curr: &str,
    mut ranges: [Vec<usize>; 4],
) -> usize {
    if curr == "A" {
        return ranges.iter().map(|v| v.len()).product();
    }
    if curr == "R" {
        return 0;
    }
    let mut ans = 0;
    let workflow = &workflows[curr];
    for &(p, lt, n, label) in &workflow.0 {
        let i =
            "xmas".chars().position(|c| c == p).unwrap();
        let mut newranges = ranges.clone();
        (newranges[i], ranges[i]) = ranges[i]
            .iter()
            .partition(
                |&&val| if lt { val < n } else { val > n },
            );
        ans += count_accepted(workflows, label, newranges);
    }
    ans += count_accepted(workflows, workflow.1, ranges);
    ans
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (workflows, _) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|l| {
            let (name, rest) = l.split_once('{').unwrap();
            let (rest, label) = rest[0..rest.len() - 1]
                .split_at(rest.rfind(',').unwrap());
            let rules = rest
                .split(',')
                .map(|rule| {
                    let (rest, label) =
                        rule.split_once(':').unwrap();
                    let lt = rest.contains('<');
                    let (name, n) = rest
                        .split_once(if lt {
                            '<'
                        } else {
                            '>'
                        })
                        .unwrap();
                    (
                        name.as_bytes()[0] as char,
                        lt,
                        n.parse::<usize>().unwrap(),
                        label,
                    )
                })
                .collect::<Vec<_>>();
            (name, (rules, &label[1..]))
        })
        .collect::<HashMap<_, _>>();

    let res = count_accepted(
        &workflows,
        "in",
        std::array::from_fn(|_| {
            (1..=4000).collect::<Vec<_>>()
        }),
    );
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!("167409079868000", process(input)?);
        Ok(())
    }
}
