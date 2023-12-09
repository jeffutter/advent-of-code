use itertools::Itertools;

pub fn parse<'a>(data: &'a str) -> Vec<Vec<i32>> {
    data.lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

pub fn part1<'a>(histories: Vec<Vec<i32>>) -> i32 {
    fold_histories(histories, 0, &|acc, history| acc + history.last().unwrap())
}

pub fn part2<'a>(histories: Vec<Vec<i32>>) -> i32 {
    fold_histories(histories, 0, &|acc, history| history.first().unwrap() - acc)
}

fn fold_histories(histories: Vec<Vec<i32>>, init: i32, f: &dyn Fn(i32, &Vec<i32>) -> i32) -> i32 {
    histories
        .into_iter()
        .map(|history| build_histories(history).iter().rev().skip(1).fold(init, f))
        .sum()
}

fn build_histories(start: Vec<i32>) -> Vec<Vec<i32>> {
    let mut histories: Vec<Vec<i32>> = Vec::new();
    let mut next: Vec<i32> = start;

    loop {
        histories.push(next.clone());

        if next.iter().all(|v| *v == 0) {
            break;
        }

        next = next
            .iter()
            .zip(next.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect_vec();
    }

    histories
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 114);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 2);
    }

    generate_test! { 2023, 9, 1, 1681758908}
    generate_test! { 2023, 9, 2, 803}
}
