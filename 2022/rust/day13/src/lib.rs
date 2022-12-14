use std::{cmp::Ordering, fmt};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, newline},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

pub fn part1<'a>(pairs: Vec<Pair>) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            let mut a = vec![(pair.0).0.clone(), (pair.0).1.clone()];
            let b = vec![(pair.0).0.clone(), (pair.0).1.clone()];
            a.sort();
            if a == b {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2<'a>(pairs: Vec<Pair>) -> usize {
    let divider1 = Packet::List(vec![Packet::List(vec![Packet::I(6)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::I(2)])]);

    pairs
        .into_iter()
        .chain(vec![Pair((divider1.clone(), divider2.clone()))].into_iter())
        .flat_map(|pair| vec![(pair.0).0, (pair.0).1])
        .sorted()
        .enumerate()
        .filter_map(|(i, pair)| {
            if pair == divider1 {
                return Some(i + 1);
            }
            if pair == divider2 {
                return Some(i + 1);
            }
            None
        })
        .product()
}

pub fn parse<'a>(data: &'a str) -> Vec<Pair> {
    let (_rest, pairs) = parse_pairs(data).unwrap();
    assert_eq!(_rest.trim(), "");
    pairs
}

fn parse_pairs(s: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(many1(line_ending), pair)(s)
}

fn pair(s: &str) -> IResult<&str, Pair> {
    let (rest, (a, _, b)) = tuple((packet, newline, packet))(s)?;
    Ok((rest, Pair((a, b))))
}

fn packet(s: &str) -> IResult<&str, Packet> {
    let (rest, list) = delimited(
        tag("["),
        separated_list0(tag(","), alt((i, packet))),
        tag("]"),
    )(s)?;
    Ok((rest, Packet::List(list)))
}

fn i(s: &str) -> IResult<&str, Packet> {
    let (rest, i) = parser::from_dig(s)?;
    Ok((rest, Packet::I(i)))
}

pub struct Pair((Packet, Packet));

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pair: \n{:?}\n{:?}\n", (self.0).0, (self.0).1)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Packet {
    I(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::I(a), Packet::I(b)) => a.cmp(b),
            (Packet::I(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            (Packet::List(_), Packet::I(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
        }
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I(i) => write!(f, "{}", i),
            Self::List(l) => f.debug_list().entries(l).finish(),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp_integers() {
        let a = Packet::I(1);
        let b = Packet::I(3);
        assert_eq!(Ordering::Less, a.cmp(&b))
    }

    #[test]
    fn list_eq() {
        let a = Packet::List(vec![]);
        let b = Packet::List(vec![]);
        assert_eq!(Ordering::Equal, a.cmp(&b));

        let c = Packet::List(vec![Packet::I(3)]);
        let d = Packet::List(vec![Packet::I(3)]);
        assert_eq!(Ordering::Equal, c.cmp(&d))
    }

    #[test]
    fn list_less() {
        let a = Packet::List(vec![]);
        let b = Packet::List(vec![Packet::I(1)]);
        assert_eq!(Ordering::Less, a.cmp(&b));

        let c = Packet::List(vec![Packet::I(1)]);
        let d = Packet::List(vec![Packet::I(2)]);
        assert_eq!(Ordering::Less, c.cmp(&d));
    }

    #[test]
    fn list_greater() {
        let a = Packet::List(vec![Packet::I(1)]);
        let b = Packet::List(vec![]);
        assert_eq!(Ordering::Greater, a.cmp(&b));

        let c = Packet::List(vec![Packet::I(2)]);
        let d = Packet::List(vec![Packet::I(1)]);
        assert_eq!(Ordering::Greater, c.cmp(&d));
    }

    #[test]
    fn mixed_eq() {
        let a = Packet::List(vec![Packet::I(1)]);
        let b = Packet::I(1);
        assert_eq!(Ordering::Equal, a.cmp(&b));

        let c = Packet::I(1);
        let d = Packet::List(vec![Packet::I(1)]);
        assert_eq!(Ordering::Equal, c.cmp(&d))
    }

    #[test]
    fn mixed_less() {
        let a = Packet::List(vec![]);
        let b = Packet::I(1);
        assert_eq!(Ordering::Less, a.cmp(&b));

        let c = Packet::List(vec![Packet::I(1)]);
        let d = Packet::I(2);
        assert_eq!(Ordering::Less, c.cmp(&d));

        let e = Packet::I(1);
        let f = Packet::List(vec![Packet::I(1), Packet::I(1)]);
        assert_eq!(Ordering::Less, e.cmp(&f));
    }

    #[test]
    fn mixed_greater() {
        let a = Packet::I(1);
        let b = Packet::List(vec![]);
        assert_eq!(Ordering::Greater, a.cmp(&b));

        let c = Packet::I(2);
        let d = Packet::List(vec![Packet::I(1)]);
        assert_eq!(Ordering::Greater, c.cmp(&d));

        let e = Packet::I(2);
        let f = Packet::List(vec![Packet::I(1), Packet::I(1)]);
        assert_eq!(Ordering::Greater, e.cmp(&f));
    }
}
