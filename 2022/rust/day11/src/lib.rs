use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

pub fn part1<'a>(mut monkeys: Vec<Monkey>) -> u64 {
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[idx].items.drain(..).collect();
            monkeys[idx].inspected += items.len() as u64;
            let monkey = monkeys[idx].clone();
            for mut item in items {
                match monkey.operation {
                    Op::Add(v) => item += v,
                    Op::Multiply(v) => item *= v,
                    Op::Square => item *= item,
                }
                item /= 3;
                if item % monkey.test == 0 {
                    monkeys[monkey.true_idx].items.push(item);
                } else {
                    monkeys[monkey.false_idx].items.push(item);
                }
            }
        }
    }

    monkeys
        .iter()
        .map(|x| x.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn part2<'a>(mut monkeys: Vec<Monkey>) -> u64 {
    let common_multiple: u64 = monkeys.iter().map(|monkey| monkey.test).product();

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[idx].items.drain(..).collect();
            monkeys[idx].inspected += items.len() as u64;
            let monkey = monkeys[idx].clone();
            for mut item in items {
                match monkey.operation {
                    Op::Add(v) => item += v,
                    Op::Multiply(v) => item *= v,
                    Op::Square => item *= item,
                }
                item %= common_multiple;
                if item % monkey.test == 0 {
                    monkeys[monkey.true_idx].items.push(item);
                } else {
                    monkeys[monkey.false_idx].items.push(item);
                }
            }
        }
    }

    monkeys
        .iter()
        .map(|x| x.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn parse<'a>(data: &'a str) -> Vec<Monkey> {
    let (_rest, monkeys) = parse_monkeys(data).unwrap();

    monkeys
}

fn parse_monkeys(s: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(many1(newline), monkey)(s)
}

fn monkey(s: &str) -> IResult<&str, Monkey> {
    let (rest, _) = tuple((tag("Monkey "), digit1, tag(":")))(s)?;
    let (rest, _) = many1(newline)(rest)?;
    let (rest, items) = preceded(
        tuple((multispace1, tag("Starting items: "))),
        separated_list1(
            tag(", "),
            map_res(digit1, |s: &str| u64::from_str_radix(s, 10)),
        ),
    )(rest)?;
    let (rest, _) = many1(newline)(rest)?;
    let (rest, operation) = preceded(
        tuple((multispace1, tag("Operation: new = old "))),
        alt((add, multiply, square)),
    )(rest)?;
    let (rest, _) = many1(newline)(rest)?;
    let (rest, test) = preceded(
        tuple((multispace1, tag("Test: divisible by "))),
        map_res(digit1, |s: &str| u64::from_str_radix(s, 10)),
    )(rest)?;
    let (rest, _) = many1(newline)(rest)?;
    let (rest, true_idx) = preceded(
        tuple((multispace1, tag("If true: throw to monkey "))),
        parser::from_dig,
    )(rest)?;
    let (rest, _) = many1(newline)(rest)?;
    let (rest, false_idx) = preceded(
        tuple((multispace1, tag("If false: throw to monkey "))),
        parser::from_dig,
    )(rest)?;

    let monkey = Monkey {
        items,
        operation,
        test,
        true_idx: true_idx.try_into().unwrap(),
        false_idx: false_idx.try_into().unwrap(),
        inspected: 0,
    };
    Ok((rest, monkey))
}

fn add(s: &str) -> IResult<&str, Op> {
    map(
        preceded(
            tag("+ "),
            map_res(digit1, |s: &str| u64::from_str_radix(s, 10)),
        ),
        |i| Op::Add(i),
    )(s)
}

fn multiply(s: &str) -> IResult<&str, Op> {
    map(
        preceded(
            tag("* "),
            map_res(digit1, |s: &str| u64::from_str_radix(s, 10)),
        ),
        |i| Op::Multiply(i),
    )(s)
}

fn square(s: &str) -> IResult<&str, Op> {
    map(tag("* old"), |_| Op::Square)(s)
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<u64>,
    operation: Op,
    test: u64,
    true_idx: usize,
    false_idx: usize,
    inspected: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add(u64),
    Multiply(u64),
    Square,
}

#[cfg(test)]
mod tests {
    use super::*;
}
