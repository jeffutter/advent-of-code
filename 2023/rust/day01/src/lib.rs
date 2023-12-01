use itertools::Itertools;
use regex::Regex;

pub fn parse<'a>(data: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    data.lines()
}

pub fn part1<'a>(input: impl Iterator<Item = &'a str>) -> i32 {
    input
        .map(|l| {
            let ns = l
                .chars()
                .filter_map(|c| c.to_string().parse::<i32>().ok())
                .collect_vec();

            (ns.first().unwrap() * 10) + ns.last().unwrap()
        })
        .sum()
}

pub fn part2<'a>(input: impl Iterator<Item = &'a str>) -> i32 {
    let re: Regex =
        Regex::new(r"(1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let reverse_re: Regex =
        Regex::new(r"(1|2|3|4|5|6|7|8|9|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

    input
        .map(|l| {
            let first = re.find(l).unwrap().as_str();
            let last = reverse_re
                .find(&l.chars().rev().collect::<String>())
                .unwrap()
                .as_str()
                .chars()
                .rev()
                .collect::<String>();

            (match_string_num(first) * 10) + match_string_num(&last)
        })
        .sum()
}

fn match_string_num(str: &str) -> i32 {
    match str {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!(),
    }
}
