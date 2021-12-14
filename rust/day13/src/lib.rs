use std::collections::HashSet;

use nom::{
    bytes::complete::{tag, take},
    character::complete::line_ending,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

fn point(s: &str) -> IResult<&str, Point> {
    let (rest, (x, _, y)) = tuple((parser::from_dig, tag(","), parser::from_dig))(s)?;

    Ok((rest, Point { x, y }))
}

fn points(s: &str) -> IResult<&str, HashSet<Point>> {
    let (rest, points) = separated_list1(line_ending, point)(s)?;

    let mut hs: HashSet<Point> = HashSet::new();

    for point in points {
        hs.insert(point);
    }

    Ok((rest, hs))
}

fn fold(s: &str) -> IResult<&str, Fold> {
    let (rest, (_, c, _, n)) =
        tuple((tag("fold along "), take(1usize), tag("="), parser::from_dig))(s)?;

    match c {
        "x" => Ok((rest, Fold::X(n))),
        "y" => Ok((rest, Fold::Y(n))),
        _ => panic!(),
    }
}

fn folds(s: &str) -> IResult<&str, Vec<Fold>> {
    separated_list1(line_ending, fold)(s)
}

fn parse_data(data: String) -> (HashSet<Point>, Vec<Fold>) {
    let (_rest, (points, _, folds)) = tuple((points, many1(line_ending), folds))(&data).unwrap();

    (points, folds)
}

fn fold_grid<'a, 'b>(points: &'a mut HashSet<Point>, fold: &'b Fold) -> &'a HashSet<Point> {
    for point in points.clone().iter() {
        match fold {
            Fold::X(x) => {
                if point.x > *x {
                    points.remove(point);
                    points.insert(Point {
                        x: x - (point.x - x),
                        y: point.y,
                    });
                }
            }
            Fold::Y(y) => {
                if point.y > *y {
                    points.remove(point);
                    points.insert(Point {
                        x: point.x,
                        y: y - (point.y - y),
                    });
                }
            }
        }
    }
    points
}

pub fn part1(data: String) -> usize {
    let (mut points, folds) = parse_data(data);

    fold_grid(&mut points, &folds[0]);

    points.len()
}

pub fn part2(data: String) -> String {
    let (mut points, folds) = parse_data(data);

    for fold in folds {
        fold_grid(&mut points, &fold);
    }

    let max_x = points.iter().fold(0, |max, point| max.max(point.x));
    let max_y = points.iter().fold(0, |max, point| max.max(point.y));

    let mut s: String = String::new();
    s.push('\n');

    for y in 0..=max_y {
        for x in 0..=max_x {
            match points.contains(&Point { x, y }) {
                true => s.push('#'),
                false => s.push('.'),
            }
        }
        s.push('\n')
    }

    s
}
