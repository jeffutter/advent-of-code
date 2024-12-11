use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use util::Pos;

type OutType = usize;

#[derive(Debug)]
pub struct Map {
    antennae: HashMap<char, HashSet<Pos<i8>>>,
    max_x: i8,
    max_y: i8,
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> Map {
    let mut antennae: HashMap<char, HashSet<Pos<i8>>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if x > max_x {
                max_x = x;
            }
            if c == '.' {
                continue;
            }

            let point = Pos::new(x as i8, y as i8);

            antennae
                .entry(c)
                .and_modify(|antennae| {
                    antennae.insert(point.clone());
                })
                .or_insert([point].into());
        }
        if y > max_y {
            max_y = y;
        }
    }

    Map {
        antennae,
        max_x: max_x as i8,
        max_y: max_y as i8,
    }
}

fn in_bounds(p: &Pos<i8>, max_x: i8, max_y: i8) -> bool {
    p.x >= 0 && p.x <= max_x && p.y >= 0 && p.y <= max_y
}

fn translate(p: &Pos<i8>, x: i8, y: i8, max_x: i8, max_y: i8) -> Option<Pos<i8>> {
    let new = Pos::new(p.x + x, p.y + y);
    if in_bounds(&new, max_x, max_y) {
        return Some(new);
    }
    None
}

#[allow(unused_variables)]
pub fn part1(map: Map) -> OutType {
    map.antennae
        .iter()
        .flat_map(|(c, locations)| locations.iter().combinations(2))
        .fold(HashSet::new(), |mut antinodes, points| {
            let diff_x = points[0].x - points[1].x;
            let diff_y = points[0].y - points[1].y;

            if let Some(new) = translate(points[0], diff_x, diff_y, map.max_x, map.max_y) {
                antinodes.insert(new);
            }
            if let Some(new) = translate(points[1], -diff_x, -diff_y, map.max_x, map.max_y) {
                antinodes.insert(new);
            }
            antinodes
        })
        .len()
}

#[allow(unused_variables)]
pub fn part2(map: Map) -> OutType {
    map.antennae
        .iter()
        .flat_map(|(c, locations)| locations.iter().combinations(2))
        .fold(HashSet::new(), |mut antinodes, points| {
            let diff_x = points[0].x - points[1].x;
            let diff_y = points[0].y - points[1].y;

            let mut a_point = Some(points[0].clone());
            while let Some(pos) = a_point {
                antinodes.insert(pos.clone());
                a_point = translate(&pos, diff_x, diff_y, map.max_x, map.max_y)
            }

            let mut b_point = Some(points[1].clone());
            while let Some(pos) = b_point {
                antinodes.insert(pos.clone());
                b_point = translate(&pos, -diff_x, -diff_y, map.max_x, map.max_y)
            }

            antinodes
        })
        .len()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#,
        1,
        14
    );

    generate_test!(
        r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#,
        2,
        34
    );

    generate_test! { 2024, 8, 1, 252}
    generate_test! { 2024, 8, 2, 839}
}
