use std::iter::zip;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};
use parser::FromDig;

pub fn parse<'a>(data: &'a str) -> IResult<&str, Vec<(usize, usize)>> {
    let (rest, times) = preceded(
        tag("Time:"),
        delimited(
            multispace0,
            separated_list1(multispace1, <usize as FromDig>::from_dig),
            multispace0,
        ),
    )(data)?;

    let (rest, distances) = preceded(
        tag("Distance:"),
        delimited(
            multispace0,
            separated_list1(multispace1, <usize as FromDig>::from_dig),
            multispace0,
        ),
    )(rest)?;

    assert_eq!("", rest.trim());

    Ok((rest, zip(times, distances).collect()))
}

pub fn part1<'a>(input: IResult<&str, Vec<(usize, usize)>>) -> usize {
    let (_, data) = input.unwrap();
    let mut res = 1;

    for (time, distance) in data {
        let mut winners = 0;

        for hold in 0..time {
            let rem = time - hold;
            let d = hold * rem;
            if d > distance {
                winners += 1;
            }
        }

        res *= winners;
    }

    res
}

pub fn part2<'a>(input: IResult<&str, Vec<(usize, usize)>>) -> usize {
    let (_, data) = input.unwrap();
    let time: String = data.iter().map(|(t, _d)| t.to_string()).collect();
    let distance: String = data.iter().map(|(_t, d)| d.to_string()).collect();
    let time: usize = time.parse().unwrap();
    let distance: usize = distance.parse().unwrap();

    let mut winners = 0;

    for hold in 0..time {
        let rem = time - hold;
        let d = hold * rem;
        if d > distance {
            winners += 1;
        }
    }

    winners
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_sample1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 288);
    }

    #[test]
    fn test_sample2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 71503);
    }

    generate_test! { 2023, 6, 1, 138915}
    generate_test! { 2023, 6, 2, 27340847}
}
