use std::collections::{HashMap, VecDeque};

use util::{BitMap, Direction, Pos};

type InputType = Manifold;
type OutType = usize;

enum Successors {
    Split([Pos<usize>; 2]),
    Straight(Pos<usize>),
}

#[derive(Debug)]
pub struct Manifold {
    start: Pos<usize>,
    beams: BitMap<usize>,
    splitters: BitMap<usize>,
}

impl Manifold {
    fn new(data: &str) -> Self {
        let height = data.lines().count();
        let width = data.lines().next().unwrap().chars().count();

        let mut start = Pos::new(0, 0);
        let beams = BitMap::new(width, height);
        let mut splitters = BitMap::new(width, height);

        for (y, line) in data.lines().rev().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    'S' => start = Pos::new(x, y),
                    '^' => splitters.insert(&Pos::new(x, y)),
                    '.' => (),
                    _ => unimplemented!(),
                }
            }
        }

        Self {
            start,
            beams,
            splitters,
        }
    }

    fn successors(&self, beam: &Pos<usize>) -> Option<Successors> {
        if let Some(next) = beam.translate(&Direction::N) {
            if !self.splitters.contains(&next) {
                return Some(Successors::Straight(next));
            }

            return Some(Successors::Split([
                next.translate(&Direction::E).unwrap(),
                next.translate(&Direction::W).unwrap(),
            ]));
        }

        None
    }
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    Manifold::new(data)
}

#[allow(unused_variables)]
pub fn part1(mut manifold: InputType) -> OutType {
    let mut split_count = 0;
    let mut last_beams = VecDeque::new();
    last_beams.push_front(manifold.start.clone());

    while let Some(beam) = last_beams.pop_front() {
        match manifold.successors(&beam) {
            Some(Successors::Straight(next)) => {
                manifold.beams.insert(&next);
                last_beams.push_front(next);
                continue;
            }
            Some(Successors::Split(successors)) => {
                let mut did_split = false;

                for successor in successors {
                    if manifold.beams.contains(&successor) {
                        continue;
                    }

                    did_split = true;
                    manifold.beams.insert(&successor);
                    last_beams.push_front(successor);
                }

                if did_split {
                    split_count += 1;
                }
            }
            None => (),
        }
    }

    split_count
}

#[allow(unused_variables)]
pub fn part2(manifold: InputType) -> OutType {
    count_paths(&manifold, &manifold.start.clone(), &mut HashMap::new())
}

fn count_paths(
    manifold: &InputType,
    beam: &Pos<usize>,
    cache: &mut HashMap<Pos<usize>, usize>,
) -> usize {
    if let Some(count) = cache.get(beam) {
        return *count;
    }

    let count = match manifold.successors(beam) {
        Some(Successors::Split(successors)) => successors
            .iter()
            .map(|next| count_paths(manifold, next, cache))
            .sum(),
        Some(Successors::Straight(next)) => count_paths(manifold, &next, cache),
        None => 1,
    };

    cache.insert(beam.clone(), count);

    count
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const TEST_INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    generate_test!(TEST_INPUT, 1, 21);
    generate_test!(TEST_INPUT, 2, 40);

    generate_test! { 2025, 7, 1, 1672}
    generate_test! { 2025, 7, 2, 231229866702355}
}
