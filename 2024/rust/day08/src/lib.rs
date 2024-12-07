type InputType = usize;
type OutType = usize;

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    1
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    input
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    input
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(r#""#, 1, 0);

    generate_test!(r#""#, 2, 0);

    generate_test! { 2024, 8, 1, 0}
    generate_test! { 2024, 8, 2, 0}
}
