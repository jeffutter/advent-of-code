type InputType = usize;
type OutType = usize;

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    0
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    input;
    1
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    input;
    1
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const TEST_INPUT: &str = r#""#;

    generate_test!(TEST_INPUT, 1, 0);

    generate_test!(TEST_INPUT, 2, 0);

    generate_test! { 2025, 9, 1, 0}
    generate_test! { 2025, 9, 2, 0}
}
