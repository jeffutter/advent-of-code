use parser::dig;
use winnow::{
    Parser,
    ascii::{line_ending, newline, space0, space1},
    combinator::{delimited, preceded, separated},
    token::{one_of, rest},
};

type InputType<'a> = &'a str;
type OutType = usize;

pub fn number_line(input: &mut &str) -> winnow::error::Result<Vec<usize>> {
    preceded(space0, separated(1.., dig::<usize>, space1)).parse_next(input)
}

pub fn number_lines(input: &mut &str) -> winnow::error::Result<Vec<Vec<usize>>> {
    separated(0.., number_line, line_ending).parse_next(input)
}

pub fn math_line(input: &mut &str) -> winnow::error::Result<Vec<char>> {
    separated(0.., one_of(['+', '*']), space1).parse_next(input)
}

#[allow(unused_variables)]
pub fn parse(data: &'_ str) -> InputType<'_> {
    data
}

#[allow(unused_variables)]
pub fn part1(data: InputType) -> OutType {
    let (rows, _, math, _): (Vec<Vec<usize>>, _, Vec<_>, _) =
        (number_lines, line_ending, math_line, rest)
            .parse(data)
            .unwrap();

    (0..rows[0].len())
        .map(|i| match math[i] {
            '+' => rows.iter().map(|r| r[i]).sum::<usize>(),
            '*' => rows.iter().map(|r| r[i]).product(),
            _ => unimplemented!(),
        })
        .sum()
}

#[allow(unused_variables)]
pub fn part2(data: InputType) -> OutType {
    let lines: Vec<&str> = data.lines().collect();
    let max_width = lines.iter().map(|l| l.len()).max().unwrap();

    // Transpose and reverse in one step
    let columns_rtl: Vec<String> = (0..max_width)
        .rev()
        .map(|col| {
            lines
                .iter()
                .map(|line| line.chars().nth(col).unwrap_or(' '))
                .collect()
        })
        .collect();

    let rtl = columns_rtl.join("\n");

    let (problems, rest): (Vec<(Vec<usize>, char)>, _) = (
        separated(
            1..,
            (
                separated(1.., delimited(space0, dig::<usize>, space0), newline),
                one_of(['*', '+']),
            ),
            (newline, space0, newline),
        ),
        rest,
    )
        .parse(&rtl)
        .unwrap();

    assert_eq!(rest, "");

    problems
        .iter()
        .map(|(nums, op)| match op {
            '+' => nums.iter().sum::<usize>(),
            '*' => nums.iter().product(),
            _ => unimplemented!(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const TEST_INPUT: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +"#;

    generate_test!(TEST_INPUT, 1, 4277556);

    generate_test!(TEST_INPUT, 2, 3263827);

    generate_test! { 2025, 6, 1, 5171061464548}
    generate_test! { 2025, 6, 2, 10189959087258}
}
