use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use util::Pos;

type InputType = Map;
type OutType = usize;

pub struct Map {
    points: HashMap<Pos<i32>, usize>,
    trailheads: HashSet<Pos<i32>>,
    peaks: HashSet<Pos<i32>>,
    max_x: i32,
    max_y: i32,
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let mut points = HashMap::new();
    let mut trailheads = HashSet::new();
    let mut peaks = HashSet::new();
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    for (y, line) in data.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pos = Pos::new(x as i32, y as i32);
            let height: usize = char.to_digit(10).unwrap() as usize;
            points.insert(pos.clone(), height);
            if height == 0 {
                trailheads.insert(pos.clone());
            }
            if height == 9 {
                peaks.insert(pos.clone());
            }
            if x as i32 > max_x {
                max_x = x as i32;
            }
        }
        if y as i32 > max_y {
            max_y = y as i32;
        }
    }

    Map {
        points,
        trailheads,
        peaks,
        max_x,
        max_y,
    }
}

fn num_paths(map: &Map, cur: &Pos<i32>, peak: &Pos<i32>, last: Option<&Pos<i32>>) -> usize {
    if peak == cur {
        return 1;
    }

    cur.successors_4()
        .iter()
        .filter(|next| {
            if next.x < 0 || next.y < 0 || next.x > map.max_x || next.y > map.max_y {
                return false;
            }

            if let Some(last) = &last {
                if last == next {
                    return false;
                }
            }

            let cur_height = map.points.get(cur).unwrap();
            let next_height = map.points.get(next).unwrap();

            if cur_height + 1 != *next_height {
                return false;
            }

            true
        })
        .map(|next| num_paths(map, next, peak, Some(cur)))
        .sum()
}

#[allow(unused_variables)]
pub fn part1(map: InputType) -> OutType {
    map.trailheads
        .iter()
        .cartesian_product(map.peaks.iter())
        .filter(|(trailhead, peak)| num_paths(&map, trailhead, peak, None) > 0)
        .count()
}

#[allow(unused_variables)]
pub fn part2(map: InputType) -> OutType {
    map.trailheads
        .iter()
        .cartesian_product(map.peaks.iter())
        .map(|(trailhead, peak)| num_paths(&map, trailhead, peak, None))
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    #[test]
    fn example_0() {
        let data = parse(
            r#"0123
1234
8765
9876"#,
        );
        assert_eq!(part1(data), 1)
    }

    generate_test!(
        r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#,
        1,
        36
    );

    generate_test!(
        r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#,
        2,
        81
    );

    generate_test! { 2024, 10, 1, 489}
    generate_test! { 2024, 10, 2, 1086}
}
