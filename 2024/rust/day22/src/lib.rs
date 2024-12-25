use std::{collections::HashMap, iter};

use itertools::iterate;
use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::iter::{ParallelBridge, ParallelIterator};

type InputType<'a> = Box<dyn Iterator<Item = i64> + 'a + Send>;
type OutType = i64;

#[allow(unused_variables)]
pub fn parse(data: &'_ str) -> InputType<'_> {
    Box::new(data.lines().map(|l| l.parse().unwrap()))
}

fn step(secret: &i64) -> i64 {
    let mut secret = (secret ^ secret << 6) % 16777216;
    secret = (secret ^ secret >> 5) % 16777216;
    (secret ^ secret << 11) % 16777216
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    input.map(|p| iterate(p, step).nth(2000).unwrap()).sum()
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    *input
        .par_bridge()
        .flat_map(|n| n_sequences(n, 2000))
        .fold(HashMap::new, |mut counts, (k, v)| {
            *counts.entry(k).or_insert(0) += v;
            counts
        })
        .reduce(HashMap::new, |mut a, b| {
            for (k, v) in b {
                *a.entry(k).or_insert(0) += v;
            }
            a
        })
        .values()
        .max()
        .unwrap()
}

fn n_sequences(secret: i64, n: i64) -> impl ParallelIterator<Item = ((i64, i64, i64, i64), i64)> {
    let mut state = secret;

    let prices = iter::from_fn(move || {
        let prev = state;
        state = step(&state);
        Some(prev % 10)
    })
    .take(n as usize);

    prices
        .clone()
        .tuple_windows()
        .map(|(w1, w2)| w1 - w2)
        .tuple_windows::<(_, _, _, _)>()
        .zip(prices.skip(4))
        .unique_by(|(k, _v)| *k)
        .par_bridge()
        .into_par_iter()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"1
10
100
2024"#,
        1,
        37327623
    );

    generate_test!(
        r#"1
2
3
2024"#,
        2,
        23
    );

    generate_test! { 2024, 22, 1, 20068964552}
    generate_test! { 2024, 22, 2, 2246}
}
