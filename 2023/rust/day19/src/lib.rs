use std::{
    collections::{HashMap, VecDeque},
    ops::RangeInclusive,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use parser::FromDig;

#[derive(Debug, Clone)]
pub enum Target {
    Workflow(String),
    Accept,
    Reject,
}

#[derive(Debug)]
pub enum RatingField {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
pub enum Operation {
    GT(RatingField, i64, Target),
    LT(RatingField, i64, Target),
    Apply(Target),
}

impl Operation {
    fn apply(&self, r: &Rating) -> Option<Target> {
        match self {
            Operation::GT(rf, i, t) => {
                if r.get_field(rf) > *i {
                    return Some(t.clone());
                }
                None
            }
            Operation::LT(rf, i, t) => {
                if r.get_field(rf) < *i {
                    return Some(t.clone());
                }
                None
            }
            Operation::Apply(t) => Some(t.clone()),
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    operations: Vec<Operation>,
}

impl Workflow {
    fn apply(&self, r: &Rating) -> Target {
        self.operations
            .iter()
            .find_map(|operation| operation.apply(r))
            .unwrap_or(Target::Reject)
    }
}

#[derive(Debug, Clone)]
pub struct Rating {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Rating {
    fn get_field(&self, rf: &RatingField) -> i64 {
        match rf {
            RatingField::X => self.x,
            RatingField::M => self.m,
            RatingField::A => self.a,
            RatingField::S => self.s,
        }
    }

    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
pub struct PartHeap {
    workflows: HashMap<String, Workflow>,
    ratings: Vec<Rating>,
}

#[derive(Clone, Debug)]
pub struct Paths {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl Paths {
    fn new() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }

    fn get_field(&self, rf: &RatingField) -> RangeInclusive<usize> {
        match rf {
            RatingField::X => self.x.clone(),
            RatingField::M => self.m.clone(),
            RatingField::A => self.a.clone(),
            RatingField::S => self.s.clone(),
        }
    }

    fn merge_field(&self, rf: &RatingField, r: RangeInclusive<usize>) -> Self {
        let Self { x, m, a, s } = self;
        match rf {
            RatingField::X => Self {
                x: r,
                m: m.clone(),
                a: a.clone(),
                s: s.clone(),
            },
            RatingField::M => Self {
                x: x.clone(),
                m: r,
                a: a.clone(),
                s: s.clone(),
            },
            RatingField::A => Self {
                x: x.clone(),
                m: m.clone(),
                a: r,
                s: s.clone(),
            },
            RatingField::S => Self {
                x: x.clone(),
                m: m.clone(),
                a: a.clone(),
                s: r,
            },
        }
    }

    fn split(&self, rf: &RatingField, x: usize) -> (Self, Self) {
        let r = self.get_field(&rf);

        let a = *r.start()..=(x - 1);
        let b = x..=*r.end();
        (self.merge_field(&rf, a), self.merge_field(&rf, b))
    }

    fn possible_path_count(&self) -> usize {
        (self.x.end() - self.x.start() + 1)
            * (self.m.end() - self.m.start() + 1)
            * (self.a.end() - self.a.start() + 1)
            * (self.s.end() - self.s.start() + 1)
    }
}

impl PartHeap {
    fn apply(&self) -> i64 {
        self.ratings
            .iter()
            .filter(|rating| {
                let mut step = self.workflows.get("in").unwrap();
                loop {
                    match step.apply(rating) {
                        Target::Workflow(wf) => {
                            step = self.workflows.get(&wf).unwrap();
                        }
                        Target::Accept => return true,
                        Target::Reject => return false,
                    }
                }
            })
            .map(|rating| rating.sum())
            .sum()
    }

    fn count_possible(&self) -> usize {
        let mut good = Vec::new();

        let mut q = VecDeque::new();
        q.push_front((self.workflows.get("in").unwrap(), Paths::new()));

        while let Some((workflow, paths)) = q.pop_front() {
            let mut remaining_paths = paths.clone();

            for operation in workflow.operations.iter() {
                let (target, good_paths, other_paths) = match operation {
                    Operation::GT(rf, i, t) => {
                        let (lower, higher) =
                            remaining_paths.split(rf, (*i + 1).try_into().unwrap());
                        (t, higher, Some(lower))
                    }
                    Operation::LT(rf, i, t) => {
                        let (lower, higher) = remaining_paths.split(rf, (*i).try_into().unwrap());
                        (t, lower, Some(higher))
                    }
                    Operation::Apply(t) => (t, remaining_paths.clone(), None),
                };

                match target {
                    Target::Workflow(w) => {
                        q.push_back((self.workflows.get(w).unwrap(), good_paths))
                    }
                    Target::Accept => good.push(good_paths),
                    Target::Reject => (),
                }

                match other_paths {
                    Some(paths) => remaining_paths = paths,
                    None => break,
                }
            }
        }

        good.into_iter()
            .map(|paths| paths.possible_path_count())
            .sum()
    }
}

fn parse_target(s: &str) -> IResult<&str, Target> {
    alt((
        map(tag("A"), |_| Target::Accept),
        map(tag("R"), |_| Target::Reject),
        map(alpha1, |s: &str| Target::Workflow(s.to_string())),
    ))(s)
}

fn parse_rating_field(s: &str) -> IResult<&str, RatingField> {
    alt((
        map(tag("x"), |_| RatingField::X),
        map(tag("m"), |_| RatingField::M),
        map(tag("a"), |_| RatingField::A),
        map(tag("s"), |_| RatingField::S),
    ))(s)
}

fn parse_gt(s: &str) -> IResult<&str, Operation> {
    let (rest, (rf, _, i, _, target)) = tuple((
        parse_rating_field,
        tag(">"),
        <i64 as FromDig>::from_dig,
        tag(":"),
        parse_target,
    ))(s)?;
    Ok((rest, Operation::GT(rf, i, target)))
}

fn parse_lt(s: &str) -> IResult<&str, Operation> {
    let (rest, (rf, _, i, _, target)) = tuple((
        parse_rating_field,
        tag("<"),
        <i64 as FromDig>::from_dig,
        tag(":"),
        parse_target,
    ))(s)?;
    Ok((rest, Operation::LT(rf, i, target)))
}

fn parse_apply(s: &str) -> IResult<&str, Operation> {
    map(parse_target, |t| Operation::Apply(t))(s)
}

fn parse_operation(s: &str) -> IResult<&str, Operation> {
    alt((parse_gt, parse_lt, parse_apply))(s)
}

fn parse_workflow(s: &str) -> IResult<&str, (String, Workflow)> {
    let (rest, name) = alpha1(s)?;
    let (rest, operations) = delimited(
        tag("{"),
        separated_list1(tag(","), parse_operation),
        tag("}"),
    )(rest)?;

    Ok((rest, (name.to_string(), Workflow { operations })))
}

fn parse_workflows(s: &str) -> IResult<&str, HashMap<String, Workflow>> {
    map(separated_list1(newline, parse_workflow), |workflows| {
        workflows.into_iter().collect::<HashMap<String, Workflow>>()
    })(s)
}

fn parse_rating(s: &str) -> IResult<&str, Rating> {
    delimited(
        tag("{"),
        map(
            tuple((
                preceded(tag("x="), <i64 as FromDig>::from_dig),
                preceded(tag(",m="), <i64 as FromDig>::from_dig),
                preceded(tag(",a="), <i64 as FromDig>::from_dig),
                preceded(tag(",s="), <i64 as FromDig>::from_dig),
            )),
            |(x, m, a, s)| Rating { x, m, a, s },
        ),
        tag("}"),
    )(s)
}

fn parse_ratings(s: &str) -> IResult<&str, Vec<Rating>> {
    separated_list1(newline, parse_rating)(s)
}

pub fn parse<'a>(data: &'a str) -> PartHeap {
    let (rest, workflows) = terminated(parse_workflows, many1(newline))(data).unwrap();
    let (rest, ratings) = parse_ratings(rest).unwrap();

    assert_eq!(rest.trim(), "");

    PartHeap { workflows, ratings }
}

pub fn part1<'a>(part_heap: PartHeap) -> i64 {
    part_heap.apply()
}

pub fn part2<'a>(part_heap: PartHeap) -> usize {
    part_heap.count_possible()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 19114);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 167409079868000);
    }

    generate_test! { 2023, 19, 1, 353046}
    generate_test! { 2023, 19, 2, 125355665599537}
}
