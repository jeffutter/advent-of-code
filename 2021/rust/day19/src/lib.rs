use std::collections::hash_set::Iter;
use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::{many0, many1, separated_list1},
    sequence::{preceded, terminated, tuple},
};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl Beacon {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn distance(&self, beacon: &Beacon) -> Offset {
        let Beacon {
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let Beacon {
            x: x2,
            y: y2,
            z: z2,
        } = beacon;

        Offset::new(x1 - x2, y1 - y2, z1 - z2)
    }

    fn translate(&mut self, Offset { dx, dy, dz }: &Offset) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Offset {
    dx: i32,
    dy: i32,
    dz: i32,
}

impl Offset {
    pub fn new(dx: i32, dy: i32, dz: i32) -> Self {
        Self { dx, dy, dz }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Report(HashSet<Beacon>);

impl Report {
    pub fn new() -> Self {
        Report(HashSet::new())
    }

    pub fn from_vec(vec: Vec<Beacon>) -> Self {
        Report(HashSet::from_iter(vec.iter().cloned()))
    }

    fn rotation_iter(&self) -> ReportRotator {
        ReportRotator {
            report: self,
            rotation: 0,
        }
    }

    fn iter(&self) -> Iter<Beacon> {
        self.0.iter()
    }

    fn insert(&mut self, beacon: Beacon) -> bool {
        self.0.insert(beacon)
    }

    fn translated(&self, translation: &Offset) -> Self {
        let mut report = Self::new();

        for beacon in self.iter() {
            let mut new_beacon = beacon.clone();
            new_beacon.translate(translation);
            report.insert(new_beacon);
        }

        report
    }

    fn merge(&mut self, report: &Report) -> Option<Offset> {
        let translated_candidate = report.rotation_iter().find_map(|candidate| {
            let distance_counts: HashMap<Offset, i32> = self
                .iter()
                .cartesian_product(candidate.iter())
                .map(|(a, b)| a.distance(b))
                .fold(HashMap::new(), |mut acc, distance| {
                    acc.entry(distance).and_modify(|x| *x += 1).or_insert(1);
                    acc
                });

            let translation = distance_counts.iter().max_by_key(|(_k, v)| *v);

            if let Some((translation, count)) = translation {
                if count >= &12 {
                    Some((candidate.translated(translation), translation.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        });

        if let Some((candidate, translation)) = translated_candidate {
            for beacon in candidate.iter() {
                self.insert(beacon.clone());
            }
            Some(translation)
        } else {
            None
        }
    }
}

struct ReportRotator<'a> {
    report: &'a Report,
    rotation: u8,
}

impl<'a> Iterator for ReportRotator<'a> {
    type Item = Report;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rotation > 23 {
            return None;
        }

        let next_report: Vec<Beacon> = self
            .report
            .iter()
            .map(|beacon| {
                let Beacon { x, y, z } = beacon;

                match self.rotation {
                    0 => Beacon::new(*x, *y, *z),
                    1 => Beacon::new(*x, *z, -y),
                    2 => Beacon::new(*x, -y, -z),
                    3 => Beacon::new(*x, -z, *y),
                    4 => Beacon::new(*y, *x, -z),
                    5 => Beacon::new(*y, *z, *x),
                    6 => Beacon::new(*y, -x, *z),
                    7 => Beacon::new(*y, -z, -x),
                    8 => Beacon::new(*z, *x, *y),
                    9 => Beacon::new(*z, *y, -x),
                    10 => Beacon::new(*z, -x, -y),
                    11 => Beacon::new(*z, -y, *x),
                    12 => Beacon::new(-x, *y, -z),
                    13 => Beacon::new(-x, *z, *y),
                    14 => Beacon::new(-x, -y, *z),
                    15 => Beacon::new(-x, -z, -y),
                    16 => Beacon::new(-y, *x, *z),
                    17 => Beacon::new(-y, *z, -x),
                    18 => Beacon::new(-y, -x, -z),
                    19 => Beacon::new(-y, -z, *x),
                    20 => Beacon::new(-z, *x, -y),
                    21 => Beacon::new(-z, *y, *x),
                    22 => Beacon::new(-z, -x, *y),
                    23 => Beacon::new(-z, -y, -x),
                    _ => unreachable!(),
                }
            })
            .collect();

        self.rotation += 1;

        Some(Report::from_vec(next_report))
    }
}

fn parse(data: String) -> Vec<Report> {
    let (_rest, scanners) = many1(terminated(
        preceded(
            tuple((tag("--- scanner "), digit1, tag(" ---"), line_ending)),
            map(
                separated_list1(
                    line_ending,
                    map(
                        tuple((
                            terminated(parser::signed_dig, tag(",")),
                            terminated(parser::signed_dig, tag(",")),
                            parser::signed_dig,
                        )),
                        |(x, y, z)| Beacon::new(x, y, z),
                    ),
                ),
                Report::from_vec,
            ),
        ),
        many0(line_ending),
    ))(&data)
    .unwrap();

    scanners
}

fn combine_scanners(data: String) -> (Report, Vec<Offset>) {
    let mut reports = parse(data);
    let mut combined_report = reports.pop().unwrap();
    let mut offsets: Vec<Offset> = Vec::new();

    while !reports.is_empty() {
        let report = reports.remove(0);

        if let Some(offset) = combined_report.merge(&report) {
            offsets.push(offset)
        } else {
            reports.push(report)
        }
    }

    (combined_report, offsets)
}

pub fn part1(data: String) -> usize {
    let (combined_report, _) = combine_scanners(data);

    combined_report.0.len()
}

pub fn part2(data: String) -> i32 {
    let (_, offsets) = combine_scanners(data);

    offsets
        .iter()
        .cartesian_product(offsets.iter())
        .map(|(a, b)| (a.dx - b.dx).abs() + (a.dy - b.dy).abs() + (a.dz - b.dz).abs())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "\
--- scanner 0 ---
0,2,1
4,1,2
3,3,3

--- scanner 1 ---
-1,-1,-1
-5,0,4
-2,1,1"
            .to_string();

        assert_eq!(
            parse(data),
            vec![
                Report::from_vec(vec![
                    Beacon::new(0, 2, 1),
                    Beacon::new(4, 1, 2),
                    Beacon::new(3, 3, 3)
                ]),
                Report::from_vec(vec![
                    Beacon::new(-1, -1, -1),
                    Beacon::new(-5, 0, 4),
                    Beacon::new(-2, 1, 1)
                ])
            ]
        )
    }
}
