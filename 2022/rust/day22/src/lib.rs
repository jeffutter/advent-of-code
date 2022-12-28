use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

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

pub fn part2(mut state: State) -> usize {
    for instruction in state.instructions.clone() {
        match instruction {
            Instruction::Move(x) => {
                for _ in 0..x {
                    state.step_folded();
                }
            }
            Instruction::Left => state.facing = state.facing.turn(Instruction::Left),
            Instruction::Right => state.facing = state.facing.turn(Instruction::Right),
        }
    }

    1000 * state.pos.y + 4 * state.pos.x + state.facing.value()
}

pub fn parse<'a>(data: &'a str) -> State {
    let mut state = State::new();
    let mut parse_moves = false;
    let mut find_start = true;

    for (yidx, line) in data.lines().enumerate() {
        if parse_moves {
            let (_, instructions) = many1(alt((mv, left, right)))(line).unwrap();
            state.set_instructions(instructions);
            break;
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

    state.cube_size = ((state.map.len() / 6) as f64).sqrt() as usize;

    let first_face_min_x = (1..=state.max_x)
        .find(|x| state.map.get(&Point::new(*x, 1)).is_some())
        .unwrap();

    let front_face = Face::new(FaceName::Front, first_face_min_x, 1, state.cube_size);

    let mut q: VecDeque<(Face, FaceName)> = VecDeque::new();

    state.faces.insert(FaceName::Front, front_face.clone());
    q.push_front((front_face.clone(), FaceName::Bottom));

    while let Some((face, prev_face)) = q.pop_front() {
        if state.faces.len() == 6 {
            break;
        }

        for (name, top_left) in face.maybe_adjacent(prev_face) {
            if state.faces.contains_key(&name) {
                continue;
            }
            if state
                .faces
                .values()
                .map(|f| f.top_left.clone())
                .any(|tl| tl == top_left)
            {
                continue;
            }
            if state.map.get(&top_left).is_some() {
                let new_face = Face::new(name.clone(), top_left.x, top_left.y, state.cube_size);
                state.faces.insert(name, new_face.clone());
                q.push_front((new_face, face.name.clone()));
            }
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

#[derive(Clone)]
pub struct Face {
    name: FaceName,
    top_left: Point,
    bottom_right: Point,
}

impl Face {
    fn new(name: FaceName, x: usize, y: usize, size: usize) -> Self {
        Self {
            name,
            top_left: Point::new(x, y),

            bottom_right: Point::new(x + size - 1, y + size - 1),
        }
    }

    fn maybe_adjacent(&self, prev: FaceName) -> Vec<(FaceName, Point)> {
        use FaceName::*;

        let above_x = self.top_left.x;
        let above_y = self.top_left.y.checked_sub(self.height());
        let above_point = above_y.map(|y| Point::new(above_x, y));
        let left_x = self.top_left.x.checked_sub(self.width());
        let left_y = self.top_left.y;
        let left_point = left_x.map(|x| Point::new(x, left_y));
        let below_x = self.top_left.x;
        let below_y = self.bottom_right.y + 1;
        let below_point = Point::new(below_x, below_y);
        let right_x = self.bottom_right.x + 1;
        let right_y = self.top_left.y;
        let right_point = Point::new(right_x, right_y);

        match (prev, &self.name) {
            (Bottom, Front) => [
                above_point.map(|point| (Top, point)),
                Some((Bottom, below_point)),
                left_point.map(|point| (Left, point)),
                Some((Right, right_point)),
            ],
            (Top, Front) => [
                above_point.map(|point| (Bottom, point)),
                Some((Top, below_point)),
                left_point.map(|point| (Right, point)),
                Some((Left, right_point)),
            ],
            (Left, Front) => [
                above_point.map(|point| (Right, point)),
                Some((Left, below_point)),
                left_point.map(|point| (Top, point)),
                Some((Bottom, right_point)),
            ],
            (Right, Front) => [
                above_point.map(|point| (Left, point)),
                Some((Right, below_point)),
                left_point.map(|point| (Bottom, point)),
                Some((Top, right_point)),
            ],
            (Front, Top) => [
                above_point.map(|point| (Back, point)),
                Some((Front, below_point)),
                left_point.map(|point| (Left, point)),
                Some((Right, right_point)),
            ],
            (Back, Top) => [
                above_point.map(|point| (Front, point)),
                Some((Back, below_point)),
                left_point.map(|point| (Right, point)),
                Some((Left, right_point)),
            ],
            (Left, Top) => [
                above_point.map(|point| (Right, point)),
                Some((Left, below_point)),
                left_point.map(|point| (Back, point)),
                Some((Front, right_point)),
            ],
            (Right, Top) => [
                above_point.map(|point| (Left, point)),
                Some((Right, below_point)),
                left_point.map(|point| (Front, point)),
                Some((Back, right_point)),
            ],
            (Back, Bottom) => [
                above_point.map(|point| (Front, point)),
                Some((Back, below_point)),
                left_point.map(|point| (Right, point)),
                Some((Left, right_point)),
            ],
            (Front, Bottom) => [
                above_point.map(|point| (Front, point)),
                Some((Back, below_point)),
                left_point.map(|point| (Left, point)),
                Some((Right, right_point)),
            ],
            (Left, Bottom) => [
                above_point.map(|point| (Right, point)),
                Some((Left, below_point)),
                left_point.map(|point| (Front, point)),
                Some((Back, right_point)),
            ],
            (Right, Bottom) => [
                above_point.map(|point| (Left, point)),
                Some((Right, below_point)),
                left_point.map(|point| (Back, point)),
                Some((Front, right_point)),
            ],

            (Bottom, Back) => [
                above_point.map(|point| (Top, point)),
                Some((Bottom, below_point)),
                left_point.map(|point| (Left, point)),
                Some((Right, right_point)),
            ],
            (Top, Back) => [
                above_point.map(|point| (Bottom, point)),
                Some((Top, below_point)),
                left_point.map(|point| (Left, point)),
                Some((Right, right_point)),
            ],
            (Left, Back) => [
                above_point.map(|point| (Right, point)),
                Some((Left, below_point)),
                left_point.map(|point| (Bottom, point)),
                Some((Top, right_point)),
            ],
            (Right, Back) => [
                above_point.map(|point| (Left, point)),
                Some((Right, below_point)),
                left_point.map(|point| (Top, point)),
                Some((Bottom, right_point)),
            ],
            (Bottom, Left) => [
                above_point.map(|point| (Back, point)),
                Some((Front, below_point)),
                left_point.map(|point| (Top, point)),
                Some((Bottom, right_point)),
            ],
            (Top, Left) => [
                above_point.map(|point| (Bottom, point)),
                Some((Top, below_point)),
                left_point.map(|point| (Front, point)),
                Some((Back, right_point)),
            ],
            (Front, Left) => [
                above_point.map(|point| (Back, point)),
                Some((Front, below_point)),
                left_point.map(|point| (Bottom, point)),
                Some((Top, right_point)),
            ],
            (Back, Left) => [
                above_point.map(|point| (Bottom, point)),
                Some((Top, below_point)),
                left_point.map(|point| (Front, point)),
                Some((Back, right_point)),
            ],
            (Bottom, Right) => [
                above_point.map(|point| (Top, point)),
                Some((Bottom, below_point)),
                left_point.map(|point| (Front, point)),
                Some((Back, right_point)),
            ],
            (Top, Right) => [
                above_point.map(|point| (Bottom, point)),
                Some((Top, below_point)),
                left_point.map(|point| (Back, point)),
                Some((Front, right_point)),
            ],
            (Front, Right) => [
                above_point.map(|point| (Back, point)),
                Some((Front, below_point)),
                left_point.map(|point| (Top, point)),
                Some((Bottom, right_point)),
            ],
            (Back, Right) => [
                above_point.map(|point| (Front, point)),
                Some((Back, below_point)),
                left_point.map(|point| (Bottom, point)),
                Some((Top, right_point)),
            ],
            _ => unreachable!(),
        }
        .into_iter()
        .filter_map(|x| x)
        .collect()
    }

    fn width(&self) -> usize {
        self.bottom_right.x - self.top_left.x + 1
    }

    fn height(&self) -> usize {
        self.bottom_right.y - self.top_left.y + 1
    }

    fn contains(&self, p: &Point) -> bool {
        p.x >= self.top_left.x
            && p.x <= self.bottom_right.x
            && p.y >= self.top_left.y
            && p.y <= self.bottom_right.y
    }
}

impl fmt::Debug for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} - [{:?},{:?}]",
            self.name, self.top_left, self.bottom_right
        )
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum FaceName {
    Front,
    Back,
    Top,
    Left,
    Right,
    Bottom,
}

pub struct State {
    map: HashMap<Point, Tile>,
    faces: HashMap<FaceName, Face>,
    cube_size: usize,
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
            faces: HashMap::new(),
            pos: Point::new(0, 0),
            instructions: vec![],
            facing: Facing::East,
            min_x: usize::MAX,
            max_x: usize::MIN,
            min_y: usize::MAX,
            max_y: usize::MIN,
            cube_size: 0,
        }
    }

    fn point_in_cube(&self, cube_x: usize, cube_y: usize, x: usize, y: usize) -> Point {
        let x = self.cube_size * (cube_x - 1) + x;
        let y = self.cube_size * (cube_y - 1) + y;
        Point::new(x, y)
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

    fn step_folded(&mut self) {
        use FaceName::*;
        use Facing::*;
        use Tile::*;

        let (curr_face_name, x_offset, y_offset) = self.face_for_point(&self.pos).unwrap();
        let exiting = (x_offset == 0 && self.facing == West)
            || (x_offset == self.cube_size - 1 && self.facing == East)
            || (y_offset == 0 && self.facing == North)
            || (y_offset == self.cube_size - 1 && self.facing == South);

        // YUCK!
        let new_face_name = if self.cube_size == 4 {
            match (&curr_face_name, &self.facing, exiting) {
                (face, _, false) => face.clone(),
                (Back, East, _) => Right,
                (Back, North, _) => Bottom,
                (Back, South, _) => Top,
                (Back, West, _) => Left,
                (Bottom, East, _) => Right,
                (Bottom, North, _) => Front,
                (Bottom, South, _) => Back,
                (Bottom, West, _) => Left,
                (Front, East, _) => Right,
                (Front, North, _) => Top,
                (Front, South, _) => Bottom,
                (Front, West, _) => Left,
                (Left, East, _) => Back,
                (Left, North, _) => Front,
                (Left, South, _) => Back,
                (Left, West, _) => Top,
                (Right, East, _) => Front,
                (Right, North, _) => Bottom,
                (Right, South, _) => Top,
                (Right, West, _) => Back,
                (Top, East, _) => Left,
                (Top, North, _) => Front,
                (Top, South, _) => Back,
                (Top, West, _) => Right,
            }
        } else {
            match (&curr_face_name, &self.facing, exiting) {
                (face, _, false) => face.clone(),
                (Back, East, _) => Right,
                (Back, North, _) => Bottom,
                (Back, South, _) => Top,
                (Back, West, _) => Left,
                (Bottom, East, _) => Right,
                (Bottom, North, _) => Front,
                (Bottom, South, _) => Back,
                (Bottom, West, _) => Left,
                (Front, East, _) => Right,
                (Front, North, _) => Top,
                (Front, South, _) => Bottom,
                (Front, West, _) => Left,
                (Left, East, _) => Back,
                (Left, North, _) => Bottom,
                (Left, South, _) => Top,
                (Left, West, _) => Front,
                (Right, East, _) => Back,
                (Right, North, _) => Top,
                (Right, South, _) => Bottom,
                (Right, West, _) => Front,
                (Top, East, _) => Back,
                (Top, North, _) => Left,
                (Top, South, _) => Right,
                (Top, West, _) => Front,
            }
        };

        let cube_point_x_diff = self.cube_point(&new_face_name).unwrap().x as i32
            - self.cube_point(&curr_face_name).unwrap().x as i32;
        let cube_point_y_diff = self.cube_point(&new_face_name).unwrap().y as i32
            - self.cube_point(&curr_face_name).unwrap().y as i32;

        let new_facing = match (cube_point_x_diff, cube_point_y_diff, &self.facing) {
            (0, 0, _) => self.facing.clone(),
            (1, 0, _) => East,
            (-1, 0, _) => West,
            (0, 1, _) => South,
            (0, -1, _) => North,
            (-1, -1, West) => West,
            (-1, 1, West) => South,
            (-1, 1, South) => West,
            (-1, 2, West) => East,
            (-1, 2, East) => West,
            (-1, 3, North) => East,
            (1, -1, East) => North,
            (1, -1, North) => East,
            (1, -1, West) => East,
            (1, 1, North) => North,
            (1, 1, East) => South,
            (1, -2, East) => West,
            (1, -2, West) => East,
            (1, -3, West) => South,
            (-2, -1, South) => North,
            (-2, 1, North) => East,
            (2, -3, South) => South,
            (-2, 3, North) => North,
            _ => unreachable!(
                "{}, {} = {:?}",
                cube_point_x_diff, cube_point_y_diff, &self.facing
            ),
        };

        let new_x_offset = match (&self.facing, &new_facing, exiting) {
            (East, East, false) => x_offset + 1,
            (West, West, false) => x_offset - 1,
            (_, _, false) => x_offset,
            (_, East, _) => 0,
            (_, West, _) => self.cube_size - 1,
            (old, new, _) if old == new => x_offset,
            (East, South, _) => self.cube_size - 1 - y_offset,
            (North, South, _) => self.cube_size - 1 - x_offset,
            (South, North, _) => self.cube_size - 1 - x_offset,
            _ => y_offset,
        };

        let new_y_offset = match (&self.facing, &new_facing, exiting) {
            (South, South, false) => y_offset + 1,
            (North, North, false) => y_offset - 1,
            (_, _, false) => y_offset,
            (_, South, _) => 0,
            (_, North, _) => self.cube_size - 1,
            (old, new, _) if old == new => y_offset,
            (East, West, _) => self.cube_size - 1 - y_offset,
            (West, East, _) => self.cube_size - 1 - y_offset,
            _ => x_offset,
        };

        let new_face = self.faces.get(&new_face_name).unwrap();

        let next_point = Point::new(
            new_face.top_left.x + new_x_offset,
            new_face.top_left.y + new_y_offset,
        );

        match self.map.get(&next_point) {
            Some(Wall) => (),
            Some(Clear) => {
                self.pos = next_point;
                self.facing = new_facing;
            }
            None => {
                println!(
                    "curr: {:?} {:?}, new_facing: {:?}, folded_next: {:?}",
                    self.pos, self.facing, new_facing, next_point
                );
                println!("{:?}", self);
                unreachable!()
            }
        }
    }

    fn step(&mut self) {
        use Facing::*;
        use Tile::*;

        let next_point = match self.facing {
            North => Point::new(self.pos.x, self.pos.y - 1),
            East => Point::new(self.pos.x + 1, self.pos.y),
            South => Point::new(self.pos.x, self.pos.y + 1),
            West => Point::new(self.pos.x - 1, self.pos.y),
        };

        match self.map.get(&next_point) {
            Some(Wall) => (),
            Some(Clear) => {
                self.pos = next_point;
            }
            None => {
                match self.facing {
                    North => {
                        let (next_y, tile) = (self.min_y..=self.max_y)
                            .rev()
                            .find_map(|y| self.map.get(&Point::new(self.pos.x, y)).map(|t| (y, t)))
                            .unwrap();

                        if *tile == Clear {
                            self.pos = Point::new(self.pos.x, next_y);
                        }
                    }
                    East => {
                        let (next_x, tile) = (self.min_x..=self.max_x)
                            .find_map(|x| self.map.get(&Point::new(x, self.pos.y)).map(|t| (x, t)))
                            .unwrap();
                        if *tile == Clear {
                            self.pos = Point::new(next_x, self.pos.y);
                        }
                    }
                    South => {
                        let (next_y, tile) = (self.min_y..=self.max_y)
                            .find_map(|y| self.map.get(&Point::new(self.pos.x, y)).map(|t| (y, t)))
                            .unwrap();
                        if *tile == Clear {
                            self.pos = Point::new(self.pos.x, next_y);
                        }
                    }
                    West => {
                        let (next_x, tile) = (self.min_x..=self.max_x)
                            .rev()
                            .find_map(|x| self.map.get(&Point::new(x, self.pos.y)).map(|t| (x, t)))
                            .unwrap();
                        if *tile == Clear {
                            self.pos = Point::new(next_x, self.pos.y);
                        }
                    }
                };
            }
        }
    }

    fn face_for_point(&self, p: &Point) -> Option<(FaceName, usize, usize)> {
        self.faces.values().find(|f| f.contains(p)).map(|f| {
            let x_offset = p.x - f.top_left.x;
            let y_offset = p.y - f.top_left.y;
            (f.name.clone(), x_offset, y_offset)
        })
    }

    fn cube_point(&self, face_name: &FaceName) -> Option<Point> {
        self.faces.get(&face_name).map(|f| {
            Point::new(
                (f.top_left.x / self.cube_size) + 1,
                (f.top_left.y / self.cube_size) + 1,
            )
        })
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FaceName::*;

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

        write!(f, "Faces:\n")?;
        let mut y = self.min_y;
        let mut x = self.min_x;
        while y <= self.max_y {
            while x <= self.max_x {
                let point = Point::new(x, y);
                let v = match self.map.get(&point) {
                    None => " ",
                    Some(_) => match self
                        .face_for_point(&point)
                        .map(|(f_name, _, _)| f_name.clone())
                    {
                        None => "X",
                        Some(Front) => "F",
                        Some(Back) => "B",
                        Some(Left) => "L",
                        Some(Right) => "R",
                        Some(Top) => "T",
                        Some(Bottom) => "D",
                    },
                };

                write!(f, "{}", v)?;

                x += self.cube_size;
            }

            write!(f, "\n")?;
            x = self.min_x;
            y += self.cube_size;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Clear,
    Wall,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Move(usize),
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
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
    use Facing::*;

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

    const SMALL_REAL: &str = r#"     ..........
     ..........
     ..........
     ..........
     ..........
     .....     
     .....     
     .....     
     .....     
     .....     
..........     
..........     
..........     
..........     
..........     
.....          
.....          
.....          
.....          
.....           "#;

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

    #[test]
    fn point_in_cube() {
        let state = parse(SMALL_REAL);
        assert_eq!(Point::new(7, 1), state.point_in_cube(2, 1, 2, 1));
        assert_eq!(Point::new(1, 12), state.point_in_cube(1, 3, 1, 2));
    }

    #[test]
    fn small_real() {
        let state = parse(SMALL_REAL);
        assert_eq!(5, state.cube_size);
    }

    #[test]
    fn internal_moves() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 1, 3, 3);
        state.facing = North;
        state.step_folded();
        assert_eq!(state.point_in_cube(2, 1, 3, 2), state.pos);
        assert_eq!(North, state.facing);
        state.facing = South;
        state.step_folded();
        assert_eq!(state.point_in_cube(2, 1, 3, 3), state.pos);
        assert_eq!(South, state.facing);
        state.facing = East;
        state.step_folded();
        assert_eq!(state.point_in_cube(2, 1, 4, 3), state.pos);
        assert_eq!(East, state.facing);
        state.facing = West;
        state.step_folded();

        assert_eq!(state.point_in_cube(2, 1, 3, 3), state.pos);
        assert_eq!(West, state.facing);
        state.step_folded();
        assert_eq!(state.point_in_cube(2, 1, 2, 3), state.pos);
        state.step_folded();
        assert_eq!(state.point_in_cube(2, 1, 1, 3), state.pos);
        state.step_folded();
        assert_eq!(state.point_in_cube(1, 3, 1, 3), state.pos);

        state.pos = state.point_in_cube(2, 1, 3, 3);
        state.facing = East;
        state.step_folded();
        assert_eq!(state.point_in_cube(2, 1, 4, 3), state.pos);
        state.step_folded();
        assert_eq!(state.point_in_cube(2, 1, 5, 3), state.pos);
        state.step_folded();
        assert_eq!(state.point_in_cube(3, 1, 1, 3), state.pos);
    }

    #[test]
    fn front_north() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 1, 2, 1);
        state.facing = North;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(1, 4, 1, 2), state.pos);
        assert_eq!(East, state.facing);
    }

    #[test]
    fn front_west() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 1, 1, 2);
        state.facing = West;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(1, 3, 1, 4), state.pos);
        assert_eq!(East, state.facing);
    }

    #[test]
    fn front_east() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 1, 5, 2);
        state.facing = East;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(3, 1, 1, 2), state.pos);
        assert_eq!(East, state.facing);
    }

    #[test]
    fn front_south() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 1, 2, 5);
        state.facing = South;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 2, 2, 1), state.pos);
        assert_eq!(South, state.facing);
    }

    #[test]
    fn bottom_west() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 2, 1, 2);
        state.facing = West;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(1, 3, 2, 1), state.pos);
        assert_eq!(South, state.facing);
    }

    #[test]
    fn bottom_east() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 2, 5, 2);
        state.facing = East;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(3, 1, 2, 5), state.pos);
        assert_eq!(North, state.facing);
    }

    #[test]
    fn bottom_north() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 2, 2, 1);
        state.facing = North;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 1, 2, 5), state.pos);
        assert_eq!(North, state.facing);
    }

    #[test]
    fn bottom_south() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 2, 2, 5);
        state.facing = South;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 3, 2, 1), state.pos);
        assert_eq!(South, state.facing);
    }

    #[test]
    fn back_east() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 3, 5, 2);
        state.facing = East;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(3, 1, 5, 4), state.pos);
        assert_eq!(West, state.facing);
    }

    #[test]
    fn back_south() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 3, 2, 5);
        state.facing = South;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(1, 4, 5, 2), state.pos);
        assert_eq!(West, state.facing);
    }

    #[test]
    fn back_west() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 3, 1, 2);
        state.facing = West;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(1, 3, 5, 2), state.pos);
        assert_eq!(West, state.facing);
    }

    #[test]
    fn back_north() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(2, 3, 2, 1);
        state.facing = North;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 2, 2, 5), state.pos);
        assert_eq!(North, state.facing);
    }

    #[test]
    fn left_north() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(1, 3, 2, 1);
        state.facing = North;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 2, 1, 2), state.pos);
        assert_eq!(East, state.facing);
    }

    #[test]
    fn left_south() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(1, 3, 2, 5);
        state.facing = South;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(1, 4, 2, 1), state.pos);
        assert_eq!(South, state.facing);
    }

    #[test]
    fn left_west() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(1, 3, 1, 2);
        state.facing = West;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 1, 1, 4), state.pos);
        assert_eq!(East, state.facing);
    }

    #[test]
    fn left_east() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(1, 3, 5, 2);
        state.facing = East;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 3, 1, 2), state.pos);
        assert_eq!(East, state.facing);
    }

    #[test]
    fn top_west() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(1, 4, 1, 2);
        state.facing = West;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 1, 2, 1), state.pos);
        assert_eq!(South, state.facing);
    }

    #[test]
    fn top_east() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(1, 4, 5, 2);
        state.facing = East;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 3, 2, 5), state.pos);
        assert_eq!(North, state.facing);
    }

    #[test]
    fn top_south() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(1, 4, 2, 5);
        state.facing = South;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(3, 1, 2, 1), state.pos);
        assert_eq!(South, state.facing);
    }

    #[test]
    fn top_north() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(1, 4, 2, 1);
        state.facing = North;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(1, 3, 2, 5), state.pos);
        assert_eq!(North, state.facing);
    }

    #[test]
    fn right_east() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(3, 1, 5, 2);
        state.facing = East;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 3, 5, 4), state.pos);
        assert_eq!(West, state.facing);
    }

    #[test]
    fn right_west() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(3, 1, 1, 2);
        state.facing = West;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 1, 5, 2), state.pos);
        assert_eq!(West, state.facing);
    }

    #[test]
    fn right_north() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(3, 1, 2, 1);
        state.facing = North;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(1, 4, 2, 5), state.pos);
        assert_eq!(North, state.facing);
    }

    #[test]
    fn right_south() {
        let mut state = parse(SMALL_REAL);
        state.pos = state.point_in_cube(3, 1, 2, 5);
        state.facing = South;
        println!("Before:\n{:?}", state);
        state.step_folded();
        println!("After:\n{:?}", state);
        assert_eq!(state.point_in_cube(2, 2, 5, 2), state.pos);
        assert_eq!(West, state.facing);
    }

    #[test]
    fn cube_point() {
        use FaceName::*;

        let state = parse(SMALL_REAL);
        assert_eq!(5, state.cube_size);
        assert_eq!(Point::new(2, 1), state.cube_point(&Front).unwrap());
        assert_eq!(Point::new(3, 1), state.cube_point(&Right).unwrap());
        assert_eq!(Point::new(2, 2), state.cube_point(&Bottom).unwrap());
        assert_eq!(Point::new(1, 3), state.cube_point(&Left).unwrap());
        assert_eq!(Point::new(2, 3), state.cube_point(&Back).unwrap());
        assert_eq!(Point::new(1, 4), state.cube_point(&Top).unwrap());
    }
}
