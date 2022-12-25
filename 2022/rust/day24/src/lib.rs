use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use itertools::Itertools;

pub fn part1(state: State) -> u16 {
    bfs(state).step
}

pub fn part2(mut state: State) -> u16 {
    state = bfs(state);
    std::mem::swap(&mut state.start, &mut state.end);
    state = bfs(state);
    std::mem::swap(&mut state.start, &mut state.end);
    bfs(state).step
}

fn bfs(state: State) -> State {
    let mut seen: HashSet<(u16, u16, u16)> = HashSet::new();
    let mut q: VecDeque<State> = VecDeque::new();
    q.push_back(state.clone());
    while let Some(mut curr) = q.pop_front() {
        if !seen.insert((curr.step, curr.pos.x, curr.pos.y)) {
            continue;
        }

        if curr.pos == curr.end {
            return curr;
        }

        for state in curr.next_states() {
            q.push_back(state);
        }
    }
    state
}

pub fn parse<'a>(data: &'a str) -> State {
    let height = data.lines().count() as u16;
    let width = data.lines().nth(0).unwrap().len() as u16;

    let mut state = State::new(width, height);

    for (yidx, line) in data.lines().enumerate() {
        for (xidx, char) in line.chars().enumerate() {
            match char {
                '^' => state.up_blizzards[xidx - 1].set((yidx - 1).try_into().unwrap(), true),
                'v' => state.down_blizzards[xidx - 1].set((yidx - 1).try_into().unwrap(), true),
                '>' => state.right_blizzards[yidx - 1].set((xidx - 1).try_into().unwrap(), true),
                '<' => state.left_blizzards[yidx - 1].set((xidx - 1).try_into().unwrap(), true),
                '#' => state.walls[yidx].set(xidx.try_into().unwrap(), true),
                '.' => {
                    if yidx == 0 {
                        state.start = Point::new(xidx.try_into().unwrap(), 0);
                    }
                    if yidx == (height - 1).into() {
                        state.end = Point::new(xidx.try_into().unwrap(), yidx.try_into().unwrap());
                    }
                }
                _ => (),
            }
        }
    }

    state.pos = state.start.clone();

    state
}

#[derive(Clone)]
pub struct State {
    step: u16,
    start: Point,
    end: Point,
    pos: Point,
    up_blizzards: Vec<BitVec>,
    down_blizzards: Vec<BitVec>,
    left_blizzards: Vec<BitVec>,
    right_blizzards: Vec<BitVec>,
    walls: Vec<BitVec>,
    width: u16,
    height: u16,
}

impl State {
    fn new(width: u16, height: u16) -> Self {
        Self {
            step: 0,
            pos: Point::new(0, 0),
            start: Point::new(0, 0),
            end: Point::new(0, 0),
            up_blizzards: vec![BitVec::new(height - 2); (width - 2).into()],
            down_blizzards: vec![BitVec::new(height - 2); (width - 2).into()],
            left_blizzards: vec![BitVec::new(width - 2); (height - 2).into()],
            right_blizzards: vec![BitVec::new(width - 2); (height - 2).into()],
            walls: vec![BitVec::new(width); height.into()],
            width,
            height,
        }
    }

    fn get(&self, x: u16, y: u16) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::new();
        let point = Point::new(x.into(), y.into());
        let mut empty = true;
        if self.pos == point {
            tiles.push(Tile::Me);
        }
        if self.start == point {
            tiles.push(Tile::Start);
            return tiles;
        }
        if self.end == point {
            tiles.push(Tile::End);
            return tiles;
        }
        if y < self.height {
            if self.walls[y as usize].get(x) {
                tiles.push(Tile::Wall);
                return tiles;
            }
        }

