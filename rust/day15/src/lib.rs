use std::cmp::Ordering;
use std::collections::hash_map::Iter;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Debug;

use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

enum Direction {
    X,
    Y,
}

#[derive(Clone, Eq, PartialEq)]
struct Map(HashMap<Pos, u32>);

impl Map {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    fn iter(&self) -> Iter<Pos, u32> {
        self.0.iter()
    }

    fn insert(&mut self, k: Pos, v: u32) -> &Self {
        self.0.insert(k, v);
        self
    }

    fn get(&self, k: &Pos) -> Option<&u32> {
        self.0.get(k)
    }

    fn max_x(&self) -> i32 {
        self.iter().fold(0, |acc, (Pos { x, y: _ }, _)| acc.max(*x))
    }

    fn max_y(&self) -> i32 {
        self.iter().fold(0, |acc, (Pos { x: _, y }, _)| acc.max(*y))
    }

    fn successors(&self, pos: &Pos) -> Vec<(Pos, u32)> {
        let &Pos { x, y } = pos;
        vec![
            Pos { x: x - 1, y: y },
            Pos { x: x, y: y - 1 },
            Pos { x: x, y: y + 1 },
            Pos { x: x + 1, y: y },
        ]
        .into_iter()
        .filter_map(|p| {
            if let Some(v) = self.get(&p) {
                Some((p, *v))
            } else {
                None
            }
        })
        .collect()
    }

    fn expand(&mut self, direction: Direction, times: i32) -> &Self {
        let orig_map = self.clone();
        let max = match direction {
            Direction::X => orig_map.max_x(),
            Direction::Y => orig_map.max_y(),
        };

        for i in 1..times {
            orig_map.iter().for_each(|(pos, v)| {
                let new_pos = match direction {
                    Direction::X => Pos {
                        x: pos.x + ((max + 1) * i),
                        y: pos.y,
                    },
                    Direction::Y => Pos {
                        x: pos.x,
                        y: pos.y + ((max + 1) * i),
                    },
                };
                let new_v = match v + i as u32 {
                    n if n > 9 => n - 9,
                    n => n,
                };

                self.insert(new_pos, new_v);
            })
        }

        self
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_y = self.max_y();

        for y in 0..=self.max_x() {
            for x in 0..=max_y {
                let v = self.0.get(&Pos { x, y }).unwrap_or(&0);
                write!(f, "{}", v)?;
            }
            if y != max_y {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
    let mut dist: HashMap<Pos, u32> = map.iter().map(|(pos, _cost)| (*pos, u32::MAX)).collect();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    visited.insert(start);
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[&position] {
            continue;
        }

        for (neighbor, neighbor_cost) in map.successors(&position) {
            if visited.contains(&neighbor) {
                continue;
            }

            if neighbor_cost < dist[&neighbor] {
                let next = State {
                    cost: cost + neighbor_cost,
                    position: neighbor,
                };

                heap.push(next);
                visited.insert(neighbor);
                dist.insert(next.position, next.cost);
            }
        }
    }

    None
}

fn parse_map(data: String) -> Map {
    data.lines().enumerate().fold(Map::new(), |acc, (y, line)| {
        line.chars().enumerate().fold(acc, |mut acc, (x, c)| {
            let v = c.to_digit(10).unwrap();

            acc.insert(
                Pos {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                },
                v as u32,
            );
            acc
        })
    })
}

pub fn part1(data: String) -> u32 {
    let map = parse_map(data);
    let goal = Pos {
        x: map.max_x(),
        y: map.max_y(),
    };

    let result = shortest_path(&map, Pos { x: 0, y: 0 }, goal);

    result.unwrap()
}

pub fn part2(data: String) -> u32 {
    let mut map = parse_map(data);
    map.expand(Direction::X, 5);
    map.expand(Direction::Y, 5);

    let goal = Pos {
        x: map.max_x(),
        y: map.max_y(),
    };

    let result = shortest_path(&map, Pos { x: 0, y: 0 }, goal);

    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_x_1() {
        let mut map = parse_map("123456789".to_string());
        let new_map = parse_map("123456789".to_string());
        map.expand(Direction::X, 1);

        assert_eq!(map, new_map);
    }

    #[test]
    fn test_expand_x_2() {
        let mut map = parse_map("123456789".to_string());
        let new_map = parse_map("123456789234567891".to_string());
        map.expand(Direction::X, 2);

        assert_eq!(map, new_map);
    }

    #[test]
    fn test_expand_y() {
        let mut map = parse_map("123456789".to_string());
        let new_map = parse_map("123456789".to_string());
        map.expand(Direction::Y, 1);

        assert_eq!(map, new_map);

        let mut map = parse_map("123456789".to_string());
        let new_map = parse_map("123456789\n234567891".to_string());
        map.expand(Direction::Y, 2);

        assert_eq!(map, new_map);
    }

    #[test]
    fn test_expand_x_y() {
        let mut map = parse_map("123".to_string());
        let new_map = parse_map("123234345\n234345456\n345456567".to_string());
        map.expand(Direction::X, 3);
        map.expand(Direction::Y, 3);

        assert_eq!(map, new_map);
    }
}
