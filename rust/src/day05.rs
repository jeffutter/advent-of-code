use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::{
        complete::{digit1, space1},
        streaming::line_ending,
    },
    combinator::map_res,
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

#[derive(Debug, PartialEq, Eq)]
struct Pipe {
    start: Point,
    end: Point,
}

impl Pipe {
    pub fn new(start: Point, end: Point) -> Pipe {
        Pipe { start, end }
    }

    fn points(&self) -> Vec<Point> {
        let ydiff = self.start.y - self.end.y;
        let xdiff = self.start.x - self.end.x;

        match (xdiff.abs(), ydiff.abs()) {
            (maxxdiff, maxydiff) if maxxdiff >= maxydiff => {
                let slope = match xdiff {
                    0 => 0,
                    n => ydiff / n,
                };
                let c = self.start.y - self.start.x * slope;

                let maxx = self.start.x.max(self.end.x);
                let minx = self.start.x.min(self.end.x);

                (minx..=maxx)
                    .map(|x| {
                        let y = slope * x + c;
                        Point::new(x, y)
                    })
                    .collect()
            }
            (maxxdiff, maxydiff) if maxxdiff < maxydiff => {
                let slope = match ydiff {
                    0 => 0,
                    n => xdiff / n,
                };
                let c = self.start.x - self.start.y * slope;

                let maxy = self.start.y.max(self.end.y);
                let miny = self.start.y.min(self.end.y);

                (miny..=maxy)
                    .map(|y| {
                        let x = slope * y + c;
                        Point::new(x, y)
                    })
                    .collect()
            }
            (_, _) => unreachable!(),
        }
    }
}

fn from_dig(s: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| i32::from_str_radix(s, 10))(s)
}

fn point(s: &str) -> IResult<&str, Point> {
    let (rest, (x, _, y)) = tuple((from_dig, tag(","), from_dig))(s)?;
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

    let mut board: HashMap<Point, u32> = HashMap::new();

    pipes
        .iter()
        .filter(|pipe| pipe.start.x == pipe.end.x || pipe.start.y == pipe.end.y)
        .for_each(|pipe| {
            pipe.points().iter().for_each(|point| {
                board
                    .entry(point.clone())
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            })
        });

    board.iter().filter(|(_, n)| **n > 1).count()
}

pub fn part2(data: String) -> usize {
    let (_res, pipes) = pipes(&data).unwrap();

    let mut board: HashMap<Point, u32> = HashMap::new();

    pipes.iter().for_each(|pipe| {
        pipe.points().iter().for_each(|point| {
            board
                .entry(point.clone())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        })
    });

    board.iter().filter(|(_, n)| **n > 1).count()
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
}
