use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

fn main() {
    let input = include_str!("../input.txt");
    let output = part_1(input);
    println!("{output}");

    let output = part_2(input);
    println!("{output}");
}

fn part_1(input: &str) -> usize {
    let mut input = input.lines();

    let mut workflows = HashMap::new();
    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }

        let (name, workflow) = parse_workflow(line);
        workflows.insert(name, workflow);
    }

    let mut parts = Vec::new();
    while let Some(line) = input.next() {
        let part = parse_part(line);
        parts.push(part);
    }

    let mut total = 0;

    for p in parts {
        if is_part_accepted(&p, &workflows) {
            for v in p {
                total += v;
            }
        }
    }

    total
}

fn part_2(input: &str) -> usize {
    let mut input = input.lines();

    let mut workflows = HashMap::new();
    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }

        let (name, workflow) = parse_workflow(line);
        workflows.insert(name, workflow);
    }

    let all_part_ranges = [1..4001, 1..4001, 1..4001, 1..4001];

    let mut total = 0;
    let mut pending = VecDeque::<(&str, Part)>::new();
    pending.push_back(("in", all_part_ranges));

    while let Some((dest, part)) = pending.pop_front() {
        if dest == ACCEPTED {
            let mut combinations = 1;
            for category in part {
                combinations *= category.end - category.start;
            }
            total += combinations;
            continue;
        }

        if dest == REJECTED {
            continue;
        }

        let workflow = &workflows[dest];
        check_workflow(workflow, part, &mut pending);
    }

    total
}

fn check_workflow<'a>(
    workflow: &Workflow<'a>,
    mut part: Part,
    pending: &mut VecDeque<(&'a str, Part)>,
) {
    for rule in workflow.rules.iter() {
        let value = part[rule.key].clone();
        match rule.op {
            ComparisonOp::Less => {
                if value.start < rule.value {
                    if value.end <= rule.value {
                        pending.push_back((rule.destination, part));
                        return;
                    } else {
                        let mut new_part = part.clone();
                        new_part[rule.key] = value.start..rule.value;
                        pending.push_back((rule.destination, new_part));
                        part[rule.key] = rule.value..value.end;
                    }
                }
            }
            ComparisonOp::Greater => {
                if value.end >= rule.value {
                    if value.start > rule.value {
                        pending.push_back((rule.destination, part));
                        return;
                    } else {
                        let mut new_part = part.clone();
                        new_part[rule.key] = rule.value + 1..value.end;
                        pending.push_back((rule.destination, new_part));
                        part[rule.key] = value.start..rule.value + 1;
                    }
                }
            }
        }
    }

    pending.push_back((workflow.fallback, part));
}

fn is_part_accepted(part: &[usize; 4], workflows: &HashMap<&str, Workflow>) -> bool {
    let mut current = "in";
    loop {
        let workflow = &workflows[current];
        current = workflow.get_destination(part);
        if current == ACCEPTED || current == REJECTED {
            break;
        }
    }

    current == ACCEPTED
}

fn parse_workflow(input: &str) -> (&str, Workflow) {
    let (name, rest) = input.split_once('{').unwrap();
    let rest = &rest[..rest.len() - 1];

    let mut workflow = Workflow {
        rules: Vec::new(),
        fallback: "",
    };
    for rule in rest.split(',') {
        if let Some((expr, dest)) = rule.split_once(':') {
            if let Some((lhs, rhs)) = expr.split_once('>') {
                workflow.rules.push(WorkflowRule {
                    destination: dest,
                    key: key_to_index(lhs),
                    op: ComparisonOp::Greater,
                    value: rhs.parse().unwrap(),
                });
            } else if let Some((lhs, rhs)) = expr.split_once('<') {
                workflow.rules.push(WorkflowRule {
                    destination: dest,
                    key: key_to_index(lhs),
                    op: ComparisonOp::Less,
                    value: rhs.parse().unwrap(),
                });
            } else {
                unreachable!();
            }
        } else {
            workflow.fallback = rule;
        }
    }

    (name, workflow)
}

fn parse_part(input: &str) -> [usize; 4] {
    let mut out = [0; 4];

    for (l, r) in input[1..input.len() - 1]
        .split(',')
        .map(|it| it.split_once('=').unwrap())
    {
        let i = key_to_index(l);
        out[i] = r.parse().unwrap();
    }
    out
}

fn key_to_index(key: &str) -> usize {
    match key {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => unreachable!(),
    }
}

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";

type Part = [Range<usize>; 4];

struct Workflow<'a> {
    rules: Vec<WorkflowRule<'a>>,
    fallback: &'a str,
}

impl Workflow<'_> {
    fn get_destination(&self, part: &[usize; 4]) -> &str {
        for rule in self.rules.iter() {
            let value = part[rule.key];
            let is_match = match rule.op {
                ComparisonOp::Less => value < rule.value,
                ComparisonOp::Greater => value > rule.value,
            };
            if is_match {
                return rule.destination;
            }
        }

        self.fallback
    }
}

struct WorkflowRule<'a> {
    key: usize,
    op: ComparisonOp,
    value: usize,
    destination: &'a str,
}

#[derive(Clone, Copy)]
enum ComparisonOp {
    Less,
    Greater,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"px{a<2006:qkq,m>2090:A,rfg}
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

    #[test]
    fn part_1_basic() {
        let output = part_1(INPUT);
        assert_eq!(output, 19114);
    }

    #[test]
    fn part_2_test() {
        let output = part_2(INPUT);
        assert_eq!(output, 167409079868000);
    }
}
