use std::fmt;
use std::fmt::Debug;

use nom::{
    branch::alt,
    bytes::complete::tag,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Eq, PartialEq, Clone)]
struct Pair {
    l: Node,
    r: Node,
}

impl Pair {
    pub fn new(l: Node, r: Node) -> Self {
        Self { l, r }
    }
}

impl Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?},{:?}]", self.l, self.r)
    }
}

#[derive(Eq, PartialEq, Clone)]
struct Value {
    value: i32,
}

impl Value {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Node {
    Pair(Box<Pair>),
    Value(Value),
}

impl Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Pair(pair) => write!(f, "{:?}", pair),
            Node::Value(value) => write!(f, "{:?}", value),
        }
    }
}

impl Node {
    pub fn pair(l: Self, r: Self) -> Self {
        Node::Pair(Box::new(Pair::new(l, r)))
    }

    pub fn value(value: i32) -> Self {
        Node::Value(Value::new(value))
    }

    fn add_left(&mut self, i: i32) {
        match self {
            Node::Value(old) => *self = Self::value(old.value + i),
            Node::Pair(pair) => pair.l.add_left(i),
        }
    }

    fn add_right(&mut self, i: i32) {
        match self {
            Node::Value(old) => *self = Self::value(old.value + i),
            Node::Pair(pair) => pair.r.add_right(i),
        }
    }

    fn explode(&mut self, depth: usize) -> (Option<i32>, Option<i32>, bool) {
        match self {
            Node::Value(_value) => (None, None, false),
            Node::Pair(pair) => {
                if depth == 4 {
                    let left = match &pair.l {
                        Node::Value(value) => Some(value.value),
                        _ => None,
                    };
                    let right = match &pair.r {
                        Node::Value(value) => Some(value.value),
                        _ => None,
                    };
                    *self = Node::value(0);
                    return (left, right, true);
                }

                let (ll, lr, lexploded) = pair.l.explode(depth + 1);

                if let Some(i) = lr {
                    pair.r.add_left(i);
                }
                if lexploded {
                    return (ll, None, true);
                }

                let (rl, rr, rexploded) = pair.r.explode(depth + 1);

                if let Some(i) = rl {
                    pair.l.add_right(i)
                }
                (None, rr, rexploded)
            }
        }
    }

    pub fn split(&mut self) -> bool {
        match self {
            Self::Value(value) if value.value < 10 => false,
            Self::Value(value) => {
                let left = value.value / 2;
                let right = value.value - left;
                *self = Node::pair(Node::value(left), Node::value(right));
                true
            }
            Self::Pair(pair) => pair.l.split() || pair.r.split(),
        }
    }

    fn reduce(&mut self) {
        loop {
            let (_, _, reduced) = self.explode(0);
            if reduced || self.split() {
                continue;
            }
            break;
        }
    }

    fn magnitude(&self) -> i32 {
        match self {
            Self::Value(value) => value.value,
            Self::Pair(pair) => 3 * pair.l.magnitude() + 2 * pair.r.magnitude(),
        }
    }
}

impl std::ops::Add for Node {
    type Output = Node;
    fn add(self, rhs: Self) -> Self::Output {
        let mut n = Node::pair(self, rhs);
        n.reduce();
        n
    }
}

fn snum(s: &str) -> IResult<&str, Node> {
    let (rest, num) = parser::from_dig(s)?;
    Ok((rest, Node::value(num)))
}

fn pair(s: &str) -> IResult<&str, Node> {
    let (rest, (a, b)) = delimited(
        tag("["),
        separated_pair(alt((snum, pair)), tag(","), alt((snum, pair))),
        tag("]"),
    )(s)?;

    Ok((rest, Node::pair(a, b)))
}

fn parse(data: String) -> Node {
    let (_rest, snums) = pair(&data).unwrap();
    snums
}

pub fn part1(data: String) -> i32 {
    let added = data
        .lines()
        .map(|line| parse(line.to_string()))
        .reduce(|acc, line| acc + line);

    added.unwrap().magnitude()
}

pub fn part2(data: String) -> i32 {
    let snums: Vec<Node> = data.lines().map(|line| parse(line.to_string())).collect();

    let mut max: i32 = 0;

    for i in 0..snums.len() {
        for j in 0..snums.len() {
            if i == j {
                continue;
            }

            let a = &snums[i];
            let b = &snums[j];

            max = max
                .max((a.clone() + b.clone()).magnitude())
                .max((b.clone() + a.clone()).magnitude());
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]".to_string();

        assert_eq!(
            parse(data),
            Node::pair(
                Node::pair(
                    Node::pair(
                        Node::pair(Node::value(1), Node::value(2)),
                        Node::pair(Node::value(3), Node::value(4)),
                    ),
                    Node::pair(
                        Node::pair(Node::value(5), Node::value(6)),
                        Node::pair(Node::value(7), Node::value(8)),
                    ),
                ),
                Node::value(9),
            )
        )
    }

    #[test]
    fn test_reduce() {
        let a = "[[[[4,3],4],4],[7,[[8,4],9]]]".to_string();
        let b = "[1,1]".to_string();

        let snumsa = parse(a);
        let snumsb = parse(b);

        assert_eq!(
            snumsa + snumsb,
            parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string())
        )
    }

    #[test]
    fn test_magnitude_1() {
        let data = "[[1,2],[[3,4],5]]".to_string();

        assert_eq!(parse(data).magnitude(), 143)
    }

    #[test]
    fn test_magnitude_2() {
        let data = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string();

        assert_eq!(parse(data).magnitude(), 1384)
    }

    #[test]
    fn test_magnitude_3() {
        let data = "[[[[1,1],[2,2]],[3,3]],[4,4]]".to_string();

        assert_eq!(parse(data).magnitude(), 445)
    }

    #[test]
    fn test_magnitude_4() {
        let data = "[[[[3,0],[5,3]],[4,4]],[5,5]]".to_string();

        assert_eq!(parse(data).magnitude(), 791)
    }

    #[test]
    fn test_magnitude_5() {
        let data = "[[[[5,0],[7,4]],[5,5]],[6,6]]".to_string();

        assert_eq!(parse(data).magnitude(), 1137)
    }

    #[test]
    fn test_magnitude_6() {
        let data = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string();

        assert_eq!(parse(data).magnitude(), 3488)
    }
}
