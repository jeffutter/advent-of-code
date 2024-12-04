type InputType = Box<dyn Iterator<Item = Vec<i32>>>;
type OutType = usize;

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    Box::new(vec![].into_iter())
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    1
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    1
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"
        "#,
        1,
        0
    );

    generate_test!(
        r#"
        "#,
        2,
        0
    );

    generate_test! { 2024, 5, 1, 0}
    generate_test! { 2024, 5, 2, 0}
}
