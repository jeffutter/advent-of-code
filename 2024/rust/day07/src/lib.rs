use itertools::Itertools;

type InputType = Vec<(i64, Vec<i64>)>;
type OutType = i64;

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    data.lines()
        .map(|l| {
            let mut parts = l.split(":");
            let result = parts.next().unwrap().trim().parse().unwrap();
            let values = parts
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|v| v.trim().parse().unwrap())
                .collect();
            (result, values)
        })
        .collect()
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
    Concat,
}

fn possibly_true(result: i64, values: Vec<i64>, ops: &[Op]) -> bool {
    values
        .windows(2)
        .map(|_| ops)
        .multi_cartesian_product()
        .any(|ops| {
            let mut ops = ops.iter();
            let mut values = values.iter();
            let mut acc = *values.next().unwrap();

            for v in values {
                match ops.next().unwrap() {
                    Op::Add => acc += v,
                    Op::Mul => acc *= v,
                    Op::Concat => {
                        acc = [acc.to_string(), v.to_string()]
                            .join("")
                            .parse::<i64>()
                            .unwrap()
                    }
                }
            }

            acc == result
        })
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    input
        .iter()
        .filter(|(result, values)| possibly_true(*result, values.to_vec(), &[Op::Add, Op::Mul]))
        .map(|(result, values)| result)
        .sum()
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    input
        .iter()
        .filter(|(result, values)| {
            possibly_true(*result, values.to_vec(), &[Op::Add, Op::Mul, Op::Concat])
        })
        .map(|(result, values)| result)
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#,
        1,
        3749
    );

    generate_test!(
        r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#,
        2,
        11387
    );

    generate_test! { 2024, 7, 1, 6231007345478}
    generate_test! { 2024, 7, 2, 333027885676693}
}
