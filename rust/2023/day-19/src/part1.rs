use hashbrown::HashMap;
use itertools::Itertools;

use crate::custom_error::AocError;

type WorkFlows<'a> = HashMap<
    &'a str,
    (
        Vec<(char, bool, usize, &'a str)>,
        &'a str,
    ),
>;

fn filter(
    workflows: &WorkFlows<'_>,
    vals: [usize; 4],
) -> bool {
    let mut curr = "in";
    while curr != "A" && curr != "R" {
        let workflow = &workflows[curr];
        curr = workflow
            .0
            .iter()
            .find(|&&(p, lt, n, _)| {
                let i = "xmas"
                    .chars()
                    .position(|c| c == p)
                    .unwrap();
                if lt {
                    vals[i] < n
                } else {
                    vals[i] > n
                }
            })
            .map(|&(_, _, _, label)| label)
            .unwrap_or(workflow.1);
    }
    curr == "A"
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (workflows, parts) =
        input.split_once("\n\n").unwrap();

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

    let res = parts
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_ascii_digit())
                .filter(|l| !l.is_empty())
                .map(|w| w.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .filter(|&(x, m, a, s)| {
            filter(&workflows, [x, m, a, s])
        })
        .map(|(x, m, a, s)| x + m + a + s)
        .sum::<usize>();

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
        assert_eq!("19114", process(input)?);
        Ok(())
    }
}
