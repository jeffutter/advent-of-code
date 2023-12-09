use std::str::Lines;

pub fn parse<'a>(data: &'a str) -> Lines<'a> {
    data.lines()
}

pub fn part1<'a>(_input: impl Iterator<Item = &'a str>) -> i32 {
    1
}

pub fn part2<'a>(_input: impl Iterator<Item = &'a str>) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#""#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 0);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 0);
    }

    generate_test! { 2023, 12, 1, 0}
    generate_test! { 2023, 12, 2, 0}
}
