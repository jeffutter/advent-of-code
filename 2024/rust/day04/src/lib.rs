use std::collections::{HashMap, HashSet, VecDeque};

use util::{Direction8, Pos};

type DataType = HashMap<Pos<i32>, char>;
type OutType = usize;

pub fn parse(data: &str) -> DataType {
    let mut points = HashMap::new();

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            points.insert(Pos::new(x as i32, y as i32), c);
        }
    }

    points
}

pub fn search_xmas(
    grid: &DataType,
    point: &Pos<i32>,
    direction: &Direction8,
    mut remaining: VecDeque<char>,
) -> bool {
    if remaining.is_empty() {
        return true;
    }

    let next = remaining.pop_front().unwrap();
    let char = grid.get(point).unwrap_or(&'.');

    if *char != next {
        return false;
    }

    let next_pos = point.translate8(direction).unwrap();

    search_xmas(grid, &next_pos, direction, remaining)
}

const MAS: [char; 3] = ['M', 'A', 'S'];

fn collect_mas(grid: &DataType, directions: [&Pos<i32>; 3]) -> HashSet<char> {
    directions
        .into_iter()
        .filter_map(|p| grid.get(p))
        .filter(|c| MAS.contains(c))
        .copied()
        .collect()
}

fn search_mas(grid: &DataType, point: &Pos<i32>) -> bool {
    if grid.get(point) != Some(&'A') {
        return false;
    }
    let nw = point.translate8(&Direction8::NW).unwrap();
    let ne = point.translate8(&Direction8::NE).unwrap();
    let sw = point.translate8(&Direction8::SW).unwrap();
    let se = point.translate8(&Direction8::SE).unwrap();
    let diag1 = collect_mas(grid, [&nw, point, &se]);
    let diag2 = collect_mas(grid, [&ne, point, &sw]);

    diag1.len() == 3 && diag2.len() == 3
}

pub fn part1(grid: DataType) -> OutType {
    let xmas: VecDeque<char> = vec!['X', 'M', 'A', 'S'].into();
    grid.keys()
        .flat_map(|p| Direction8::all().map(move |d| (p, d)))
        .filter(|(p, d)| search_xmas(&grid, p, d, xmas.clone()))
        .count()
}

pub fn part2(grid: DataType) -> OutType {
    grid.keys().filter(|p| search_mas(&grid, p)).count()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#,
        1,
        18
    );

    generate_test!(
        r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#,
        2,
        9
    );

    generate_test! { 2024, 4, 1, 2599}
    generate_test! { 2024, 4, 2, 1948}
}
