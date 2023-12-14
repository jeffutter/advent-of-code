#[derive(Debug)]
pub struct Valley {
    cols: Vec<u32>,
    rows: Vec<u32>,
}

pub fn parse<'a>(data: &'a str) -> Vec<Valley> {
    let mut valleys = Vec::new();

    for valley in data.split("\n\n") {
        let num_cols = valley.lines().next().unwrap().len();
        let num_rows = valley.lines().count();
        let mut cols: Vec<u32> = vec![0; num_cols];
        let mut rows: Vec<u32> = vec![0; num_rows];

        for (y, line) in valley.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        let row = rows.get_mut(y).unwrap();
                        let col = cols.get_mut(x).unwrap();

                        *row |= 1 << x;
                        *col |= 1 << y;
                    }
                    _ => (),
                }
            }
        }

        valleys.push(Valley { rows, cols })
    }

    valleys
}

fn check_reflection(rows: &Vec<u32>, idx: usize) -> bool {
    let (left, right) = rows.split_at(idx);
    left.iter().rev().zip(right.iter()).all(|(l, r)| l == r)
}

fn check_reflection_smudge(rows: &Vec<u32>, idx: usize) -> bool {
    let (left, right) = rows.split_at(idx);

    let mut found_one_bit_off = false;
    for (l, r) in left.iter().rev().zip(right.iter()) {
        if !found_one_bit_off && one_bit_off(*l, *r) {
            found_one_bit_off = true;
            continue;
        }
        if l != r {
            return false;
        }
    }

    found_one_bit_off
}

fn find_mirror(rows: Vec<u32>, smudge: bool) -> Option<usize> {
    for idx in 1..rows.len() {
        if !smudge && check_reflection(&rows, idx) {
            return Some(idx);
        }

        if smudge && check_reflection_smudge(&rows, idx) {
            return Some(idx);
        }
    }
    None
}

fn sum_mirrors(valleys: Vec<Valley>, smudge: bool) -> usize {
    valleys
        .into_iter()
        .map(|valley| {
            if let Some(idx) = find_mirror(valley.rows, smudge) {
                return idx * 100;
            }
            if let Some(idx) = find_mirror(valley.cols, smudge) {
                return idx;
            }
            0
        })
        .sum()
}

fn one_bit_off(a: u32, b: u32) -> bool {
    if a == b {
        return false;
    }
    let val = a ^ b;
    val & (val - 1) == 0
}

pub fn part1<'a>(valleys: Vec<Valley>) -> usize {
    sum_mirrors(valleys, false)
}

pub fn part2<'a>(valleys: Vec<Valley>) -> usize {
    sum_mirrors(valleys, true)
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 405);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 400);
    }

    generate_test! { 2023, 13, 1, 29846}
    generate_test! { 2023, 13, 2, 25401}
}
