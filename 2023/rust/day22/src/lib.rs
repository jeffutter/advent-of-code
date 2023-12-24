use std::collections::HashMap;

use itertools::Itertools;
use util::{Cube, Direction3, Point3};

#[derive(Clone, PartialEq, Eq)]
pub struct Brick {
    cube: Cube<i32>,
    id: i32,
}

impl Brick {
    fn translate(&self, d: &Direction3) -> Option<Self> {
        self.cube
            .translate(d)
            .map(|cube| Self { cube, id: self.id })
    }

    fn min_x(&self) -> i32 {
        self.cube.min_x()
    }
    fn max_x(&self) -> i32 {
        self.cube.max_x()
    }
    fn min_y(&self) -> i32 {
        self.cube.min_y()
    }
    fn max_y(&self) -> i32 {
        self.cube.max_y()
    }
    fn min_z(&self) -> i32 {
        self.cube.min_z()
    }
    fn max_z(&self) -> i32 {
        self.cube.max_z()
    }
}

impl std::fmt::Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.id, self.cube)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Stack {
    bricks: Vec<Brick>,
}

impl Stack {
    pub fn drop(&mut self) -> usize {
        let mut changed_map = HashMap::new();

        loop {
            let mut changed = false;
            for i in 0..self.bricks.len() {
                let brick = &self.bricks[i];
                let brick_id = brick.id;

                if let Some(new_brick) = brick.translate(&Direction3::O) {
                    if new_brick.min_z() < 1 {
                        continue;
                    }

                    let mut new_stack = self.clone();
                    new_stack.bricks.remove(i);

                    let collision_count = new_stack.collision_count(&new_brick);

                    if collision_count == 0 {
                        self.bricks[i] = new_brick;
                        changed = true;
                        changed_map
                            .entry(brick_id)
                            .and_modify(|n| *n += 1)
                            .or_insert(1);
                    }
                }
            }
            if !changed {
                break;
            }
        }

        // changed_map.values().sum()
        changed_map.len()
    }

    pub fn collision_count(&self, brick: &Brick) -> usize {
        self.bricks
            .iter()
            .filter(|bbrick| bbrick.cube.collision(&brick.cube))
            .count()
    }

    pub fn could_disintegrate(&self) -> (usize, usize) {
        let mut count = 0;
        let mut dropped = 0;

        for i in 0..self.bricks.len() {
            let mut stack = self.clone();
            stack.bricks.remove(i);
            let before = stack.clone();
            let dropped_count = stack.drop();
            if stack == before {
                count += 1;
            } else {
                dropped += dropped_count;
            }
        }

        (count, dropped)
    }

    fn min_x(&self) -> i32 {
        self.bricks.iter().map(|brick| brick.min_x()).min().unwrap()
    }

    fn max_x(&self) -> i32 {
        self.bricks.iter().map(|brick| brick.max_x()).max().unwrap()
    }

    fn min_y(&self) -> i32 {
        self.bricks.iter().map(|brick| brick.min_y()).min().unwrap()
    }

    fn max_y(&self) -> i32 {
        self.bricks.iter().map(|brick| brick.max_y()).max().unwrap()
    }

    fn min_z(&self) -> i32 {
        self.bricks.iter().map(|brick| brick.min_z()).min().unwrap()
    }

    fn max_z(&self) -> i32 {
        self.bricks.iter().map(|brick| brick.max_z()).max().unwrap()
    }
}

impl std::fmt::Debug for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.bricks.clone()).finish()?;

        write!(f, "\n")?;

        for z in (self.min_z()..=self.max_z()).rev() {
            write!(f, "\n")?;

            if z == self.max_z() {
                for x in self.min_x()..=self.max_x() {
                    write!(f, "{}", x)?;
                }
                write!(f, "      ")?;
                for y in self.min_y()..=self.max_y() {
                    write!(f, "{}", y)?;
                }
                write!(f, "\n")?;
            }

            for x in self.min_x()..=self.max_x() {
                if let Some(id) = (self.min_y()..=self.max_y()).find_map(|y| {
                    let point = Point3::new(x, y, z);
                    let cube = Cube::new(point.clone(), point);

                    self.bricks.iter().find_map(|brick| {
                        if cube.intersect(&brick.cube) {
                            return Some(brick.id);
                        }
                        None
                    })
                }) {
                    write!(f, "{}", id)?;
                } else {
                    write!(f, ".")?;
                }
            }

            write!(f, " {}", z)?;

            write!(f, "    ")?;

            for y in self.min_y()..=self.max_y() {
                if let Some(id) = (self.min_x()..=self.max_x()).find_map(|x| {
                    let point = Point3::new(x, y, z);
                    let cube = Cube::new(point.clone(), point);
                    self.bricks.iter().find_map(|brick| {
                        if cube.intersect(&brick.cube) {
                            return Some(brick.id);
                        }
                        None
                    })
                }) {
                    write!(f, "{}", id)?;
                } else {
                    write!(f, ".")?;
                }
            }

            write!(f, " {}", z)?;
        }

        Ok(())
    }
}

pub fn parse<'a>(data: &'a str) -> Stack {
    let mut cubes = Vec::new();

    for line in data.lines() {
        let (front, back) = line.split("~").collect_tuple().unwrap();
        let (fx, fy, fz) = front
            .split(",")
            .map(|f| f.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let (bx, by, bz) = back
            .split(",")
            .map(|b| b.parse().unwrap())
            .collect_tuple()
            .unwrap();

        let cube = Cube::new(Point3::new(fx, fy, fz), Point3::new(bx, by, bz));

        cubes.push(cube);
    }

    cubes.sort_by_key(|cube| (cube.min_z(), cube.min_x(), cube.min_y()));

    let mut id = 0;

    let bricks = cubes
        .into_iter()
        .map(|cube| {
            id += 1;
            Brick { id, cube }
        })
        .collect_vec();

    Stack { bricks }
}

pub fn part1<'a>(mut stack: Stack) -> usize {
    stack.drop();
    stack.could_disintegrate().0
}

pub fn part2<'a>(mut stack: Stack) -> usize {
    stack.drop();
    stack.could_disintegrate().1
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 5);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 7);
    }

    generate_test! { 2023, 22, 1, 409}
    generate_test! { 2023, 22, 2, 61097}
}
