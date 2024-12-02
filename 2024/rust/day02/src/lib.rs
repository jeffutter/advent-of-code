use itertools::Itertools;

pub fn parse(data: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    data.lines().map(|l| {
        l.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect()
    })
}

pub fn part1(input: impl Iterator<Item = Vec<i32>>) -> usize {
    input
        .filter(|row| {
            let diffs = row.iter().tuple_windows().map(|(a, b)| a - b).collect_vec();

            (diffs.iter().all(|d| d.is_positive()) || diffs.iter().all(|d| d.is_negative()))
                && diffs.iter().all(good_diff)
        })
        .count()
}

fn good_diff(i: &i32) -> bool {
    (1..=3).contains(&i.abs())
}

pub fn part2(input: impl Iterator<Item = Vec<i32>>) -> usize {
    input
        .filter(|row| {
            (0..row.len()).any(|i| {
                let mut row = row.clone();
                row.remove(i);
                let diffs = row.iter().tuple_windows().map(|(a, b)| a - b).collect_vec();

                (diffs.iter().all(|d| d.is_positive()) || diffs.iter().all(|d| d.is_negative()))
                    && diffs.iter().all(good_diff)
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    #[test]
    fn sample1() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let data = parse(input);
        assert_eq!(part1(data), 2);
    }

    #[test]
    fn sample2() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let data = parse(input);
        assert_eq!(part2(data), 4);
    }

    generate_test! { 2024, 2, 1, 483}
    generate_test! { 2024, 2, 2, 528}
}
