use itertools::FoldWhile::{self, Continue, Done};
use itertools::Itertools;
use std::fmt::{self, Display};

pub fn part1<'a>(grid: Grid<i32>) -> i32 {
    let mut count: i32 = 0;

    for x in 0..=grid.max_x {
        for y in 0..=grid.max_y {
            if x == 0 || y == 0 || x == grid.max_x || y == grid.max_y {
                count += 1;
            } else {
                let v = grid.get(x, y).unwrap();
                let mut left = 0..x;
                let mut right = (x + 1)..=grid.max_x;
                let mut above = 0..y;
                let mut below = (y + 1)..=grid.max_y;

                let any_left = left.any(|x| grid.get(x, y).unwrap() >= v);
                let any_right = right.any(|x| grid.get(x, y).unwrap() >= v);
                let any_above = above.any(|y| grid.get(x, y).unwrap() >= v);
                let any_below = below.any(|y| grid.get(x, y).unwrap() >= v);

                if !(any_left && any_right && any_above && any_below) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn fold_scenic_score<T: Clone + PartialOrd>(
    grid: &Grid<T>,
    x: usize,
    y: usize,
    v: T,
    acc: i32,
) -> FoldWhile<i32> {
    let new_v = grid.get(x, y).unwrap();
    if new_v >= v {
        Done(acc + 1)
    } else {
        Continue(acc + 1)
    }
}

pub fn part2<'a>(grid: Grid<i32>) -> i32 {
    let mut max_scenic_score: i32 = 0;

    for x in 0..=grid.max_x {
        for y in 0..=grid.max_y {
            let v = grid.get(x, y).unwrap();
            let mut left = (0..x).rev();
            let mut right = (x + 1)..=grid.max_x;
            let mut above = (0..y).rev();
            let mut below = (y + 1)..=grid.max_y;

            let left_score = left
                .fold_while(0, |acc, x| fold_scenic_score(&grid, x, y, v, acc))
                .into_inner();
            let right_score = right
                .fold_while(0, |acc, x| fold_scenic_score(&grid, x, y, v, acc))
                .into_inner();
            let above_score = above
                .fold_while(0, |acc, y| fold_scenic_score(&grid, x, y, v, acc))
                .into_inner();
            let below_score = below
                .fold_while(0, |acc, y| fold_scenic_score(&grid, x, y, v, acc))
                .into_inner();

            let total_score = left_score * right_score * above_score * below_score;

            if total_score > max_scenic_score {
                max_scenic_score = total_score;
            }
        }
    }

    max_scenic_score
}

pub fn parse<'a>(data: &'a str) -> Grid<i32> {
    let mut grid = Grid::new();

    for (y, line) in data.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let v: i32 = char.to_string().parse().unwrap();
            grid.insert(x, y, v);
        }
    }

    grid
}

pub struct Grid<T> {
    data: Vec<Vec<Option<T>>>,
    max_x: usize,
    max_y: usize,
}

impl<T: Clone> Grid<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn insert(&mut self, x: usize, y: usize, v: T) {
        if x > self.max_x {
            self.max_x = x;
        }
        if y > self.max_y {
            self.max_y = y;
        }

        if let Some(row) = self.data.get_mut(y) {
            if row.len() <= x {
                row.resize(x + 1, None);
            }
            row[x] = Some(v);
        } else {
            if self.data.len() <= y {
                self.data.resize(y + 1, vec![]);
            }
            let mut new_row = vec![];
            new_row.resize(x + 1, None);
            new_row[x] = Some(v);
            self.data[y] = new_row;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        self.data
            .get(y)
            .map(|row| row.get(x))
            .flatten()
            .cloned()
            .flatten()
    }
}

impl<T: Clone + Display> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                match self.get(x, y) {
                    Some(v) => write!(f, "{}", v)?,
                    None => write!(f, "_")?,
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heights() {
        let data = r#"30373
25512
65332
33549
35390"#;
        let parsed = parse(data);
        let res = part1(parsed);
        assert_eq!(res, 21)
    }

    #[test]
    fn test_scenic_score() {
        let data = r#"30373
25512
65332
33549
35390"#;
        let parsed = parse(data);
        let res = part2(parsed);
        assert_eq!(res, 8)
    }
}
