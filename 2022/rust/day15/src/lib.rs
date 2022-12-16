use std::{collections::HashMap, fmt};

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1, sequence::tuple,
    IResult,
};

use rayon::prelude::*;

pub fn part1(map: Map) -> usize {
    _do_part1(map, 2000000)
}

fn _do_part1(map: Map, target: i32) -> usize {
    let mut res = 0;
    for x in map.min_x..=map.max_x {
        let pos = Pos::new(x, target);
        if map.covers(&pos) {
            res += 1;
        }
    }
    res
}

pub fn part2(map: Map) -> i64 {
    _do_part2(map, 4000000)
}

fn _do_part2(map: Map, max: i32) -> i64 {
    let pos = map
        .edge_points()
        .filter(|pos| {
            0 <= pos.x
                && pos.x <= max
                && 0 <= pos.y
                && pos.y <= max
                && !map.sensors.contains_key(pos)
                && !map.sensors.values().any(|s| s.beacon == *pos)
        })
        .find_any(|pos| !map.covers(&pos))
        .unwrap();

    (pos.x as i64 * 4000000) + pos.y as i64
}

pub fn parse<'a>(data: &'a str) -> Map {
    let (rest, map) = parse_map(data).unwrap();
    assert_eq!(rest.trim(), "");
    map
}

fn parse_map(s: &str) -> IResult<&str, Map> {
    let (rest, sensors) = separated_list1(newline, sensor)(s)?;

    let mut map = Map::new();

    for sensor in sensors {
        map.add(sensor);
    }

    Ok((rest, map))
}

fn sensor(s: &str) -> IResult<&str, Sensor> {
    let (rest, (_, x, _, y, _, beacon_x, _, beacon_y)) = tuple((
        tag("Sensor at x="),
        parser::signed_dig,
        tag(", y="),
        parser::signed_dig,
        tag(": closest beacon is at x="),
        parser::signed_dig,
        tag(", y="),
        parser::signed_dig,
    ))(s)?;

    Ok((rest, Sensor::new(x, y, beacon_x, beacon_y)))
}

#[derive(Debug)]
pub struct Map {
    sensors: HashMap<Pos, Sensor>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Map {
    pub fn new() -> Self {
        Self {
            sensors: HashMap::new(),
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
        }
    }

    pub fn add(&mut self, sensor: Sensor) {
        let min_x = sensor.pos.x - sensor.distance;
        let min_y = sensor.pos.y - sensor.distance;
        let max_x = sensor.pos.x + sensor.distance;
        let max_y = sensor.pos.y + sensor.distance;
        if min_x < self.min_x {
            self.min_x = min_x
        }
        if min_y < self.min_y {
            self.min_y = min_y
        }
        if max_x > self.max_x {
            self.max_x = max_x
        }
        if max_y > self.max_y {
            self.max_y = max_y
        }
        self.sensors.insert(sensor.pos.clone(), sensor);
    }

    fn covers(&self, pos: &Pos) -> bool {
        self.sensors.values().any(|sensor| sensor.covers(&pos))
    }

    fn edge_points(&self) -> impl ParallelIterator<Item = Pos> + '_ {
        self.sensors
            .values()
            .par_bridge()
            .flat_map(|sensor| sensor.surrounding_points())
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.min_y..self.max_y {
            write!(f, "\n")?;
            for x in self.min_x..self.max_x {
                let pos = Pos::new(x, y);
                let v = match (pos.clone(), self.covers(&pos)) {
                    (p, _) if self.sensors.iter().any(|(pp, _)| p == *pp) => "S",
                    (p, _) if self.sensors.iter().any(|(_, s)| s.beacon == p) => "B",
                    (_, true) => "#",
                    (_, false) => ".",
                };

                write!(f, "{}", v)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Sensor {
    pos: Pos,
    beacon: Pos,
    distance: i32,
}

impl Sensor {
    pub fn new(x: i32, y: i32, beacon_x: i32, beacon_y: i32) -> Self {
        let pos = Pos::new(x, y);
        let beacon = Pos::new(beacon_x, beacon_y);
        let distance = pos.distance(&beacon);

        Self {
            pos,
            beacon,
            distance,
        }
    }

    fn covers(&self, pos: &Pos) -> bool {
        if self.pos == *pos {
            return false;
        }
        if self.beacon == *pos {
            return false;
        }
        self.pos.distance(pos) <= self.distance
    }

    fn surrounding_points(&self) -> impl ParallelIterator<Item = Pos> {
        let top = self.pos.y - self.distance;
        let right = self.pos.x + self.distance;
        let bottom = self.pos.y + self.distance;
        let left = self.pos.x - self.distance;

        let top_right_points = (self.pos.x..=right + 1)
            .zip((top - 1)..=self.pos.y)
            .map(|(x, y)| Pos::new(x, y));

        let bottom_right_points = (self.pos.x..=right + 1)
            .zip((self.pos.y..=(bottom + 1)).rev())
            .map(|(x, y)| Pos::new(x, y));

        let top_left_points = ((left - 1)..=self.pos.x)
            .zip(((top - 1)..=self.pos.y).rev())
            .map(|(x, y)| Pos::new(x, y));

        let bottom_left_points = ((left - 1)..=self.pos.x)
            .zip(self.pos.y..=bottom + 1)
            .map(|(x, y)| Pos::new(x, y));

        top_right_points
            .chain(bottom_right_points)
            .chain(bottom_left_points)
            .chain(top_left_points)
            .par_bridge()
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Pos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test1() {
        let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
        let parsed = parse(input);
        let res = _do_part1(parsed, 10);
        assert_eq!(26, res)
    }

    #[test]
    fn test2() {
        let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
        let parsed = parse(input);
        let res = _do_part2(parsed, 20);
        assert_eq!(56000011, res)
    }

    #[test]
    fn test_surrounding_points() {
        let sensor = Sensor::new(3, 3, 2, 2);
        let points = sensor.surrounding_points();
        assert_eq!(
            vec![
                Pos::new(3, 0),
                Pos::new(4, 1),
                Pos::new(5, 2),
                Pos::new(6, 3),
                Pos::new(5, 4),
                Pos::new(4, 5),
                Pos::new(3, 6),
                Pos::new(2, 5),
                Pos::new(1, 4),
                Pos::new(0, 3),
                Pos::new(1, 2),
                Pos::new(2, 1),
            ]
            .into_iter()
            .collect::<HashSet<_>>(),
            points.into_iter().collect::<HashSet<_>>()
        )
    }
}
