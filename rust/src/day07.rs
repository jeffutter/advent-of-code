use crate::parser::separated_digits;

pub fn part1(data: String) -> i32 {
    let (_rest, mut positions) = separated_digits(&data).unwrap();

    positions.sort();

    let target = positions[positions.len() / 2];

    let mut sum: i32 = 0;
    for pos in positions {
        let dist = (pos - target).abs();
        sum += dist;
    }

    sum
}

pub fn part2(data: String) -> i32 {
    let (_rest, positions) = separated_digits(&data).unwrap();

    let target: i32 = positions.iter().sum::<i32>() / positions.len() as i32;

    let mut sum: i32 = 0;
    for pos in positions {
        let dist = (pos - target).abs();
        sum += (1..=dist).fold(0, |a, b| a + b);
    }

    sum
}
