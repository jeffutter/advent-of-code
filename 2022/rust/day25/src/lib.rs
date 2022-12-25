pub fn part1(_state: State) -> &'static str {
    "foo"
}

pub fn part2(_state: State) -> u16 {
    1
}

pub fn parse<'a>(_data: &'a str) -> State {
    State::new()
}

#[derive(Clone)]
pub struct State {}

impl State {
    fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn test1() {
        let parsed = parse(INPUT);
        let res = part1(parsed);
        assert_eq!("2=-1=0", res)
    }

    #[test]
    fn test2() {
        let parsed = parse(INPUT);
        let res = part2(parsed);
        assert_eq!(0, res)
    }
}
