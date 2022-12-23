use std::{collections::HashMap, fmt};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::many1,
    IResult,
};

pub fn part1(mut state: State) -> usize {
    for instruction in state.instructions.clone() {
        match instruction {
            Instruction::Move(x) => {
                for _ in 0..x {
                    state.step();
                }
            }
            Instruction::Left => state.facing = state.facing.turn(Instruction::Left),
            Instruction::Right => state.facing = state.facing.turn(Instruction::Right),
        }
    }

    1000 * state.pos.y + 4 * state.pos.x + state.facing.value()
}

pub fn part2(state: State) -> i64 {
    // let cube_size = (state.max_y - state.min_y).min(state.max_x - state.min_x);
    let cube_size = ((state.map.len() / 6) as f64).sqrt() as usize;
    println!("Cube Size: {}", cube_size);
    1
}

pub fn parse<'a>(data: &'a str) -> State {
    let mut state = State::new();
    let mut parse_moves = false;
    let mut find_start = true;

    for (yidx, line) in data.lines().enumerate() {
        if parse_moves {
            let (_, instructions) = many1(alt((mv, left, right)))(line).unwrap();
            state.set_instructions(instructions);
            return state;
        }

        for (xidx, char) in line.chars().enumerate() {
            if char == '.' {
                if find_start && yidx == 0 {
                    state.pos = Point::new(xidx + 1, yidx + 1);
                    find_start = false;
                }
                let point = Point::new(xidx + 1, yidx + 1);
                state.add_point(point, Tile::Clear);
            }
            if char == '#' {
                let point = Point::new(xidx + 1, yidx + 1);
                state.add_point(point, Tile::Wall);
            }
        }

        if line.trim() == "" {
            parse_moves = true;
        }
    }

    state
}

fn mv(s: &str) -> IResult<&str, Instruction> {
    map(
        map_res(digit1, |s: &str| usize::from_str_radix(s, 10)),
        |x| Instruction::Move(x),
    )(s)
}

fn left(s: &str) -> IResult<&str, Instruction> {
    let (rest, _) = tag("L")(s)?;
    Ok((rest, Instruction::Left))
}

fn right(s: &str) -> IResult<&str, Instruction> {
    let (rest, _) = tag("R")(s)?;
    Ok((rest, Instruction::Right))
}

pub struct State {
    map: HashMap<Point, Tile>,
    instructions: Vec<Instruction>,
    pos: Point,
    facing: Facing,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl State {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            pos: Point::new(0, 0),
            instructions: vec![],
            facing: Facing::East,
            min_x: usize::MAX,
            max_x: usize::MIN,
            min_y: usize::MAX,
            max_y: usize::MIN,
        }
    }

    fn add_point(&mut self, p: Point, t: Tile) {
        if p.x < self.min_x {
            self.min_x = p.x;
        }
        if p.x > self.max_x {
            self.max_x = p.x;
        }
        if p.y < self.min_y {
            self.min_y = p.y;
        }
        if p.y > self.max_y {
            self.max_y = p.y;
        }
        self.map.insert(p, t);
    }

    fn set_instructions(&mut self, is: Vec<Instruction>) {
        self.instructions = is;
    }

    fn step(&mut self) {
        let next_point = match self.facing {
            Facing::North => Point::new(self.pos.x, self.pos.y - 1),
            Facing::East => Point::new(self.pos.x + 1, self.pos.y),
            Facing::South => Point::new(self.pos.x, self.pos.y + 1),
            Facing::West => Point::new(self.pos.x - 1, self.pos.y),
        };

        match self.map.get(&next_point) {
            Some(Tile::Wall) => (),
            Some(Tile::Clear) => {
                self.pos = next_point;
            }
            None => {
                match self.facing {
                    Facing::North => {
                        let (next_y, tile) = (self.min_y..=self.max_y)
                            .rev()
                            .find_map(|y| self.map.get(&Point::new(self.pos.x, y)).map(|t| (y, t)))
                            .unwrap();

                        if *tile == Tile::Clear {
                            self.pos = Point::new(self.pos.x, next_y);
                        }
                    }
                    Facing::East => {
                        let (next_x, tile) = (self.min_x..=self.max_x)
                            .find_map(|x| self.map.get(&Point::new(x, self.pos.y)).map(|t| (x, t)))
                            .unwrap();
                        if *tile == Tile::Clear {
                            self.pos = Point::new(next_x, self.pos.y);
                        }
                    }
                    Facing::South => {
                        let (next_y, tile) = (self.min_y..=self.max_y)
                            .find_map(|y| self.map.get(&Point::new(self.pos.x, y)).map(|t| (y, t)))
                            .unwrap();
                        if *tile == Tile::Clear {
                            self.pos = Point::new(self.pos.x, next_y);
                        }
                    }
                    Facing::West => {
                        let (next_x, tile) = (self.min_x..=self.max_x)
                            .rev()
                            .find_map(|x| self.map.get(&Point::new(x, self.pos.y)).map(|t| (x, t)))
                            .unwrap();
                        if *tile == Tile::Clear {
                            self.pos = Point::new(next_x, self.pos.y);
                        }
                    }
                };
            }
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let point = Point::new(x, y);
                if point == self.pos {
                    match self.facing {
                        Facing::North => write!(f, "^")?,
                        Facing::East => write!(f, ">")?,
                        Facing::South => write!(f, "v")?,
                        Facing::West => write!(f, "<")?,
                    }
                    continue;
                }

                match self.map.get(&point) {
                    Some(Tile::Wall) => write!(f, "#")?,
                    Some(Tile::Clear) => write!(f, ".")?,
                    None => write!(f, " ")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Tile {
    Clear,
    Wall,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Move(usize),
    Left,
    Right,
}

#[derive(Debug)]
enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    fn turn(&self, i: Instruction) -> Facing {
        match (self, i) {
            (Facing::North, Instruction::Left) => Facing::West,
            (Facing::North, Instruction::Right) => Facing::East,
            (Facing::East, Instruction::Left) => Facing::North,
            (Facing::East, Instruction::Right) => Facing::South,
            (Facing::South, Instruction::Left) => Facing::East,
            (Facing::South, Instruction::Right) => Facing::West,
            (Facing::West, Instruction::Left) => Facing::South,
            (Facing::West, Instruction::Right) => Facing::North,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> usize {
        match self {
            Facing::North => 3,
            Facing::East => 0,
            Facing::South => 1,
            Facing::West => 2,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

    #[test]
    fn test1() {
        let parsed = parse(INPUT);
        let res = part1(parsed);
        assert_eq!(6032, res)
    }

    #[test]
    fn test2() {
        let parsed = parse(INPUT);
        let res = part2(parsed);
        assert_eq!(5031, res)
    }
}
