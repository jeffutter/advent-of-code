use std::{collections::HashMap, fmt};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, peek},
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

pub fn part1<'a>(mut cave: Cave) -> usize {
    let pos = Pos::new(500, 0);
    while cave.drop(&pos, None, &Contents::Sand) {}
    cave.points
        .iter()
        .filter(|(_, contents)| ***contents == Contents::Sand)
        .count()
}

pub fn part2<'a>(mut cave: Cave) -> usize {
    let pos = Pos::new(500, 0);
    let floor_y = cave.max_y + 2;
    while cave.drop(&pos, Some(floor_y), &Contents::Sand) {}
    cave.points
        .iter()
        .filter(|(_, contents)| ***contents == Contents::Sand)
        .count()
}

pub fn parse<'a>(data: &'a str) -> Cave {
    let (rest, segment_lines) = segment_lines(data).unwrap();
    assert_eq!(rest.trim(), "");

    let mut cave = Cave::new();
    for segment in segment_lines.iter().flatten() {
        cave.add(segment, &Contents::Rock)
    }

    cave
}

fn segment_lines(s: &str) -> IResult<&str, Vec<Vec<Segment>>> {
    separated_list1(newline, segments)(s)
}

fn segments(s: &str) -> IResult<&str, Vec<Segment>> {
    terminated(many1(segment), pos)(s)
}

fn segment(s: &str) -> IResult<&str, Segment> {
    let (rest, (start, _, end)) = tuple((pos, tag(" -> "), peek(pos)))(s)?;

    Ok((rest, Segment::new(start, end)))
}

fn pos(s: &str) -> IResult<&str, Pos> {
    let (rest, (x, _, y)) = tuple((
        map_res(digit1, |s: &str| usize::from_str_radix(s, 10)),
        tag(","),
        map_res(digit1, |s: &str| usize::from_str_radix(s, 10)),
    ))(s)?;

    Ok((rest, Pos::new(x, y)))
}

#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub struct Segment {
    start: Pos,
    end: Pos,
}

impl Segment {
    pub fn new(start: Pos, end: Pos) -> Self {
        if let [start, end] = &vec![start, end].into_iter().sorted().collect::<Vec<Pos>>()[..] {
            Self {
                start: start.clone(),
                end: end.clone(),
            }
        } else {
            unimplemented!()
        }
    }

    pub fn to_pos(&self) -> Vec<Pos> {
        let mut pos: Vec<Pos> = Vec::new();
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                pos.push(Pos::new(x, y))
            }
        }

        pos
    }
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

#[derive(Debug)]
pub struct Cave<'a> {
    points: HashMap<Pos, &'a Contents>,
    max_y: usize,
    max_x: usize,
    min_y: usize,
    min_x: usize,
}

impl<'a> Cave<'a> {
    pub fn new() -> Self {
        let hm: HashMap<Pos, &'a Contents> = HashMap::new();
        Self {
            points: hm,
            max_y: 0,
            max_x: 0,
            min_y: usize::MAX,
            min_x: usize::MAX,
        }
    }

    fn add(&mut self, segment: &Segment, contents: &'a Contents) {
        for pos in segment.to_pos() {
            self.add_pos(pos, contents)
        }
    }

    fn add_pos(&mut self, pos: Pos, contents: &'a Contents) {
        if pos.y > self.max_y {
            self.max_y = pos.y
        }
        if pos.y < self.min_y {
            self.min_y = pos.y
        }
        if pos.x > self.max_x {
            self.max_x = pos.x
        }
        if pos.x < self.min_x {
            self.min_x = pos.x
        }
        self.points.insert(pos, contents);
    }

    fn get(&self, pos: &Pos) -> Option<&'a &Contents> {
        self.points.get(pos)
    }

    fn contains(&self, pos: &Pos) -> bool {
        self.points.contains_key(pos)
    }

    fn drop(&mut self, pos: &Pos, floor_y: Option<usize>, contents: &'a Contents) -> bool {
        let origin = pos.clone();
        let mut cur = pos.clone();
        loop {
            let points = vec![cur.below(), cur.diagonal_left(), cur.diagonal_right()];

            let next = points.into_iter().find(|point| !self.contains(&point));

            match (next, cur, floor_y) {
                (None, cur, _) if cur == origin => {
                    self.add_pos(cur, contents);
                    return false;
                }
                (None, cur, _) => {
                    self.add_pos(cur, contents);
                    return true;
                }
                (Some(pos), cur, Some(floor_y)) if pos.y == floor_y => {
                    self.add_pos(cur, contents);
                    return true;
                }
                (Some(pos), _, Some(_)) => cur = pos,
                (Some(pos), _, None) if pos.y > self.max_y => return false,
                (Some(pos), _, None) => cur = pos,
            }
        }
    }
}

impl<'a> fmt::Display for Cave<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let val = match self.get(&Pos::new(x, y)) {
                    Some(Contents::Sand) => "o",
                    Some(Contents::Rock) => "#",
                    None => ".",
                };
                write!(f, "{}", val)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn diagonal_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    fn diagonal_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Contents {
    Sand,
    Rock,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sand1() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
        let parsed = parse(input);
        let res = part1(parsed);
        assert_eq!(24, res)
    }

    #[test]
    fn test_sand2() {
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
        let parsed = parse(input);
        let res = part2(parsed);
        assert_eq!(93, res)
    }
}
