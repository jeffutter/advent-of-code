use itertools::Itertools;

pub fn part1<T: Iterator<Item = i64> + Clone>(ns: T) -> String {
    encode(ns.sum())
}

pub fn part2<T: Iterator<Item = i64>>(_ns: T) -> &'static str {
    ""
}

pub fn parse<'a>(data: &'a str) -> impl Iterator<Item = i64> + Clone + '_ {
    data.lines().into_iter().map(|line| decode(line))
}

fn decode(s: &str) -> i64 {
    let mut res = 0;
    for c in s.chars() {
        res *= 5;
        res += match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            x => unreachable!("{}", x),
        }
    }
    res
}

fn encode(i: i64) -> String {
    let mut res = String::new();
    let mut i = i.clone();
    while i > 0 {
        match i % 5 {
            0 => res.push('0'),
            1 => {
                res.push('1');
                i -= 1;
            }
            2 => {
                res.push('2');
                i -= 2;
            }
            3 => {
                res.push('=');
                i += 2;
            }
            4 => {
                res.push('-');
                i += 1;
            }
            x => unreachable!("{}", x),
        }
        i /= 5;
    }

    res.chars().rev().collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;

    #[test]
    fn test_decode() {
        assert_eq!(1, decode("1"));
        assert_eq!(2, decode("2"));
        assert_eq!(3, decode("1="));
        assert_eq!(4, decode("1-"));
        assert_eq!(5, decode("10"));
        assert_eq!(15, decode("1=0"));
        assert_eq!(314159265, decode("1121-1110-1=0"));
    }

    #[test]
    fn test_encode() {
        assert_eq!("1", encode(1));
        assert_eq!("2", encode(2));
        assert_eq!("1=", encode(3));
        assert_eq!("1-", encode(4));
        assert_eq!("10", encode(5));
        assert_eq!("1=0", encode(15));
        assert_eq!("1121-1110-1=0", encode(314159265));
    }

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
        assert_eq!("", res)
    }
}
