pub fn part1<'a>(grid: Grid) -> i32 {
    1
}

pub fn part2<'a>(grd: Grid) -> i32 {
    1
}

pub fn parse<'a>(data: &'a str) -> Grid {
    Grid::new()
}

pub struct Grid {
    data: Vec<i32>,
}

impl Grid {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
