use std::collections::HashMap;

use itertools::Itertools;
use parser::point3;
use util::Point3;
use winnow::{
    Parser,
    ascii::line_ending,
    combinator::{repeat, separated},
};

type InputType = JunctionBoxes;
type OutType = usize;
type JunctionBox = Point3<usize>;

pub struct JunctionBoxes {
    circuits: HashMap<usize, Vec<JunctionBox>>,
}

impl JunctionBoxes {
    fn connect(&mut self, j1: &JunctionBox, j2: &JunctionBox) {
        let circuit1 = *self
            .circuits
            .iter()
            .find_map(|(idx, c)| {
                if c.contains(j1) {
                    return Some(idx);
                }
                None
            })
            .unwrap();

        let circuit2 = *self
            .circuits
            .iter()
            .find_map(|(idx, c)| {
                if c.contains(j2) {
                    return Some(idx);
                }
                None
            })
            .unwrap();

        if circuit1 == circuit2 {
            return;
        }

        if self.circuits.get(&circuit1).iter().len() >= self.circuits.get(&circuit2).iter().len() {
            let mut source = self.circuits.remove(&circuit2).unwrap();
            let source_ref = source.as_mut();
            let dest = self.circuits.get_mut(&circuit1).unwrap();
            dest.append(source_ref);
        } else {
            let mut source = self.circuits.remove(&circuit1).unwrap();
            let source_ref = source.as_mut();
            let dest = self.circuits.get_mut(&circuit2).unwrap();
            dest.append(source_ref);
        }
    }

    fn all_pairs_ranked(&self) -> impl Iterator<Item = (Point3<usize>, Point3<usize>)> {
        self.circuits
            .iter()
            .flat_map(|(_, p)| p.iter().cloned())
            .sorted()
            .tuple_combinations()
            .filter(|(a, b)| a != b)
            .sorted_by_key(|(a, b)| a.straight_line_distance(b).unwrap() as u32)
    }
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let (points, _): (Vec<Point3<usize>>, Vec<_>) = (
        separated(1.., point3(","), line_ending),
        repeat(0.., line_ending),
    )
        .parse(data)
        .unwrap();

    JunctionBoxes {
        circuits: points
            .into_iter()
            .enumerate()
            .map(|(idx, point)| (idx, vec![point]))
            .collect(),
    }
}

pub fn part1(input: InputType) -> OutType {
    _part1(input, 1000)
}

pub fn _part1(mut input: InputType, n: usize) -> OutType {
    let pairs: Vec<_> = input.all_pairs_ranked().take(n).collect();
    for (p1, p2) in pairs {
        input.connect(&p1, &p2);
    }

    input
        .circuits
        .values()
        .map(|p| p.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[allow(unused_variables)]
pub fn part2(mut input: InputType) -> OutType {
    let pairs: Vec<_> = input.all_pairs_ranked().collect();
    for (p1, p2) in pairs {
        input.connect(&p1, &p2);

        if input.circuits.len() == 1 {
            return p1.x * p2.x;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn example_1() {
        let data = parse(TEST_INPUT);
        assert_eq!(_part1(data, 10), 40)
    }

    #[test]
    fn example_2() {
        let data = parse(TEST_INPUT);
        assert_eq!(part2(data), 25272)
    }

    #[test]
    fn test_1() {
        let input = util::read_input(2025, 8);
        let data = parse(&input);
        assert_eq!(part1(data), 175500)
    }

    #[test]
    fn test_2() {
        let input = util::read_input(2025, 8);
        let data = parse(&input);
        assert_eq!(part2(data), 6934702555)
    }
}
