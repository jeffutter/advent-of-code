use regex::Regex;

type RowType = Op;
type OutType = i32;

#[derive(Debug)]
pub enum Op {
    Do,
    Dont,
    Mul(i32, i32),
}

pub fn parse(data: &str) -> impl Iterator<Item = RowType> + '_ {
    let re = Regex::new(
        r"(?:(?<do>do\(\)?)|(?<dont>don't\(\))|(?<mul>mul\((?<n1>\d{1,3}),(?<n2>\d{1,3})\)))",
    )
    .unwrap();
    re.captures_iter(data)
        .map(|c| {
            if c.name("do").is_some() {
                return Op::Do;
            }
            if c.name("dont").is_some() {
                return Op::Dont;
            }

            let n1 = c.name("n1").unwrap().as_str().parse().unwrap();
            let n2 = c.name("n2").unwrap().as_str().parse().unwrap();
            Op::Mul(n1, n2)
        })
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn part1(input: impl Iterator<Item = RowType>) -> OutType {
    input
        .map(|op| match op {
            Op::Do => 0,
            Op::Dont => 0,
            Op::Mul(a, b) => a * b,
        })
        .sum()
}

pub fn part2(input: impl Iterator<Item = RowType>) -> OutType {
    input
        .fold((true, 0), |(emit, acc), op| match (emit, op) {
            (_, Op::Do) => (true, acc),
            (_, Op::Dont) => (false, acc),
            (true, Op::Mul(a, b)) => (true, (a * b) + acc),
            (false, _) => (false, acc),
        })
        .1
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        1,
        161
    );

    generate_test!(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        2,
        48
    );

    generate_test! { 2024, 3, 1, 161289189}
    generate_test! { 2024, 3, 2, 83595109}
}
