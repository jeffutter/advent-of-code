use core::fmt;
use parser;
use std::collections::hash_map::{Entry, Iter};
use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::{complete::space1, streaming::line_ending},
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Pipe {
    start: Point,
    end: Point,
}

impl Pipe {
    pub fn new(start: Point, end: Point) -> Pipe {
        if start.x >= end.x {
            Pipe {
                start: end,
                end: start,
            }
        } else {
            Pipe { start, end }
        }
    }

    fn straignt(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn descending(&self) -> bool {
        (self.start.x <= self.end.x && self.start.y <= self.end.y)
            || (self.start.x >= self.end.x && self.start.y >= self.end.y)
    }

    fn points(&self) -> Vec<Point> {
        let maxx = self.start.x.max(self.end.x);
        let minx = self.start.x.min(self.end.x);

        let maxy = self.start.y.max(self.end.y);
        let miny = self.start.y.min(self.end.y);

        let mut acc: Vec<Point> = Vec::new();

        if self.straignt() {
            for x in minx..=maxx {
                for y in miny..=maxy {
                    acc.push(Point::new(x, y))
                }
            }
        } else {
            let yrange: Vec<i32> = if !self.descending() {
                (miny..=maxy).rev().collect()
            } else {
                (miny..=maxy).collect()
            };

            (minx..=maxx)
                .zip(yrange)
                .for_each(|(x, y)| acc.push(Point::new(x, y)))
        }

        acc
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

struct Pipes(HashMap<Point, u32>);

impl Pipes {
    fn new() -> Pipes {
        Pipes(HashMap::new())
    }

    fn entry(&mut self, key: Point) -> Entry<Point, u32> {
        self.0.entry(key)
    }

    fn iter(&self) -> Iter<Point, u32> {
        self.0.iter()
    }
}

fn point(s: &str) -> IResult<&str, Point> {
    let (rest, (x, _, y)) = tuple((parser::from_dig, tag(","), parser::from_dig))(s)?;
    Ok((rest, Point::new(x, y)))
}

fn pipe(s: &str) -> IResult<&str, Pipe> {
    let (rest, (start, _, _, _, end)) = tuple((point, space1, tag("->"), space1, point))(s)?;

    Ok((rest, Pipe::new(start, end)))
}

fn pipes(s: &str) -> IResult<&str, Vec<Pipe>> {
    many0(terminated(pipe, line_ending))(s)
}

pub fn part1(data: String) -> usize {
    let (_res, pipes) = pipes(&data).unwrap();

    let mut diagram: HashMap<Point, u32> = HashMap::new();

    pipes
        .iter()
        .filter(|pipe| pipe.straignt())
        .for_each(|pipe| {
            pipe.points().iter().for_each(|point| {
                diagram
                    .entry(point.clone())
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            })
        });

    diagram.iter().filter(|(_, n)| **n >= 2).count()
}

impl fmt::Display for Pipes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (maxx, maxy) = self.0.iter().fold((0, 0), |(prevx, prevy), (point, _)| {
            (prevx.max(point.x), prevy.max(point.y))
        });

        for y in 0..=maxy {
            for x in 0..=maxx {
                let v = match self.0.get(&Point::new(x, y)).unwrap_or(&0) {
                    0 => '.',
                    n => char::from_digit(*n, 10).unwrap(),
                };

                write!(f, "{}", v)?
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

pub fn part2(data: String) -> usize {
    let (_res, pipes) = pipes(&data).unwrap();

    let mut diagram: Pipes = Pipes::new();

    pipes.iter().for_each(|pipe| {
        pipe.points().iter().for_each(|point| {
            diagram
                .entry(point.clone())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        })
    });

    diagram.iter().filter(|(_, n)| **n >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_parser() {
        let data = "603,336 -> 603,368";

        let p = Pipe::new(Point::new(603, 336), Point::new(603, 368));

        assert_eq!(pipe(&data).unwrap(), ("", p));
    }

    #[test]
    fn test_pipes_parser() {
        let data = "\
603,336 -> 603,368
123,1 -> 11,999
";

        let p = vec![
            Pipe::new(Point::new(603, 336), Point::new(603, 368)),
            Pipe::new(Point::new(123, 1), Point::new(11, 999)),
        ];

        assert_eq!(pipes(&data).unwrap(), ("", p));
    }

    #[test]
    fn test_points() {
        let pipe = Pipe::new(Point::new(1, 1), Point::new(1, 10));

        let ans = vec![
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(1, 3),
            Point::new(1, 4),
            Point::new(1, 5),
            Point::new(1, 6),
            Point::new(1, 7),
            Point::new(1, 8),
            Point::new(1, 9),
            Point::new(1, 10),
        ];

        assert_eq!(pipe.points(), ans);

        let pipe2 = Pipe::new(Point::new(1, 1), Point::new(5, 5));

        let ans2 = vec![
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(3, 3),
            Point::new(4, 4),
            Point::new(5, 5),
        ];

        assert_eq!(pipe2.points(), ans2);

        let pipe3 = Pipe::new(Point::new(2, 2), Point::new(2, 1));
        let ans3 = vec![Point::new(2, 1), Point::new(2, 2)];

        assert_eq!(pipe3.points(), ans3);
    }

    #[test]
    fn test_asc_points() {
        let pipe4 = Pipe::new(Point::new(8, 0), Point::new(0, 8));

        let ans4 = vec![
            Point::new(0, 8),
            Point::new(1, 7),
            Point::new(2, 6),
            Point::new(3, 5),
            Point::new(4, 4),
            Point::new(5, 3),
            Point::new(6, 2),
            Point::new(7, 1),
            Point::new(8, 0),
        ];

        assert_eq!(pipe4.points(), ans4);
    }
}
