use nom::{
    bytes::complete::tag,
    character::{complete::alpha1, complete::line_ending},
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

fn pattern(s: &str) -> IResult<&str, &str> {
    alpha1(s)
}

fn parse(s: &str) -> IResult<&str, Vec<(Vec<&str>, &str, Vec<&str>)>> {
    many1(terminated(
        tuple((
            separated_list1(tag(" "), pattern),
            tag(" | "),
            separated_list1(tag(" "), pattern),
        )),
        line_ending,
    ))(&s)
}

pub fn part1(data: String) -> i32 {
    let (_rest, lines) = parse(&data).unwrap();

    lines
        .iter()
        .map(|(_, _, output)| {
            output
                .iter()
                .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
                .count() as i32
        })
        .sum()
}

fn common_with(pattern: &str, target: &str) -> i32 {
    target
        .chars()
        .fold(0, |acc, c| if pattern.contains(c) { acc + 1 } else { acc })
}

pub fn part2(data: String) -> i32 {
    let (_rest, lines) = parse(&data).unwrap();

    lines
        .iter()
        .map(|(mixed_wires, _, output)| {
            let one = mixed_wires
                .iter()
                .find(|pattern| pattern.len() == 2)
                .unwrap();
            let four = mixed_wires
                .iter()
                .find(|pattern| pattern.len() == 4)
                .unwrap();

            output
                .iter()
                .rev()
                .map(|pattern| {
                    match (
                        pattern.len(),
                        common_with(pattern, one),
                        common_with(pattern, four),
                    ) {
                        (2, _, _) => 1,
                        (5, _, 2) => 2,
                        (5, 2, _) => 3,
                        (4, _, _) => 4,
                        (5, 1, _) => 5,
                        (6, 1, _) => 6,
                        (3, _, _) => 7,
                        (7, _, _) => 8,
                        (6, _, 4) => 9,
                        (6, _, 3) => 0,
                        (_, _, _) => panic!(),
                    }
                })
                .enumerate()
                .map(|(i, n)| n * (10i32.pow(i as u32)))
                .sum::<i32>()
        })
        .sum()
}
