use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, line_ending},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

fn rule(s: &str) -> IResult<&str, ((char, char), char)> {
    let (rest, (c1, c2, _, c3)) = tuple((anychar, anychar, tag(" -> "), anychar))(s)?;

    Ok((rest, ((c1, c2), c3)))
}

fn parse<'a>(data: &'a str) -> (&'a str, HashMap<(char, char), char>) {
    let (_rest, (template, _, rules)) = tuple((
        terminated(alpha1, line_ending),
        line_ending,
        separated_list1(line_ending, rule),
    ))(data)
    .unwrap();

    let r: HashMap<(char, char), char> =
        rules
            .iter()
            .fold(HashMap::new(), |mut acc, ((c1, c2), c3)| {
                acc.insert((*c1, *c2), *c3);
                acc
            });

    (template, r)
}

fn count(template: &str, rules: &HashMap<(char, char), char>, steps: u8) -> HashMap<char, i64> {
    let mut pair_count: HashMap<(char, char), i64> = HashMap::new();
    let mut counter: HashMap<char, i64> = HashMap::new();

    for c in template.chars() {
        counter.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    let t1 = template.chars();
    let mut t2 = template.chars();
    t2.next();

    for pair in t1.zip(t2).collect::<Vec<(char, char)>>() {
        pair_count.entry(pair).and_modify(|v| *v += 1).or_insert(1);
    }

    for _ in 0..steps {
        for (chunk, c) in pair_count.clone() {
            let (c1, c2) = chunk;
            let i = rules.get(&chunk).unwrap();

            pair_count.entry(chunk).and_modify(|v| *v -= c).or_insert(0);
            pair_count
                .entry((c1, *i))
                .and_modify(|v| *v += c)
                .or_insert(c);
            pair_count
                .entry((*i, c2))
                .and_modify(|v| *v += c)
                .or_insert(c);
            counter.entry(*i).and_modify(|v| *v += c).or_insert(c);
        }
    }

    counter
}

pub fn part1(data: String) -> i64 {
    let (template, rules) = parse(&data);

    let counts = count(template, &rules, 10);

    let (_, max) = counts.iter().max_by_key(|(_c, v)| *v).unwrap();
    let (_, min) = counts.iter().min_by_key(|(_c, v)| *v).unwrap();

    max - min
}

pub fn part2(data: String) -> i64 {
    let (template, rules) = parse(&data);

    let counts = count(template, &rules, 40);

    let (_, max) = counts.iter().max_by_key(|(_c, v)| *v).unwrap();
    let (_, min) = counts.iter().min_by_key(|(_c, v)| *v).unwrap();

    max - min
}
