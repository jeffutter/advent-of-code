use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

pub fn part1(v: Volcano) -> usize {
    v.lava.iter().fold(0, |acc, c| {
        acc + c.neighbors().iter().filter(|n| !v.lava.contains(n)).count()
    })
}

pub fn part2(v: Volcano) -> usize {
    let mut outside: HashSet<Cube> = HashSet::new();
    let mut q: VecDeque<Cube> = VecDeque::new();
    q.push_front(Cube::new((v.min_x - 1, v.min_y - 1, v.min_z - 1)));

    while let Some(cube) = q.pop_back() {
        for n in cube.neighbors() {
            if !((v.min_x - 1)..=(v.max_x + 1)).contains(&n.x)
                || !((v.min_y - 1)..=(v.max_y + 1)).contains(&n.y)
                || !((v.min_z - 1)..=(v.max_z + 1)).contains(&n.z)
            {
                continue;
            }

            if !v.lava.contains(&cube) {
                if outside.insert(n.clone()) {
                    q.push_back(n);
                }
            }
        }
    }

    v.lava.iter().fold(0, |acc, c| {
        acc + c
            .neighbors()
            .iter()
            .filter(|n| !v.lava.contains(n) && outside.contains(n))
            .count()
    })
}

pub fn parse<'a>(data: &'a str) -> Volcano {
    data.lines()
        .map(|l| {
            Cube::new(
                l.split(",")
                    .map(|c| c.parse().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect::<Volcano>()
}

pub fn dfs<N, FN, IN, FS>(start: N, mut successors: FN, mut success: FS) -> Option<Vec<N>>
where
    N: Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    let mut path = vec![start];
    step(&mut path, &mut successors, &mut success).then_some(path)
}

fn step<N, FN, IN, FS>(path: &mut Vec<N>, successors: &mut FN, success: &mut FS) -> bool
where
    N: Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    if success(path.last().unwrap()) {
        true
    } else {
        let successors_it = successors(path.last().unwrap());
        for n in successors_it {
            if !path.contains(&n) {
                path.push(n);
                if step(path, successors, success) {
                    return true;
                }
                path.pop();
            }
        }
        false
    }
}

pub struct Volcano {
    lava: HashSet<Cube>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

impl Volcano {
    pub fn new() -> Self {
        Self {
            lava: HashSet::new(),
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            min_z: i32::MAX,
            max_z: i32::MIN,
        }
    }

    fn add(&mut self, cube: Cube) {
        self.min_x = self.min_x.min(cube.x);
        self.max_x = self.max_x.max(cube.x);
        self.min_y = self.min_y.min(cube.y);
        self.max_y = self.max_y.max(cube.y);
        self.min_z = self.min_z.min(cube.z);
        self.max_z = self.max_z.max(cube.z);
        self.lava.insert(cube);
    }
}

impl FromIterator<Cube> for Volcano {
    fn from_iter<T: IntoIterator<Item = Cube>>(iter: T) -> Self {
        let mut v = Volcano::new();
        for cube in iter {
            v.add(cube);
        }
        v
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    pub fn new((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }

    pub fn neighbors(&self) -> HashSet<Cube> {
        [
            Cube::new((self.x - 1, self.y, self.z)),
            Cube::new((self.x, self.y - 1, self.z)),
            Cube::new((self.x, self.y, self.z - 1)),
            Cube::new((self.x, self.y, self.z + 1)),
            Cube::new((self.x, self.y + 1, self.z)),
            Cube::new((self.x + 1, self.y, self.z)),
        ]
        .into_iter()
        .collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
}
