use std::{
    collections::{hash_map::Iter, HashMap},
    fmt,
    fmt::Debug,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?},{:?})", self.x, self.y)
    }
}

struct Image {
    data: HashMap<Point, bool>,
    inverted: bool,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Image {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            inverted: false,
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
        }
    }

    fn get(&self, point: &Point) -> Option<&bool> {
        self.data.get(point)
    }

    fn insert(&mut self, point: Point, val: bool) -> Option<bool> {
        self.min_x = self.min_x.min(point.x);
        self.min_y = self.min_y.min(point.y);
        self.max_x = self.max_x.max(point.x);
        self.max_y = self.max_y.max(point.y);
        self.data.insert(point, val)
    }

    fn iter(&self) -> Iter<Point, bool> {
        self.data.iter()
    }

    fn enhance(&self, algo: &Vec<bool>) -> Image {
        let mut output: Image = Image::new();
        output.inverted = !self.inverted;

        for x in (self.min_x - 1)..=(self.max_x + 1) {
            for y in (self.min_y - 1)..=(self.max_y + 1) {
                let point = Point::new(x, y);

                let bits = self.surrounding_values(algo, &point);
                let algo_index = bits_to_number(bits);
                let out_bit = algo.get(algo_index).unwrap();

                output.insert(point, *out_bit);
            }
        }

        output
    }

    fn enhance_n(self, algo: &Vec<bool>, n: u8) -> Image {
        (0..n).fold(self, |acc, _i| acc.enhance(algo))
    }

#[rustfmt::skip]
    fn surrounding_values(&self, algo: &Vec<bool>, point: &Point) -> Vec<bool> {
    let Point { x, y } = point;

    [
        (x - 1,  y - 1), (*x,  y - 1), (x + 1,  y - 1),
        (x - 1, *y    ), (*x, *y    ), (x + 1, *y    ),
        (x - 1,  y + 1), (*x,  y + 1), (x + 1,  y + 1),
    ]
    .iter()
    .map(|(x, y)| {
        match self.get(&Point::new(*x, *y)) {
            Some(true) => true,
            Some(false) => false,
            None => {
              *algo.get(0).unwrap() && self.inverted
            },
        }
    })
    .collect()
	}
}

impl Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;

        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let res = match self.data.get(&Point::new(x, y)) {
                    Some(true) => write!(f, "#"),
                    Some(false) => write!(f, "."),
                    None => write!(f, " "),
                };

                res?
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn pixel(s: &str) -> IResult<&str, bool> {
    map(alt((tag("#"), tag("."))), |s| match s {
        "#" => true,
        "." => false,
        _ => unimplemented!(),
    })(s)
}

fn parse(data: String) -> (Vec<bool>, Image) {
    let (_rest, (algo, raw_input)) = tuple((
        terminated(many1(pixel), line_ending),
        preceded(
            many1(line_ending),
            separated_list1(line_ending, many1(pixel)),
        ),
    ))(&data)
    .unwrap();

    let mut input = Image::new();

    for (y, row) in raw_input.iter().enumerate() {
        for (x, data) in row.iter().enumerate() {
            input.insert(Point::new(x as i32, y as i32), data.clone());
        }
    }

    (algo, input)
}

fn bits_to_number(b: Vec<bool>) -> usize {
    let mut u = 0b0000000000000000;

    for bit in b {
        u <<= 1;
        if bit {
            u ^= 1;
        }
    }

    u
}

pub fn part1(data: String) -> usize {
    let (algo, input) = parse(data);

    let e = input.enhance_n(&algo, 2);

    e.iter().filter(|(_k, v)| **v).count()
}

pub fn part2(data: String) -> usize {
    let (algo, input) = parse(data);

    let e = input.enhance_n(&algo, 50);

    e.iter().filter(|(_k, v)| **v).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_number() {
        assert_eq!(
            bits_to_number(vec![
                false, false, false, true, false, false, false, true, false
            ]),
            34
        );
    }

    #[test]
    fn test_surrounding_values() {
        let mut data: Image = Image::new();

        data.insert(Point::new(0, 0), true);
        data.insert(Point::new(1, 0), true);
        data.insert(Point::new(2, 0), false);
        data.insert(Point::new(0, 1), false);
        data.insert(Point::new(1, 1), true);
        data.insert(Point::new(2, 1), true);
        data.insert(Point::new(0, 2), true);
        data.insert(Point::new(1, 2), false);
        data.insert(Point::new(2, 2), true);

        assert_eq!(
            data.surrounding_values(&vec![false], &Point::new(1, 1)),
            vec![true, true, false, false, true, true, true, false, true]
        );
    }
}
