use itertools::Itertools;

type InputType = Vec<Vec<usize>>;
type OutType = usize;

pub fn parse(data: &str) -> InputType {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn part1(input: InputType) -> OutType {
    window_max(input, 2)
}

pub fn part2(input: InputType) -> OutType {
    window_max(input, 12)
}

fn vec2usize(v: &[usize]) -> usize {
    v.iter()
        .rev()
        .enumerate()
        .map(|(idx, n)| n * (10usize.pow(idx as u32)))
        .sum::<usize>()
}

fn window_max(input: InputType, search_size: usize) -> OutType {
    input
        .iter()
        .map(|line| {
            let mut acc = Vec::new();
            let mut l = 0;
            let mut r = line.len() - (search_size - 1);

            while l <= line.len() && acc.len() < search_size {
                let window = line[l..r].to_vec();
                // Invert so we can use `position_min` which returns the index of the first match, not
                // the last like `position_max`
                let next_max = window.iter().map(|x| -(*x as i32)).position_min().unwrap();

                acc.push(window[next_max]);

                l = l + next_max + 1;
                r += 1
            }

            vec2usize(&acc)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const TEST_INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    generate_test!(TEST_INPUT, 1, 357);

    generate_test!(TEST_INPUT, 2, 3121910778619);

    generate_test! { 2025, 3, 1, 17193}
    generate_test! { 2025, 3, 2, 171297349921310}
}
