use std::{ops::RangeInclusive, slice::Iter};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Cube {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
    on: bool,
}

impl Cube {
    pub fn new(
        x: RangeInclusive<i32>,
        y: RangeInclusive<i32>,
        z: RangeInclusive<i32>,
        on: bool,
    ) -> Self {
        Self { x, y, z, on }
    }

    fn volume(&self) -> usize {
        self.x.clone().count() * self.y.clone().count() * self.z.clone().count()
    }

    fn overlap(&self, other: &Cube) -> Option<Cube> {
        let x_start = self.x.start().max(other.x.start());
        let x_end = self.x.end().min(other.x.end());
        let y_start = self.y.start().max(other.y.start());
        let y_end = self.y.end().min(other.y.end());
        let z_start = self.z.start().max(other.z.start());
        let z_end = self.z.end().min(other.z.end());

        if x_start > x_end || y_start > y_end || z_start > z_end {
            None
        } else {
            Some(Cube::new(
                *x_start..=*x_end,
                *y_start..=*y_end,
                *z_start..=*z_end,
                !self.on,
            ))
        }
    }
}

struct Reactor(Vec<Cube>);

impl Reactor {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    fn iter(&self) -> Iter<Cube> {
        self.0.iter()
    }

    fn extend(&mut self, cubes: Vec<Cube>) {
        self.0.extend(cubes)
    }

    fn push(&mut self, cube: Cube) {
        self.0.push(cube)
    }

    fn size(&self) -> usize {
        self.iter().fold(0, |mut acc, cube| {
            match cube.on {
                true => acc += cube.volume(),
                false => acc -= cube.volume(),
            };
            acc
        })
    }

    fn remove_cube(&mut self, cube: &Cube) {
        let overlaps: Vec<Cube> = self.iter().filter_map(|part| part.overlap(cube)).collect();

        self.extend(overlaps);
    }

    fn add(&mut self, cube: &Cube) {
        self.remove_cube(cube);

        if cube.on {
            self.push(cube.clone())
        }
    }
}

fn range(s: &str) -> IResult<&str, RangeInclusive<i32>> {
    let (rest, (start, end)) =
        separated_pair(parser::signed_dig, tag(".."), parser::signed_dig)(s)?;
    Ok((rest, RangeInclusive::new(start, end)))
}

fn parse(data: String) -> Vec<Cube> {
    let (_rest, ranges) = separated_list1(
        line_ending,
        tuple((
            terminated(alt((tag("on"), tag("off"))), space1),
            preceded(tag("x="), range),
            preceded(tag(",y="), range),
            preceded(tag(",z="), range),
        )),
    )(&data)
    .unwrap();

    ranges
        .iter()
        .map(|(on_off, x, y, z)| match *on_off {
            "on" => Cube::new(x.clone(), y.clone(), z.clone(), true),
            "off" => Cube::new(x.clone(), y.clone(), z.clone(), false),
            _ => unimplemented!(),
        })
        .collect()
}

pub fn part1(data: String) -> usize {
    let cubes = parse(data);

    let mut reactor = Reactor::new();

    for cube in cubes {
        if !(-50..=50).contains(cube.x.start())
            || !(-50..=50).contains(cube.x.end())
            || !(-50..=50).contains(cube.y.start())
            || !(-50..=50).contains(cube.y.end())
            || !(-50..=50).contains(cube.z.start())
            || !(-50..=50).contains(cube.z.end())
        {
            continue;
        }
        reactor.add(&cube)
    }

    reactor.size()
}
pub fn part2(data: String) -> usize {
    let cubes = parse(data);

    let mut reactor = Reactor::new();

    for cube in cubes {
        reactor.add(&cube)
    }

    reactor.size()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "\
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"
            .to_string();

        assert_eq!(
            parse(data),
            vec![
                Cube::new(
                    RangeInclusive::new(10, 12),
                    RangeInclusive::new(10, 12),
                    RangeInclusive::new(10, 12),
                    true
                ),
                Cube::new(
                    RangeInclusive::new(11, 13),
                    RangeInclusive::new(11, 13),
                    RangeInclusive::new(11, 13),
                    true
                ),
                Cube::new(
                    RangeInclusive::new(9, 11),
                    RangeInclusive::new(9, 11),
                    RangeInclusive::new(9, 11),
                    false
                ),
                Cube::new(
                    RangeInclusive::new(10, 10),
                    RangeInclusive::new(10, 10),
                    RangeInclusive::new(10, 10),
                    true
                )
            ]
        )
    }

    #[test]
    fn test_part1() {
        let data = "\
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"
            .to_string();

        assert_eq!(part1(data), 39)
    }
}
