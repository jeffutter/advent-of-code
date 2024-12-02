use std::collections::HashMap;

use itertools::izip;

pub fn parse(data: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    data.lines().map(|l| {
        let mut ns = l.split_whitespace();
        let l: i32 = ns.next().unwrap().parse().unwrap();
        let r: i32 = ns.next().unwrap().parse().unwrap();
        (l, r)
    })
}

pub fn part1(input: impl Iterator<Item = (i32, i32)>) -> i32 {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input.unzip();

    left.sort();
    right.sort();

    izip!(left, right).map(|(l, r)| (l - r).abs()).sum()
}

pub fn part2(input: impl Iterator<Item = (i32, i32)>) -> i32 {
    let (left, right) = input.fold(
        (Vec::new(), HashMap::new()),
        |(mut left, mut right), (l, r)| {
            left.push(l);
            right.entry(r).and_modify(|e| *e += 1).or_insert(1);
            (left, right)
        },
    );

    left.iter().map(|l| l * right.get(l).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test! { 2024, 1, 1, 2164381}
    generate_test! { 2024, 1, 2, 20719933}
}
