use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
};

pub fn part1(mut state: State) -> i32 {
    for _ in 0..10 {
        state.step();
    }

    (state.width() * state.height()) - (state.elves.len() as i32)
}

pub fn part2(mut state: State) -> usize {
    (1..=usize::MAX).find(|_| !state.step()).unwrap()
}

pub fn parse<'a>(data: &'a str) -> State {
    let mut state = State::new();

    for (yidx, line) in data.lines().enumerate() {
        for (xidx, char) in line.chars().enumerate() {
            if char == '#' {
                let point = Point::new(xidx as i32, yidx as i32);
                state.add_elf(point);
            }
        }
    }
    state
}

type NextMoveFn = fn(&HashSet<Point>, &Point) -> Option<Point>;

fn can_move_north(elves: &HashSet<Point>, elf: &Point) -> Option<Point> {
    if !elf.north_points().iter().any(|p| elves.contains(p)) {
        return Some(Point::new(elf.x, elf.y - 1));
    }
    return None;
}

fn can_move_east(elves: &HashSet<Point>, elf: &Point) -> Option<Point> {
    if !elf.east_points().iter().any(|p| elves.contains(p)) {
        return Some(Point::new(elf.x + 1, elf.y));
    }
    return None;
}

fn can_move_south(elves: &HashSet<Point>, elf: &Point) -> Option<Point> {
    if !elf.south_points().iter().any(|p| elves.contains(p)) {
        return Some(Point::new(elf.x, elf.y + 1));
    }
    return None;
}

fn can_move_west(elves: &HashSet<Point>, elf: &Point) -> Option<Point> {
    if !elf.west_points().iter().any(|p| elves.contains(p)) {
        return Some(Point::new(elf.x - 1, elf.y));
    }
    return None;
}

pub struct State {
    elves: HashSet<Point>,
    next_move: VecDeque<NextMoveFn>,
}

impl State {
    fn new() -> Self {
        let next_move = VecDeque::from(vec![
            can_move_north as NextMoveFn,
            can_move_south as NextMoveFn,
            can_move_west as NextMoveFn,
            can_move_east as NextMoveFn,
        ]);

        Self {
            elves: HashSet::new(),
            next_move,
        }
    }

    fn add_elf(&mut self, e: Point) {
        self.elves.insert(e);
    }

    fn min_y(&self) -> i32 {
        self.elves.iter().map(|p| p.y).min().unwrap()
    }
    fn max_y(&self) -> i32 {
        self.elves.iter().map(|p| p.y).max().unwrap()
    }
    fn min_x(&self) -> i32 {
        self.elves.iter().map(|p| p.x).min().unwrap()
    }
    fn max_x(&self) -> i32 {
        self.elves.iter().map(|p| p.x).max().unwrap()
    }
    fn width(&self) -> i32 {
        self.max_x() - self.min_x() + 1
    }
    fn height(&self) -> i32 {
        self.max_y() - self.min_y() + 1
    }

    fn step(&mut self) -> bool {
        let proposed = self
            .elves
            .iter()
            .filter(|e| e.adjacent_points().iter().any(|p| self.elves.contains(p)))
            .fold(HashMap::<Point, Point>::new(), |mut res, e| {
                let new_point = self.next_move.iter().find_map(|f| f(&self.elves, e));

                if let Some(new_point) = new_point {
                    if let Some(_) = res.insert(new_point.clone(), e.clone()) {
                        res.remove(&new_point);
                    }
                }

                res
            });

        self.next_move.rotate_left(1);

        if proposed.is_empty() {
            return false;
        }

        proposed.into_iter().for_each(|(destination, elf)| {
            self.elves.remove(&elf);
            self.elves.insert(destination);
        });

        true
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Origin: [{}, {}], size [{},{}]\n",
            self.min_x(),
            self.min_y(),
            1 + self.max_x() - self.min_x(),
            1 + self.max_y() - self.min_y()
        )?;
        for y in self.min_y()..=self.max_y() {
            for x in self.min_x()..=self.max_x() {
                match self.elves.get(&Point::new(x, y)) {
                    Some(_) => write!(f, "#")?,
                    None => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn adjacent_points(&self) -> [Point; 8] {
        [
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
        ]
    }

    fn north_points(&self) -> [Point; 3] {
        [
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
        ]
    }

    fn east_points(&self) -> [Point; 3] {
        [
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x + 1, self.y),
            Point::new(self.x + 1, self.y + 1),
        ]
    }

    fn south_points(&self) -> [Point; 3] {
        [
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
        ]
    }

    fn west_points(&self) -> [Point; 3] {
        [
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x - 1, self.y + 1),
        ]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;

    #[test]
    fn test1() {
        let parsed = parse(INPUT);
        let res = part1(parsed);
        assert_eq!(110, res)
    }

    #[test]
    fn test2() {
        let parsed = parse(INPUT);
        let res = part2(parsed);
        assert_eq!(20, res)
    }
}
