pub fn part1(state: Vec<i64>) -> i64 {
    1
}

pub fn part2(state: Vec<i64>) -> i64 {
    1
}

pub fn parse<'a>(data: &'a str) -> Vec<i64> {
    data.lines().map(|f| f.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {

    use super::*;
}
