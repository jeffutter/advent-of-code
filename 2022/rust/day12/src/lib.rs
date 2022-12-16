use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt,
};

use rayon::prelude::*;

pub fn part1<'a>(map: Map) -> u32 {
    shortest_path(&map, map.start, map.end).unwrap()
}

pub fn part2<'a>(map: Map) -> u32 {
    map.points
        .par_iter()
        .filter(|(_, val)| **val == 0)
        .filter_map(|(pos, _)| shortest_path(&map, *pos, map.end))
        .min()
        .unwrap()
}

pub fn parse<'a>(data: &'a str) -> Map {
    data.lines()
        .enumerate()
        .fold(Map::new(), |mut map, (y, line)| {
            map.max_y = map.max_y.max(y);
            line.chars().enumerate().fold(map, |mut map, (x, char)| {
                map.max_x = map.max_x.max(x);
                let point = Pos::new(x, y);

                match char {
                    'S' => {
                        map.start = point.clone();
                        map.curr = point.clone();
                        map.insert(point, 0);
                    }
                    'E' => {
                        map.end = point.clone();
                        map.insert(point, 25);
                    }
                    _ => {
                        let val = (char as u32) - 97;
                        map.insert(point, val);
                    }
                };
                map
            })
        })
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: u32,
    position: Pos,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(map: &Map, start: Pos, goal: Pos) -> Option<u32> {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut heap = BinaryHeap::new();

    visited.insert(start);
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        for neighbor in map.successors(&position) {
            if visited.insert(neighbor) {
                heap.push(State {
                    cost: cost + 1,
                    position: neighbor,
                });
            }
        }
    }

    None
}

pub struct Map {
    points: HashMap<Pos, u32>,
    max_x: usize,
    max_y: usize,
    start: Pos,
    end: Pos,
    curr: Pos,
}

impl Map {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
            max_x: 0,
            max_y: 0,
            start: Pos::new(0, 0),
            end: Pos::new(0, 0),
            curr: Pos::new(0, 0),
        }
    }

    pub fn insert(&mut self, pos: Pos, v: u32) -> &Self {
        self.points.insert(pos, v);
        self
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&u32> {
        self.points.get(&Pos::new(x, y))
    }

    pub fn get_pos(&self, pos: &Pos) -> Option<&u32> {
        self.points.get(pos)
    }

    fn successors(&self, pos: &Pos) -> Vec<Pos> {
        let &Pos { x, y } = pos;
        let curr_val = self.get_pos(pos).unwrap();

        vec![(0, 1), (1, 0), (0, -1i32), (-1i32, 0)]
            .iter()
            .filter_map(|(xx, yy)| {
                let x = x as i32;
                let y = y as i32;

                (x + xx)
                    .try_into()
                    .and_then(|x: usize| {
                        (y + yy).try_into().map(|y: usize| {
                            let dest = Pos::new(x, y);

                            match self.get_pos(&dest) {
                                Some(dest_v) if *dest_v <= *curr_val => Some(dest),
                                Some(dest_v) if *dest_v == curr_val + 1 => Some(dest),
                                Some(_) => None,
                                None => None,
                            }
                        })
                    })
                    .ok()
                    .flatten()
            })
            .collect()
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Start: ({},{})\n", self.start.x, self.start.y)?;
        write!(f, "End: ({},{})\n", self.end.x, self.end.y)?;
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let pos = Pos::new(x, y);
                let val = self.get_pos(&pos).unwrap();

                let v = match pos {
                    p if p == self.curr => "X".to_string(),
                    p if p == self.start => "S".to_string(),
                    p if p == self.end => "E".to_string(),
                    _ => (char::from_u32(val + 97)).unwrap().to_string(),
                };

                write!(f, "{}", v)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
        let parsed = parse(input);
        let res = part1(parsed);
        assert_eq!(31, res)
    }
}
