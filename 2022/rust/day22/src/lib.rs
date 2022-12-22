pub fn part1(state: Vec<i64>) -> i64 {
    1
}

pub fn part2(state: Vec<i64>) -> i64 {
    1
}

pub fn parse<'a>(data: &'a str) -> Vec<i64> {
    vec![0]
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn test1() {
        let parsed = parse(INPUT);
        let res = part1(parsed);
        assert_eq!(0, res)
    }
}
