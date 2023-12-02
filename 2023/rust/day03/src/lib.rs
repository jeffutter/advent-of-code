use std::str::Lines;

pub fn parse<'a>(data: &'a str) -> Lines<'a> {
    data.lines()
}

pub fn part1<'a>(_input: impl Iterator<Item = &'a str>) -> i32 {
    0
}

pub fn part2<'a>(_input: impl Iterator<Item = &'a str>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test! { 2023, 3, 1, 0}
    generate_test! { 2023, 3, 2, 0}
}
