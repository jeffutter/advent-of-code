pub fn parse(data: &str) -> impl Iterator<Item = &str> {
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

    generate_test! { 2024, 2, 1, 0}
    generate_test! { 2024, 2, 2, 0}
}
