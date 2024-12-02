pub fn parse(data: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    data.lines().map(|l| {
        l.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect()
    })
}

pub fn part1(_input: impl Iterator<Item = Vec<i32>>) -> usize {
    1
}

pub fn part2(_input: impl Iterator<Item = Vec<i32>>) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test! { 2024, 3, 1, 0}
    generate_test! { 2024, 3, 2, 0}
}
