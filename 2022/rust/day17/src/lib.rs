use std::{collections::VecDeque, fmt};

const MAX_BLOCKS: usize = 20;

pub fn part1(jets: impl Iterator<Item = Direction> + Clone) -> usize {
    let mut state = State::new(jets.cycle(), Shape::generate().cycle());
    state.run(2022);
    state.height()
}

pub fn part2(jets: impl Iterator<Item = Direction> + Clone) -> usize {
    let mut state = State::new(jets.cycle(), Shape::generate().cycle());

    let (cycle_size, _val, start) = floyd(state.run(1), |mut last_state| last_state.run(1));

    let mut remaining_cycles = 1000000000000 - start;
    let init_state = state.clone().run(start);
    let one_cycle = init_state.clone().run(cycle_size);
    let cycle_height = one_cycle.height() - init_state.height();
    let repeated_cycles = remaining_cycles / cycle_size;
    remaining_cycles -= repeated_cycles * cycle_size;
    let bonus_cycle = init_state.clone().run(remaining_cycles);
    let bonus_height = bonus_cycle.height() - init_state.height();
    let height = init_state.height() + (repeated_cycles * cycle_height) + bonus_height;

    height - 1
}

pub fn floyd<T, FS>(start: T, successor: FS) -> (usize, T, usize)
where
    T: Clone + PartialEq,
    FS: Fn(T) -> T,
{
    let mut tortoise = successor(start.clone());
    let mut hare = successor(successor(start.clone()));
    while tortoise != hare {
        (tortoise, hare) = (successor(tortoise), successor(successor(hare)));
    }
    let mut mu = 0;
    tortoise = start;
    while tortoise != hare {
        (tortoise, hare, mu) = (successor(tortoise), successor(hare), mu + 1);
    }
    let mut lam = 1;
    hare = successor(tortoise.clone());
    while tortoise != hare {
        (hare, lam) = (successor(hare), lam + 1);
    }
    (lam, tortoise, mu)
}

pub fn parse<'a>(data: &'a str) -> impl Iterator<Item = Direction> + 'a + Clone {
    data.trim()
        .chars()
        .map(|c| match c {
            '<' => &Direction::Left,
            '>' => &Direction::Right,
            x => unimplemented!("Direction: '{}' unknown", x),
        })
        .cloned()
}

#[derive(Clone)]
struct State<J, R>
where
    J: Iterator<Item = Direction> + Clone,
    R: Iterator<Item = Shape> + Clone,
{
    cave: Cave,
    cycles: usize,
    jet_generator: J,
    rock_generator: R,
}

impl<J, R> State<J, R>
where
    J: Iterator<Item = Direction> + Clone,
    R: Iterator<Item = Shape> + Clone,
{
    pub fn new(jet_generator: J, rock_generator: R) -> Self {
        Self {
            cave: Cave::new(),
            cycles: 0,
            jet_generator,
            rock_generator,
        }
    }

    fn run(&mut self, c: usize) -> Self {
        for _ in 0..c {
            self.cave.add(
                Rock::new(
                    self.rock_generator.next().unwrap(),
                    Point::new(2, self.height() + 3),
                ),
                &mut self.jet_generator,
            );
        }

        self.cycles += c;

        Self {
            cave: self.cave.clone(),
            cycles: self.cycles,
            jet_generator: self.jet_generator.clone(),
            rock_generator: self.rock_generator.clone(),
        }
    }

    fn height(&self) -> usize {
        self.cave.height()
    }
}

impl<J, R> PartialEq for State<J, R>
where
    J: Iterator<Item = Direction> + Clone,
    R: Iterator<Item = Shape> + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.cave == other.cave
    }
}

#[derive(Clone)]
struct Cave {
    rocks: VecDeque<Rock>,
    max_y: usize,
}

impl Cave {
    pub fn new() -> Self {
        Self {
            rocks: VecDeque::new(),
            max_y: 0,
        }
    }

    fn height(&self) -> usize {
        self.max_y
    }

