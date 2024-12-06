use std::collections::HashSet;

use util::{Direction, Pos};

type InputType = Map;
type OutType = usize;
type P = Pos<i32>;

#[derive(Clone)]
pub struct Map {
    obstructions: HashSet<P>,
    cur: P,
    cur_direction: Direction,
    bottom_left: P,
    top_right: P,
}

impl Map {
    fn turn(&mut self) {
        self.cur_direction = turn(&self.cur_direction);
    }

    fn next(&self) -> Next {
        let next = self.cur.translate(&self.cur_direction).unwrap();

        if out_of_bounds(&self.bottom_left, &self.top_right, &next) {
            return Next::OutOfBounds;
        }

        if self.obstructions.contains(&next) {
            return Next::Blocked;
        }

        Next::Ok(next)
    }
}

enum Next {
    Ok(P),
    Blocked,
    OutOfBounds,
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let mut obstructions = HashSet::new();
    let mut cur = None;
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstructions.insert(Pos::new(x as i32, y as i32));
                }
                '^' => {
                    cur = Some(Pos::new(x as i32, y as i32));
                }
                '.' => (),
                _ => unimplemented!(),
            }
            if x > max_x {
                max_x = x;
            }
        }
        if y > max_y {
            max_y = y;
        }
    }

    Map {
        obstructions,
        cur: cur.unwrap(),
        cur_direction: Direction::N,
        top_right: Pos::new(max_x as i32, max_y as i32),
        bottom_left: Pos::new(0, 0),
    }
}

fn turn(d: &Direction) -> Direction {
    match d {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    }
}

fn out_of_bounds(bottom_left: &P, top_right: &P, p: &P) -> bool {
    p.x < bottom_left.x || p.x > top_right.x || p.y < bottom_left.y || p.y > top_right.y
}

pub fn patrol(mut map: Map) -> HashSet<P> {
    let mut visited = HashSet::new();

    loop {
        visited.insert(map.cur.clone());
        match map.next() {
            Next::Ok(pos) => map.cur = pos,
            Next::Blocked => {
                map.turn();
                continue;
            }
            Next::OutOfBounds => break,
        }
    }

    visited
}

#[allow(unused_variables)]
pub fn part1(map: InputType) -> OutType {
    patrol(map).len()
}

#[allow(unused_variables)]
pub fn part2(map: InputType) -> OutType {
    let mut possible_new_obstructions = patrol(map.clone());
    possible_new_obstructions.remove(&map.cur);

    let mut cycles_found = 0;

    for new_obstruction in possible_new_obstructions {
        let mut map = map.clone();
        map.obstructions.insert(new_obstruction.clone());

        let mut visited: HashSet<(P, Direction)> = HashSet::new();

        loop {
            let cur_pd = (map.cur.clone(), map.cur_direction.clone());

            if !visited.insert(cur_pd.clone()) {
                cycles_found += 1;
                break;
            };

            match map.next() {
                Next::Ok(pos) => map.cur = pos,
                Next::Blocked => {
                    map.turn();
                    continue;
                }
                Next::OutOfBounds => {
                    break;
                }
            }
        }
    }

    cycles_found
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#,
        1,
        41
    );

    generate_test!(
        r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#,
        2,
        6
    );

    generate_test! { 2024, 6, 1, 5080}
    generate_test! { 2024, 6, 2, 1919}
}
