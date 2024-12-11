use std::{collections::HashMap, ops::Rem};

type InputType = Vec<u64>;
type OutType = usize;

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    data.trim()
        .split(" ")
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn how_many_stones(x: u64, times: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if let Some(res) = cache.get(&(x, times)) {
        return *res;
    }

    if times == 0 {
        return 1;
    }

    let res = match x {
        0 => how_many_stones(1, times - 1, cache),
        x if x.to_string().len().rem(2) == 0 => {
            let s = x.to_string();
            let (left, right) = s.split_at(s.len() / 2);
            how_many_stones(left.parse().unwrap(), times - 1, cache)
                + how_many_stones(right.parse().unwrap(), times - 1, cache)
        }
        x => how_many_stones(x * 2024, times - 1, cache),
    };

    cache.insert((x, times), res);

    res
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    let mut cache = HashMap::new();
    input
        .into_iter()
        .map(|x| how_many_stones(x, 25, &mut cache))
        .sum()
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    let mut cache = HashMap::new();
    input
        .into_iter()
        .map(|x| how_many_stones(x, 75, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(r#"125 17"#, 1, 55312);

    generate_test! { 2024, 11, 1, 222461}
    generate_test! { 2024, 11, 2, 264350935776416}
}
