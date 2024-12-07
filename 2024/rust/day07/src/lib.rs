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

impl Op {
    fn process(&self, acc: i64, v: i64) -> i64 {
        match self {
            Op::Add => acc + v,
            Op::Mul => acc * v,
            Op::Concat => acc * 10i64.pow(v.ilog10() + 1) + v,
        }
    }
}

fn possibly_true(result: &i64, total: i64, remaining: &[i64], ops: &[Op]) -> bool {
    if remaining.is_empty() {
        return *result == total;
    }

    if total > *result {
        return false;
    }

    for op in ops {
        if possibly_true(
            result,
            op.process(total, remaining[0]),
            &remaining[1..],
            ops,
        ) {
            return true;
        }
    }

    false
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    const OPS: [Op; 2] = [Op::Add, Op::Mul];
    input
        .iter()
        .filter(|(result, values)| possibly_true(result, values[0], &values[1..], &OPS))
        .map(|(result, values)| result)
        .sum()
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    const OPS: [Op; 3] = [Op::Add, Op::Mul, Op::Concat];
    input
        .iter()
        .filter(|(result, values)| possibly_true(result, values[0], &values[1..], &OPS))
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
