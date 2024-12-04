type RowType = i32;
type OutType = i32;

pub fn parse(data: &str) -> impl Iterator<Item = RowType> + '_ {
    vec![].into_iter()
}

pub fn part1(_input: impl Iterator<Item = RowType>) -> OutType {
    1
}

pub fn part2(input: impl Iterator<Item = RowType>) -> OutType {
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

    generate_test! { 2024, 4, 1, 0}
    generate_test! { 2024, 4, 2, 0}
}
