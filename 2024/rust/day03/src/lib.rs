use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::map,
    multi::{many0, many_till},
    sequence::tuple,
    IResult,
};
use parser::FromDig;

type RowType = Op;
type OutType = i32;

#[derive(Debug)]
pub enum Op {
    Do,
    Dont,
    Mul(i32, i32),
}

fn parse_do(s: &str) -> IResult<&str, Op> {
    map(tag("do()"), |_| Op::Do)(s)
}

fn parse_dont(s: &str) -> IResult<&str, Op> {
    map(tag("don't()"), |_| Op::Dont)(s)
}

fn parse_mul(s: &str) -> IResult<&str, Op> {
    map(
        tuple((
            tag("mul("),
            <i32 as FromDig>::from_dig,
            tag(","),
            <i32 as FromDig>::from_dig,
            tag(")"),
        )),
        |(_, n1, _, n2, _)| Op::Mul(n1, n2),
    )(s)
}

fn parse_op(s: &str) -> IResult<&str, Op> {
    alt((parse_do, parse_dont, parse_mul))(s)
}

fn next_parse_op(s: &str) -> IResult<&str, Op> {
    map(many_till(anychar, parse_op), |(_, op)| op)(s)
}

pub fn parse(data: &str) -> impl Iterator<Item = RowType> + '_ {
    many0(next_parse_op)(data).unwrap().1.into_iter()
}

pub fn part1(input: impl Iterator<Item = RowType>) -> OutType {
    input
        .map(|op| match op {
            Op::Do => 0,
            Op::Dont => 0,
            Op::Mul(a, b) => a * b,
        })
        .sum()
}

pub fn part2(input: impl Iterator<Item = RowType>) -> OutType {
    input
        .fold((true, 0), |(emit, acc), op| match (emit, op) {
            (_, Op::Do) => (true, acc),
            (_, Op::Dont) => (false, acc),
            (true, Op::Mul(a, b)) => (true, (a * b) + acc),
            (false, _) => (false, acc),
        })
        .1
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        1,
        161
    );

    generate_test!(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        2,
        48
    );

    generate_test! { 2024, 3, 1, 161289189}
    generate_test! { 2024, 3, 2, 83595109}
}
