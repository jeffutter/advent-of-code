use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Target {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    points: HashSet<Point>,
}

impl Target {
    pub fn new() -> Self {
        Self {
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
            points: HashSet::new(),
        }
    }

    fn insert(&mut self, point: Point) -> &Self {
        self.min_x = self.min_x.min(point.x);
        self.max_x = self.max_x.max(point.x);
        self.min_y = self.min_y.min(point.y);
        self.max_y = self.max_y.max(point.y);
        self.points.insert(point);

        self
    }

    fn contains(&self, point: Point) -> bool {
        self.points.contains(&point)
    }
}

fn range(s: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parser::signed_dig, tag(".."), parser::signed_dig)(s)
}

fn parse_input(data: String) -> Target {
    let (_, ((x1, x2), (y1, y2))) = preceded(
        tag("target area: x="),
        tuple((terminated(range, tag(", ")), preceded(tag("y="), range))),
    )(&data)
    .unwrap();

    let mut target = Target::new();
    for x in x1..=x2 {
        for y in y1..=y2 {
            target.insert(Point::new(x, y));
        }
    }

    target
}

fn step(x: &mut i32, y: &mut i32, xvel: &mut i32, yvel: &mut i32) -> () {
    *x += *xvel;
    *y += *yvel;

    *yvel -= 1;

    match *xvel {
        n if n > 0 => *xvel -= 1,
        n if n < 0 => *xvel += 1,
        _ => (),
    };
}

pub fn part1(data: String) -> i32 {
    let target = parse_input(data);

    let min_xvel = ((target.min_x * 2) as f64).sqrt() as i32;
    let max_xvel = target.max_x;

    let min_yvel = target.min_y;
    let max_yvel = target.min_y.abs() - 1;

    let mut highest_pos = i32::MIN;
    for xvel in min_xvel..=max_xvel {
        for yvel in min_yvel..=max_yvel {
            let mut local_xvel = xvel;
            let mut local_yvel = yvel;
            let mut x = 0;
            let mut y = 0;

            let mut local_highest_pos = i32::MIN;

            while x <= target.max_x && y >= target.min_y {
                local_highest_pos = local_highest_pos.max(y);

                if target.contains(Point::new(x, y)) {
                    highest_pos = highest_pos.max(local_highest_pos);
                };

                step(&mut x, &mut y, &mut local_xvel, &mut local_yvel);
            }
        }
    }

    highest_pos
}

pub fn part2(data: String) -> usize {
    let target = parse_input(data);

    let mut target_hit: HashSet<(i32, i32)> = HashSet::new();

    let min_xvel = ((target.min_x * 2) as f64).sqrt() as i32;
    let max_xvel = target.max_x;

    let min_yvel = target.min_y;
    let max_yvel = target.min_y.abs() - 1;

    for xvel in min_xvel..=max_xvel {
        for yvel in min_yvel..=max_yvel {
            let mut local_xvel = xvel;
            let mut local_yvel = yvel;
            let mut x = 0;
            let mut y = 0;

            while x <= target.max_x && y >= target.min_y {
                if target.contains(Point::new(x, y)) {
                    target_hit.insert((xvel, yvel));
                };

                step(&mut x, &mut y, &mut local_xvel, &mut local_yvel);
            }
        }
    }

    target_hit.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "target area: x=1..3, y=-1..1".to_string();

        let mut val = HashSet::new();
        val.insert(Point::new(1, -1));
        val.insert(Point::new(1, 0));
        val.insert(Point::new(1, 1));
        val.insert(Point::new(2, -1));
        val.insert(Point::new(2, 0));
        val.insert(Point::new(2, 1));
        val.insert(Point::new(3, -1));
        val.insert(Point::new(3, 0));
        val.insert(Point::new(3, 1));

        let res = parse_input(data);

        assert_eq!(res.points, val);
        assert_eq!(res.min_x, 1);
        assert_eq!(res.max_x, 3);
        assert_eq!(res.min_y, -1);
        assert_eq!(res.max_y, 1);
    }
}
