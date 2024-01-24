use std::cmp::Ordering;
use std::collections::HashMap;
use tracing::info;

use crate::custom_error::AocError;

#[derive(Debug)]
enum Action<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

#[derive(Debug)]
enum Condition {
    Greater,
    Less,
}

#[derive(Debug)]
struct Step<'a> {
    param: char,
    cond: Condition,
    value: u32,
    action: Action<'a>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    steps: Vec<Step<'a>>,
    last_resort: Action<'a>,
}

#[derive(Debug)]
struct Part(u32, u32, u32, u32);

impl Part {
    fn rating_by(&self, name: char) -> u32 {
        match name {
            'x' => self.0,
            'm' => self.1,
            'a' => self.2,
            's' => self.3,
            _ => unreachable!(),
        }
    }
}

fn parse_workflows(input: &str) -> HashMap<&str, Workflow> {
    input
        .lines()
        .map(|w| {
            let (name, rest) = w.split_once("{").unwrap();

            let (rest, last_resort_dirty) = rest
                [0..rest.len() - 1]
                .split_at(rest.rfind(',').unwrap());

            // last_resort_dirty is ',A' or ',rr' or something like that
            // so we ignore 1st char
            let last_resort = match &last_resort_dirty[1..]
            {
                "A" => Action::Accept,
                "R" => Action::Reject,
                workflow_label => {
                    Action::Workflow(workflow_label)
                }
            };

            let steps = rest
                .split(',')
                .map(|rule| {
                    let (rest, workflow_label) =
                        rule.split_once(':').unwrap();

                    let lt = rest.contains('<');
                    let (param, value) = rest
                        .split_once(if lt {
                            '<'
                        } else {
                            '>'
                        })
                        .unwrap();

                    Step {
                        param: param.as_bytes()[0] as char,
                        cond: if lt {
                            Condition::Less
                        } else {
                            Condition::Greater
                        },
                        value: value
                            .parse::<u32>()
                            .unwrap(),
                        action: match workflow_label {
                            "A" => Action::Accept,
                            "R" => Action::Reject,
                            workflow_label => {
                                Action::Workflow(
                                    workflow_label,
                                )
                            }
                        },
                    }
                })
                .collect::<Vec<_>>();

            (
                name,
                Workflow {
                    name,
                    steps,
                    last_resort,
                },
            )
        })
        .collect::<HashMap<_, _>>()
}

fn parse_parts(input: &str) -> Vec<Part> {
    input
        .lines()
        .map(|l| {
            let mut part = Part(0, 0, 0, 0);

            let params = l[1..l.len() - 1].split(",");
            params.for_each(|p| {
                let (param, value) =
                    p.split_once("=").unwrap();
                match param {
                    "x" => {
                        part.0 =
                            value.parse::<u32>().unwrap()
                    }
                    "m" => {
                        part.1 =
                            value.parse::<u32>().unwrap()
                    }
                    "a" => {
                        part.2 =
                            value.parse::<u32>().unwrap()
                    }
                    "s" => {
                        part.3 =
                            value.parse::<u32>().unwrap()
                    }
                    _ => unreachable!(),
                }
            });

            part
        })
        .collect()
}

fn is_accepted(
    part: &Part,
    workflow: &Workflow,
    workflows: &HashMap<&str, Workflow>,
) -> bool {
    for step in workflow.steps.iter() {
        let cond = match step.cond {
            Condition::Greater => Ordering::Greater,
            Condition::Less => Ordering::Less,
        };

        if part.rating_by(step.param).cmp(&step.value)
            == cond
        {
            match &step.action {
                Action::Accept => return true,
                Action::Reject => return false,
                Action::Workflow(name) => {
                    return is_accepted(
                        part,
                        &workflows[name],
                        workflows,
                    )
                }
            };
        };
    }

    match &workflow.last_resort {
        Action::Accept => true,
        Action::Reject => false,
        Action::Workflow(name) => {
            is_accepted(part, &workflows[name], workflows)
        }
    }
}

#[tracing::instrument(skip(input))]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (workflows_raw, parts_raw) =
        input.split_once("\n\n").unwrap();

    let workflows = parse_workflows(workflows_raw);
    info!(?workflows);
    let parts = parse_parts(parts_raw);

    let res = parts
        .iter()
        .filter(|p| {
            let init_workflow = &workflows["in"];

            is_accepted(p, init_workflow, &workflows)
        })
        .map(|p| p.0 + p.1 + p.2 + p.3)
        .sum::<u32>();

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
