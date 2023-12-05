use std::{collections::HashMap, ops::Range};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace1, newline},
    combinator::recognize,
    multi::{many0_count, many1, separated_list0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    IResult, Parser,
};
use parser::FromDig;

#[derive(Debug)]
pub struct SeedMap<'a> {
    seeds: Vec<i64>,
    maps: HashMap<(&'a str, &'a str), OffsetMap<'a>>,
}

#[derive(Debug)]
pub struct OffsetMap<'a> {
    from: &'a str,
    to: &'a str,
    ranges: Vec<OffsetRange>,
}

impl<'a> OffsetMap<'a> {
    fn map_seed(&self, seed: &i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|range| {
                let mapped = range.map_seed(seed);
                if mapped != *seed {
                    return Some(mapped);
                }
                None
            })
            .unwrap_or(*seed)
    }
}

#[derive(Debug)]
pub struct OffsetRange {
    range: Range<i64>,
    offset: i64,
}

impl OffsetRange {
    fn map_seed(&self, seed: &i64) -> i64 {
        if self.range.contains(seed) {
            return seed + self.offset;
        }
        *seed
    }
}

fn parse_offset_range(s: &str) -> IResult<&str, OffsetRange> {
    let (rest, (dest, _, source, _, len)) = tuple((
        <i64 as FromDig>::from_dig,
        multispace1,
        <i64 as FromDig>::from_dig,
        multispace1,
        <i64 as FromDig>::from_dig,
    ))(s)?;

    Ok((
        rest,
        OffsetRange {
            range: source..(source + len),
            offset: dest - source,
        },
    ))
}

fn parse_offset_map(s: &str) -> IResult<&str, OffsetMap> {
    let (rest, ((from, _, to), ranges)) = tuple((
        terminated(
            tuple((alpha1, tag("-to-"), alpha1)),
            tuple((tag(" map:"), newline)),
        ),
        separated_list1(newline, parse_offset_range),
    ))(s)?;

    let offset_map = OffsetMap { from, to, ranges };

    Ok((rest, offset_map))
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("1"))),
        many0_count(alt((alphanumeric1, tag("-")))),
    ))
    .parse(input)
}

fn parse_seed_map(s: &str) -> IResult<&str, SeedMap> {
    let (rest, seeds) = preceded(
        tag("seeds: "),
        separated_list0(multispace1, <i64 as FromDig>::from_dig),
    )(s)?;
    let (rest, _) = newline(rest)?;
    let (rest, _) = newline(rest)?;
    let (rest, maps) = separated_list1(many1(newline), parse_offset_map)(rest)?;

    let seed_map = SeedMap {
        seeds,
        maps: maps
            .into_iter()
            .map(|map| ((map.from, map.to), map))
            .collect(),
    };

    Ok((rest, seed_map))
}

pub fn parse<'a>(data: &'a str) -> SeedMap {
    let (rest, seed_map) = parse_seed_map(data).unwrap();
    assert_eq!(rest.trim(), "");
    seed_map
}

const TRANSFORMS: [(&str, &str); 7] = [
    ("seed", "soil"),
    ("soil", "fertilizer"),
    ("fertilizer", "water"),
    ("water", "light"),
    ("light", "temperature"),
    ("temperature", "humidity"),
    ("humidity", "location"),
];

pub fn part1<'a>(input: SeedMap) -> i64 {
    input
        .seeds
        .iter()
        .map(|seed| {
            TRANSFORMS.iter().fold(*seed, |acc, transform| {
                input.maps.get(transform).unwrap().map_seed(&acc)
            })
        })
        .min()
        .unwrap()
}

pub fn part2<'a>(input: SeedMap) -> i64 {
    let mut ranges: Vec<Range<i64>> = Vec::new();

    for mut chunk in &input.seeds.iter().chunks(2) {
        let start = chunk.next().unwrap();
        let len = chunk.next().unwrap();
        ranges.push(*start..(*start + len));
    }

    ranges
        .into_iter()
        .flat_map(|range| range.into_iter())
        .unique()
        .map(|seed| {
            TRANSFORMS.iter().fold(seed, |acc, transform| {
                input.maps.get(transform).unwrap().map_seed(&acc)
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_sample1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 35);
    }

    #[test]
    fn test_sample2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 46);
    }

    generate_test! { 2023, 5, 1, 1181555926}
    generate_test! { 2023, 5, 2, 0}
}
