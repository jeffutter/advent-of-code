use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use util::Pos;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Copy, Debug)]
pub enum MirrorsAndSplitters {
    FS,
    BS,
    H,
    V,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Beam {
    direction: Direction,
    pos: Pos<usize>,
}

impl Beam {
    fn new(pos: Pos<usize>, direction: Direction) -> Self {
        Self { pos, direction }
    }

    fn mv(&mut self, min: &Pos<usize>, max: &Pos<usize>) -> Option<&mut Beam> {
        match self.direction {
            Direction::N => {
                if self.pos.y == min.y {
                    return None;
                }

                self.pos.y -= 1;
                Some(self)
            }
            Direction::E => {
                if self.pos.x == max.x {
                    return None;
                }

                self.pos.x += 1;
                Some(self)
            }
            Direction::S => {
                if self.pos.y == max.y {
                    return None;
                }

                self.pos.y += 1;
                Some(self)
            }
            Direction::W => {
                if self.pos.x == min.x {
                    return None;
                }

                self.pos.x -= 1;
                Some(self)
            }
        }
    }

    fn apply_tile(&self, miror_or_splitter: Option<&MirrorsAndSplitters>) -> Vec<Beam> {
        match (miror_or_splitter, self.direction) {
            (Some(MirrorsAndSplitters::V), Direction::N) => vec![self.clone()],
            (Some(MirrorsAndSplitters::V), Direction::S) => vec![self.clone()],
            (Some(MirrorsAndSplitters::H), Direction::E) => vec![self.clone()],
            (Some(MirrorsAndSplitters::H), Direction::W) => vec![self.clone()],
            (Some(MirrorsAndSplitters::V), Direction::E) => {
                vec![
                    Beam::new(self.pos.clone(), Direction::N),
                    Beam::new(self.pos.clone(), Direction::S),
                ]
            }
            (Some(MirrorsAndSplitters::V), Direction::W) => {
                vec![
                    Beam::new(self.pos.clone(), Direction::N),
                    Beam::new(self.pos.clone(), Direction::S),
                ]
            }
            (Some(MirrorsAndSplitters::H), Direction::N) => {
                vec![
                    Beam::new(self.pos.clone(), Direction::E),
                    Beam::new(self.pos.clone(), Direction::W),
                ]
            }
            (Some(MirrorsAndSplitters::H), Direction::S) => {
                vec![
                    Beam::new(self.pos.clone(), Direction::E),
                    Beam::new(self.pos.clone(), Direction::W),
                ]
            }
            (Some(MirrorsAndSplitters::FS), Direction::N) => {
                vec![Beam::new(self.pos.clone(), Direction::E)]
            }
            (Some(MirrorsAndSplitters::FS), Direction::S) => {
                vec![Beam::new(self.pos.clone(), Direction::W)]
            }
            (Some(MirrorsAndSplitters::FS), Direction::E) => {
                vec![Beam::new(self.pos.clone(), Direction::N)]
            }
            (Some(MirrorsAndSplitters::FS), Direction::W) => {
                vec![Beam::new(self.pos.clone(), Direction::S)]
            }
            (Some(MirrorsAndSplitters::BS), Direction::N) => {
                vec![Beam::new(self.pos.clone(), Direction::W)]
            }
            (Some(MirrorsAndSplitters::BS), Direction::S) => {
                vec![Beam::new(self.pos.clone(), Direction::E)]
            }
            (Some(MirrorsAndSplitters::BS), Direction::E) => {
                vec![Beam::new(self.pos.clone(), Direction::S)]
            }
            (Some(MirrorsAndSplitters::BS), Direction::W) => {
                vec![Beam::new(self.pos.clone(), Direction::N)]
            }
            (None, _) => {
                vec![self.clone()]
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Contraption {
    mirrors_and_splitters: HashMap<Pos<usize>, MirrorsAndSplitters>,
    beams: VecDeque<Beam>,
    max: Pos<usize>,
    visited: HashSet<Pos<usize>>,
    visited_with_direction: HashSet<Beam>,
}

impl Contraption {
    fn fill(&mut self) {
        while let Some(beam) = self.beams.pop_front() {
            self.visited.insert(beam.pos.clone());
            if !self.visited_with_direction.insert(beam.clone()) {
                continue;
            }
            let tile = self.mirrors_and_splitters.get(&beam.pos);

            for mut beam in beam.apply_tile(tile) {
                if let Some(beam) = beam.mv(&Pos::new_unsigned(0, 0), &self.max) {
                    self.beams.push_back(beam.clone());
                }
            }
        }
    }
}

pub fn parse<'a>(data: &'a str) -> Contraption {
    let mut h = HashMap::new();
    let mut max = Pos::new_unsigned(0, 0);

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Pos::new_unsigned(x, y);
            max = Pos::new_unsigned(max.x.max(x), max.y.max(y));
            if c == '.' {
                continue;
            }
            match c {
                '/' => h.insert(p, MirrorsAndSplitters::FS),
                '\\' => h.insert(p, MirrorsAndSplitters::BS),
                '-' => h.insert(p, MirrorsAndSplitters::H),
                '|' => h.insert(p, MirrorsAndSplitters::V),
                x => unimplemented!("'{}'", x),
            };
        }
    }

    Contraption {
        mirrors_and_splitters: h,
        beams: VecDeque::new(),
        visited: HashSet::new(),
        visited_with_direction: HashSet::new(),
        max,
    }
}

pub fn part1<'a>(mut contraption: Contraption) -> usize {
    contraption
        .beams
        .push_front(Beam::new(Pos::new_unsigned(0, 0), Direction::E));
    contraption.fill();
    contraption.visited.len()
}

pub fn part2<'a>(contraption: Contraption) -> usize {
    let scenarios = (0..=contraption.max.x)
        .cartesian_product(0..=contraption.max.y)
        .filter(|(x, y)| *x == 0 || *y == 0 || *x == contraption.max.x || *y == contraption.max.y)
        .map(|(x, y)| Pos::new_unsigned(x, y))
        .flat_map(|pos| {
            if pos.x == 0 && pos.y == 0 {
                return vec![
                    Beam::new(pos.clone(), Direction::E),
                    Beam::new(pos, Direction::S),
                ];
            }
            if pos.x == contraption.max.x && pos.y == 0 {
                return vec![
                    Beam::new(pos.clone(), Direction::W),
                    Beam::new(pos, Direction::S),
                ];
            }

            if pos.x == 0 && pos.y == contraption.max.y {
                return vec![
                    Beam::new(pos.clone(), Direction::E),
                    Beam::new(pos, Direction::N),
                ];
            }
            if pos.x == contraption.max.x && pos.y == contraption.max.y {
                return vec![
                    Beam::new(pos.clone(), Direction::W),
                    Beam::new(pos, Direction::N),
                ];
            }

            if pos.x == 0 {
                return vec![Beam::new(pos, Direction::E)];
            }

            if pos.x == contraption.max.x {
                return vec![Beam::new(pos, Direction::W)];
            }

            if pos.y == 0 {
                return vec![Beam::new(pos, Direction::S)];
            }

            if pos.y == contraption.max.y {
                return vec![Beam::new(pos, Direction::N)];
            }

            unreachable!()
        });

    scenarios
        .map(|start| {
            let mut contraption = contraption.clone();
            contraption.beams.push_front(start);
            contraption.fill();
            contraption.visited.len()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 46);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 51);
    }

    generate_test! { 2023, 16, 1, 6855}
    generate_test! { 2023, 16, 2, 0}
}
