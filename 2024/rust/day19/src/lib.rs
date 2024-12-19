use std::collections::{HashMap, HashSet};

type InputType<'a> = (HashSet<&'a str>, Vec<&'a str>);
type OutType = usize;

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let mut data = data.lines();
    let available = data
        .next()
        .unwrap()
        .split(",")
        .map(|a| a.trim())
        .collect::<HashSet<_>>();
    let needed = data
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    (available, needed)
}

fn can_construct<'a>(
    available: &HashSet<&str>,
    cache: &mut HashMap<&'a str, usize>,
    needed: &'a str,
) -> usize {
    if needed.is_empty() {
        return 1;
    }

    if let Some(v) = cache.get(&needed) {
        return *v;
    }

    let n = available
        .iter()
        .filter_map(|a| needed.strip_prefix(a))
        .map(|rest| can_construct(available, cache, rest))
        .sum();

    cache.insert(needed, n);
    n
}

#[allow(unused_variables)]
pub fn part1((available, needed): InputType) -> OutType {
    let mut cache: HashMap<&str, usize> = HashMap::new();
    needed
        .iter()
        .filter(|n| can_construct(&available, &mut cache, n) > 0)
        .count()
}

#[allow(unused_variables)]
pub fn part2((available, needed): InputType) -> OutType {
    let mut cache: HashMap<&str, usize> = HashMap::new();
    needed
        .iter()
        .map(|n| can_construct(&available, &mut cache, n))
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#,
        1,
        6
    );

    generate_test!(
        r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#,
        2,
        16
    );

    generate_test! { 2024, 19, 1, 350}
    generate_test! { 2024, 19, 2, 769668867512623}
}
