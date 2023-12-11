use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;
use util::Pos;

pub struct Observations {
    galaxies: HashSet<Pos<i64>>,
    min: Pos<i64>,
    max: Pos<i64>,
}

impl Observations {
    pub fn expand_empty(&mut self, expand_size: i64) {
        let mut empty_rows = vec![];
        let mut empty_columns = vec![];

        for y in self.min.y..=self.max.y {
            if (self.min.x..=self.max.x).all(|x| self.galaxies.get(&Pos::new(x, y)) == None) {
                empty_rows.push(y);
            }
        }

        for x in self.min.x..=self.max.x {
            if (self.min.y..=self.max.y).all(|y| self.galaxies.get(&Pos::new(x, y)) == None) {
                empty_columns.push(x);
            }
        }

        let count_fn = |col_or_row: &Vec<i64>, i: &i64| -> i64 {
            col_or_row
                .iter()
                .filter(|n| n < &i)
                .count()
                .try_into()
                .unwrap()
        };

        let expanded_galaxies: HashSet<Pos<i64>> = self
            .galaxies
            .iter()
            .map(|Pos { x, y }| {
                let y_offset_count = count_fn(&empty_rows, y);
                let y_offset = y_offset_count * expand_size;
                let x_offset_count = count_fn(&empty_columns, x);
                let x_offset = x_offset_count * expand_size;

                Pos {
                    x: (x - x_offset_count) + x_offset,
                    y: (y - y_offset_count) + y_offset,
                }
            })
            .collect();

        let expanded_max = Pos {
            x: expanded_galaxies.iter().map(|pos| pos.x).max().unwrap(),
            y: expanded_galaxies.iter().map(|pos| pos.y).max().unwrap(),
        };

        self.galaxies = expanded_galaxies;
        self.max = expanded_max;
    }
}

pub fn parse<'a>(data: &'a str) -> Observations {
    let mut galaxies = HashSet::new();
    let mut min = Pos::new(i64::MAX, i64::MAX);
    let mut max = Pos::new(i64::MIN, i64::MIN);

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Pos::new(x as i64, y as i64);
            min = min.min(pos.clone());
            max = max.max(pos.clone());

            if '#' == c {
                galaxies.insert(pos);
            }
        }
    }

    Observations { galaxies, min, max }
}

pub fn part1<'a>(mut observations: Observations) -> i64 {
    observations.expand_empty(2);
    sum_distances(observations)
}

pub fn part2<'a>(mut observations: Observations) -> i64 {
    observations.expand_empty(1000000);
    sum_distances(observations)
}

pub fn sum_distances(observations: Observations) -> i64 {
    observations
        .galaxies
        .iter()
        .combinations(2)
        .par_bridge()
        .map(|combination| {
            let first = combination.first().unwrap().to_owned().clone();
            let last = combination.last().unwrap().to_owned().clone();
            first.manhattan_distance(&last)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 374);
    }

    #[test]
    fn test_sample_2() {
        let mut data = parse(&SAMPLE_INPUT);
        data.expand_empty(10);
        assert_eq!(sum_distances(data), 1030);
    }

    #[test]
    fn test_sample_3() {
        let mut data = parse(&SAMPLE_INPUT);
        data.expand_empty(100);
        assert_eq!(sum_distances(data), 8410);
    }

    generate_test! { 2023, 11, 1, 9403026}
    generate_test! { 2023, 11, 2, 543018317006}
}