    fn occupied(&self) -> impl Iterator<Item = Point> + '_ {
        self.rocks.iter().flat_map(|x| x.points())
    }

    fn add(&mut self, mut rock: Rock, jets: &mut impl Iterator<Item = Direction>) {
        loop {
            self.rocks.push_front(rock.clone());
            self.rocks.pop_front();
            let direction = jets.next().unwrap();
            rock.push(&direction, self);
            if !rock.drop(self) {
                break;
            }
        }
        let max_y = rock.max_y() + 1;
        if max_y > self.max_y {
            self.max_y = max_y;
        }
        self.rocks.push_front(rock);
        self.rocks.truncate(MAX_BLOCKS);
    }
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        if self.rocks.len() != other.rocks.len() {
            return false;
        }

        let self_occupied = self.occupied().collect::<Vec<_>>();
        let self_min_y = self_occupied.iter().map(|point| point.y).min().unwrap();
        let self_shifted = self_occupied.into_iter().map(|mut point| {
            point.y -= self_min_y;
            point
        });

        let other_occupied = other.occupied().collect::<Vec<_>>();
        let other_min_y = other_occupied.iter().map(|point| point.y).min().unwrap();
        let other_shifted = other_occupied
            .into_iter()
            .map(|mut point| {
                point.y -= other_min_y;
                point
            })
            .collect::<Vec<_>>();

        for point in self_shifted {
            if !other_shifted.contains(&point) {
                return false;
            }
        }

        true
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = 0;
        let max_x = 6;
        let min_y = 0;
        let max_y = self.occupied().map(|p| p.y).max().unwrap_or(10);

        for y in (min_y..=max_y).rev() {
            write!(f, "|")?;
            for x in min_x..=max_x {
                let val = if self.occupied().any(|p| p == Point::new(x, y)) {
                    "#"
                } else {
                    "."
                };
                write!(f, "{}", val)?;
            }
            write!(f, "|")?;
            write!(f, "\n")?;
        }
        write!(f, "+-------+\n")?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Rock {
    shape: Shape,
    point: Point,
}

impl Rock {
    pub fn new(shape: Shape, point: Point) -> Self {
        Self { shape, point }
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        match self.shape {
            Shape::Horizontal => vec![
                Point::new(self.point.x, self.point.y),
                Point::new(self.point.x + 1, self.point.y),
                Point::new(self.point.x + 2, self.point.y),
                Point::new(self.point.x + 3, self.point.y),
            ],
            Shape::Plus => vec![
                Point::new(self.point.x + 1, self.point.y),
                Point::new(self.point.x, self.point.y + 1),
                Point::new(self.point.x + 1, self.point.y + 1),
                Point::new(self.point.x + 2, self.point.y + 1),
                Point::new(self.point.x + 1, self.point.y + 2),
            ],
            Shape::Angle => vec![
                Point::new(self.point.x, self.point.y),
                Point::new(self.point.x + 1, self.point.y),
                Point::new(self.point.x + 2, self.point.y),
                Point::new(self.point.x + 2, self.point.y + 1),
                Point::new(self.point.x + 2, self.point.y + 2),
            ],
            Shape::Vertical => vec![
                Point::new(self.point.x, self.point.y),
                Point::new(self.point.x, self.point.y + 1),
                Point::new(self.point.x, self.point.y + 2),
                Point::new(self.point.x, self.point.y + 3),
            ],
            Shape::Square => vec![
                Point::new(self.point.x, self.point.y),
                Point::new(self.point.x + 1, self.point.y),
                Point::new(self.point.x, self.point.y + 1),
                Point::new(self.point.x + 1, self.point.y + 1),
            ],
        }
        .into_iter()
    }

    pub fn push(&mut self, direction: &Direction, cave: &Cave) -> bool {
        if *direction == Direction::Left && self.point.x == 0 {
            return false;
        }

        let newx = match direction {
            Direction::Left => self.point.x - 1,
            Direction::Right => self.point.x + 1,
        };
        let next = Rock::new(self.shape.clone(), Point::new(newx, self.point.y));

        if next.points().any(|p| cave.occupied().any(|op| p == op))
            || next.points().any(|point| point.x > 6)
        {
            return false;
        }
        self.point = next.point;
        true
    }

    fn drop(&mut self, cave: &Cave) -> bool {
        if self.point.y == 0 {
            return false;
        }

        let next = Rock::new(
            self.shape.clone(),
            Point::new(self.point.x, self.point.y - 1),
        );

        if next.points().any(|p| cave.occupied().any(|op| p == op)) {
            return false;
        }

        self.point = next.point;
        true
    }

    fn max_y(&self) -> usize {
        self.points().map(|point| point.y).max().unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    Horizontal,
    Plus,
    Angle,
    Vertical,
    Square,
}

impl Shape {
    pub fn generate() -> impl Iterator<Item = Shape> + Clone {
        [
            Shape::Horizontal,
            Shape::Plus,
            Shape::Angle,
            Shape::Vertical,
            Shape::Square,
        ]
        .into_iter()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";
        let parsed = parse(input);
        let res = part1(parsed);
        assert_eq!(3068, res)
    }

    #[test]
    fn test2() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";
        let parsed = parse(input);
        let res = part2(parsed);
        assert_eq!(1514285714288, res)
    }
}