        if x < self.width - 1 {
            if self.up_blizzards[(x - 1) as usize].get(y - 1) {
                tiles.push(Tile::Blizzard(Direction::Up));
                empty = false;
            }
            if self.down_blizzards[(x - 1) as usize].get(y - 1) {
                tiles.push(Tile::Blizzard(Direction::Down));
                empty = false;
            }
        }
        if y < self.height - 1 {
            if self.left_blizzards[(y - 1) as usize].get(x - 1) {
                tiles.push(Tile::Blizzard(Direction::Left));
                empty = false;
            }
            if self.right_blizzards[(y - 1) as usize].get(x - 1) {
                tiles.push(Tile::Blizzard(Direction::Right));
                empty = false;
            }
        }
        if empty {
            tiles.push(Tile::Empty);
        }
        tiles
    }

    fn progress_storm(&mut self) {
        self.up_blizzards
            .iter_mut()
            .for_each(|bv| bv.rotate_right(1));
        self.down_blizzards
            .iter_mut()
            .for_each(|bv| bv.rotate_left(1));
        self.left_blizzards
            .iter_mut()
            .for_each(|bv| bv.rotate_right(1));
        self.right_blizzards
            .iter_mut()
            .for_each(|bv| bv.rotate_left(1));
    }

    fn next_states(&mut self) -> Vec<State> {
        self.progress_storm();

        self.possible_moves()
            .map(|p| {
                let mut state = self.clone();
                state.pos = p;
                state.step += 1;
                state
            })
            .collect_vec()
    }

    fn possible_moves(&self) -> impl Iterator<Item = Point> + '_ {
        self.pos
            .adjacent_points()
            .into_iter()
            .filter(|p| p.x < self.width.into() && p.y < self.height.into())
            .filter(|p| {
                p.x.try_into()
                    .map(|x| {
                        p.y.try_into()
                            .map(|y| self.get(x, y).iter().all(|t| t.passable()))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
    }
}

enum Tile {
    Me,
    Start,
    End,
    Blizzard(Direction),
    Wall,
    Empty,
}

impl Tile {
    fn passable(&self) -> bool {
        match self {
            Tile::Me => true,
            Tile::Start => true,
            Tile::End => true,
            Tile::Blizzard(_) => false,
            Tile::Wall => false,
            Tile::Empty => true,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Step: {}\n", self.step)?;
        for yidx in 0..self.height {
            for xidx in 0..self.width {
                match self.get(xidx, yidx).first().unwrap() {
                    Tile::Me => write!(f, "☃")?,
                    Tile::Blizzard(Direction::Up) => write!(f, "⏶")?,
                    Tile::Blizzard(Direction::Down) => write!(f, "⏷")?,
                    Tile::Blizzard(Direction::Left) => write!(f, "⏴")?,
                    Tile::Blizzard(Direction::Right) => write!(f, "⏵")?,
                    Tile::Wall => write!(f, "#")?,
                    Tile::Empty => write!(f, " ")?,
                    Tile::Start => write!(f, "S")?,
                    Tile::End => write!(f, "E")?,
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;

        Ok(())
    }
}

#[derive(Clone)]
struct BitVec {
    d: u128,
    max: u16,
}

impl BitVec {
    fn new(max: u16) -> Self {
        if max > 127 {
            unimplemented!()
        }
        Self { d: 0u128, max }
    }

    fn set(&mut self, n: u16, on: bool) {
        if n > self.max {
            unimplemented!();
        }
        self.set_unchecked(n, on);
    }

    fn set_unchecked(&mut self, n: u16, on: bool) {
        if on {
            self.d |= 1 << n;
        } else {
            self.d &= !(1 << n);
        }
    }

    fn get(&self, n: u16) -> bool {
        if n > self.max {
            unimplemented!();
        }
        self.get_unchecked(n)
    }

    fn get_unchecked(&self, n: u16) -> bool {
        self.d ^ (self.d | (1u128 << n)) == 0u128
    }

    fn rotate_left(&mut self, n: u16) {
        if self.max == 127 {
            self.d = self.d.rotate_left(n as u32);
            return;
        }

        for _ in 0..n {
            let highest = self.get_unchecked(self.max - 1);
            self.d = self.d << 1;
            self.set_unchecked(0, highest);
            self.set_unchecked(self.max, false);
        }
    }

    fn rotate_right(&mut self, n: u16) {
        if self.max == 127 {
            self.d = self.d.rotate_right(n as u32);
            return;
        }

        for _ in 0..n {
            let lowest = self.get(0);
            self.d = self.d >> 1;
            self.set_unchecked(self.max - 1, lowest);
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    fn adjacent_points(&self) -> impl Iterator<Item = Point> {
        [
            Point::new(self.x, self.y - 1),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y + 1),
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

    #[test]
    fn test1() {
        let parsed = parse(INPUT);
        let res = part1(parsed);
        assert_eq!(18, res)
    }

    #[test]
    fn test2() {
        let parsed = parse(INPUT);
        let res = part2(parsed);
        assert_eq!(54, res)
    }

    #[test]
    fn rotate_left() {
        let mut bv = BitVec::new(10);
        assert_eq!(0b0000000000, bv.d);
        bv.set(0, true);
        assert_eq!(0b0000000001, bv.d);
        bv.set(1, true);
        assert_eq!(0b0000000011, bv.d);
        bv.rotate_left(1);
        assert_eq!(0b0000000110, bv.d);
        bv.rotate_left(1);
        assert_eq!(0b0000001100, bv.d);
        bv.rotate_left(6);
        assert_eq!(0b1100000000, bv.d);
        bv.rotate_left(1);
        assert_eq!(0b1000000001, bv.d);
        bv.rotate_left(1);
        assert_eq!(0b0000000011, bv.d);
    }

    #[test]
    fn rotate_right() {
        let mut bv = BitVec::new(10);
        assert_eq!(0b0000000000, bv.d);
        bv.set(1, true);
        assert_eq!(0b0000000010, bv.d);
        bv.set(2, true);
        assert_eq!(0b0000000110, bv.d);
        bv.rotate_right(1);
        assert_eq!(0b0000000011, bv.d);
        bv.rotate_right(1);
        assert_eq!(0b1000000001, bv.d);
        bv.rotate_right(2);
        assert_eq!(0b0110000000, bv.d);
    }
}
