use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use parser::FromDig;

#[derive(Debug)]
pub struct Card {
    winners: HashSet<usize>,
    nums: HashSet<usize>,
}

fn parse_nums(s: &str) -> IResult<&str, HashSet<usize>> {
    preceded(
        multispace0,
        map(
            separated_list1(multispace1, <usize as FromDig>::from_dig),
            |nums| nums.into_iter().collect::<HashSet<_>>(),
        ),
    )(s)
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let (rest, _id) = preceded(
        terminated(tag("Card"), multispace1),
        terminated(<usize as FromDig>::from_dig, tag(": ")),
    )(s)?;
    let (rest, (winners, _, nums)) = tuple((parse_nums, tag(" | "), parse_nums))(rest)?;

    Ok((rest, Card { winners, nums }))
}

pub fn parse<'a>(data: &'a str) -> impl Iterator<Item = Card> {
    let (_rest, cards) = separated_list1(newline, parse_card)(data).unwrap();
    cards.into_iter()
}

pub fn part1<'a>(input: impl Iterator<Item = Card>) -> usize {
    input
        .map(|card| {
            let intersection = card.nums.intersection(&card.winners);
            let count = intersection.count();
            match count {
                0 => 0,
                count => 2_usize.pow((count - 1) as u32),
            }
        })
        .sum()
}

pub fn part2<'a>(input: impl Iterator<Item = Card>) -> usize {
    let (_, cards) = input.fold(
        (Vec::new(), 0),
        |(mut bonuses, cards): (Vec<usize>, usize), card| {
            let intersection = card.nums.intersection(&card.winners);
            let count = intersection.count();
            let mut num_card = 1;
            bonuses.retain_mut(|bonus| {
                num_card += 1;
                *bonus -= 1;
                *bonus != 0
            });
            if count > 0 {
                for _ in 0..num_card {
                    bonuses.push(count);
                }
            }
            (bonuses, cards + num_card)
        },
    );
    cards
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58 5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_sample1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 13);
    }

    #[test]
    fn test_sample2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 30);
    }

    generate_test! { 2023, 4, 1, 15205}
    generate_test! { 2023, 4, 2, 6189740}
}
