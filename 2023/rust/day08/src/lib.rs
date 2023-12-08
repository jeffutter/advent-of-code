use std::{collections::HashMap, str::FromStr};

use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

#[derive(Debug)]
enum Direction {
    L,
    R,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value.to_lowercase().to_string().as_str() {
            "l" => Direction::L,
            "r" => Direction::R,
            _ => unimplemented!("{}", value),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    left: NodeId,
    right: NodeId,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseNodeError;

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split(", ").collect_tuple().ok_or(ParseNodeError)?;
        let left = left.strip_prefix('(').ok_or(ParseNodeError)?;
        let right = right.strip_suffix(')').ok_or(ParseNodeError)?;

        Ok(Self {
            left: left.into(),
            right: right.into(),
        })
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct NodeId {
    id: String,
}

impl NodeId {
    fn is_start(&self) -> bool {
        self.id.ends_with('A')
    }

    fn is_end(&self) -> bool {
        self.id.ends_with('Z')
    }
}

impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        NodeId {
            id: value.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<NodeId, Node>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMapError;

impl FromStr for Map {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let directions = lines
            .next()
            .ok_or(ParseMapError)?
            .chars()
            .map(|c| c.into())
            .collect_vec();

        lines.next();

        let mut nodes = HashMap::new();

        for line in lines {
            let (id, node) = line.split(" = ").collect_tuple().ok_or(ParseMapError)?;
            let id = id.trim();
            let node: Node = node.parse().map_err(|_| ParseMapError)?;
            nodes.insert(id.into(), node);
        }

        Ok(Self { directions, nodes })
    }
}

pub fn parse<'a>(data: &'a str) -> Map {
    data.parse().unwrap()
}

pub fn part1<'a>(input: Map) -> usize {
    let end: NodeId = "ZZZ".into();
    count_steps(&input, &"AAA".into(), &|node_id| node_id == &end)
}

pub fn part2<'a>(input: Map) -> usize {
    lcm(&input
        .nodes
        .keys()
        .filter(|id| id.is_start())
        .map(|start| count_steps(&input, start, &|node_id| node_id.is_end()))
        .collect_vec())
}

fn count_steps(map: &Map, start: &NodeId, is_end: &dyn Fn(&NodeId) -> bool) -> usize {
    map.directions
        .iter()
        .cycle()
        .fold_while((0, start), |(count, next), mv| {
            if is_end(next) {
                return Done((count, next));
            }
            let node = map.nodes.get(&next).unwrap();
            let next = match mv {
                Direction::L => &node.left,
                Direction::R => &node.right,
            };
            Continue((count + 1, next))
        })
        .into_inner()
        .0
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    #[test]
    fn test_sample1_1() {
        let data = parse(&SAMPLE_INPUT1);
        assert_eq!(part1(data), 2);
    }

    const SAMPLE_INPUT2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    #[test]
    fn test_sample2_1() {
        let data = parse(&SAMPLE_INPUT2);
        assert_eq!(part1(data), 6);
    }

    const SAMPLE_INPUT3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn test_sample3_1() {
        let data = parse(&SAMPLE_INPUT3);
        assert_eq!(part2(data), 6);
    }

    generate_test! { 2023, 8, 1, 20513}
    generate_test! { 2023, 8, 2, 15995167053923}
}
