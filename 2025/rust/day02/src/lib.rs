use std::{collections::HashSet, ops::RangeInclusive};

use parser::dig_pair;
use winnow::{Parser, combinator::separated};

type InputType = Vec<RangeInclusive<usize>>;
type OutType = usize;

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    separated(0.., dig_pair::<usize>("-").map(|(l, r)| l..=r), ",")
        .parse(data.trim())
        .unwrap()
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    input
        .iter()
        .map(|range| {
            let mut invalid = 0;
            for num in range.clone() {
                let num_s = num.to_string();
                let bytes = num_s.as_bytes();
                let byte_len = bytes.len();

                if byte_len.rem_euclid(2) != 0 {
                    continue;
                }

                let (lhs, rhs) = bytes.split_at(byte_len.div_euclid(2));

                if lhs == rhs {
                    invalid += num;
                }
            }

            invalid
        })
        .sum()
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    input
        .iter()
        .map(|range| {
            let mut invalid = HashSet::new();

            for num in range.clone() {
                let num_s = num.to_string();
                let bytes = num_s.as_bytes();
                let byte_len = bytes.len();

                for len in 1..=byte_len.div_euclid(2) {
                    if byte_len.rem_euclid(len) != 0 {
                        continue;
                    }

                    for chunk in bytes.windows(len) {
                        if bytes == chunk.repeat(byte_len.div_euclid(len)) {
                            invalid.insert(num);
                        }
                    }
                }
            }

            invalid.iter().sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const TEST_INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    generate_test!(TEST_INPUT, 1, 1227775554);

    generate_test!(TEST_INPUT, 2, 4174379265);

    generate_test! { 2025, 2, 1, 13108371860}
    generate_test! { 2025, 2, 2, 22471660255}
}
