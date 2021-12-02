pub fn part1(data: String) -> i32 {
    let (x, y) = data
        .lines()
        .map(|line| line.split_whitespace().collect())
        .fold((0, 0), |(x, y), line: Vec<&str>| match line.as_slice() {
            ["forward", n] => (x + n.parse::<i32>().unwrap(), y),
            ["down", n] => (x, y + n.parse::<i32>().unwrap()),
            ["up", n] => (x, y - n.parse::<i32>().unwrap()),
            _ => (x, y),
        });

    x * y
}

pub fn part2(data: String) -> i32 {
    let (x, y, _a) = data
        .lines()
        .map(|line| line.split_whitespace().collect())
        .fold((0, 0, 0), |(x, y, a), line: Vec<&str>| {
            match line.as_slice() {
                ["down", n] => (x, y, a + n.parse::<i32>().unwrap()),
                ["up", n] => (x, y, a - n.parse::<i32>().unwrap()),
                ["forward", n] => {
                    let n = n.parse::<i32>().unwrap();
                    (x + n, y + (n * a), a)
                }
                _ => (x, y, a),
            }
        });

    x * y
}
