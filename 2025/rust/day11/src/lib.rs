use std::collections::HashMap;
use std::hash::Hash;

use winnow::{
    ModalResult, Parser,
    ascii::{alpha1, line_ending, space1},
    combinator::{repeat, separated, terminated},
};

type InputType<'a> = HashMap<&'a str, Vec<&'a str>>;
type OutType = usize;

fn parse_line<'a>(data: &mut &'a str) -> ModalResult<(&'a str, Vec<&'a str>)> {
    (terminated(alpha1, ": "), separated(1.., alpha1, space1)).parse_next(data)
}

pub fn parse(data: &str) -> InputType<'_> {
    let (x, _): (Vec<(&str, Vec<&str>)>, Vec<_>) = (
        separated(1.., parse_line, line_ending),
        repeat(0.., line_ending),
    )
        .parse(data)
        .unwrap();

    x.into_iter().collect()
}

fn count_paths<N, FN, IN, FS>(start: N, mut successors: FN, mut is_goal: FS) -> usize
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    fn count_paths<N, FN, IN, FS>(
        node: &N,
        successors: &mut FN,
        is_goal: &mut FS,
        cache: &mut HashMap<N, usize>,
    ) -> usize
    where
        N: Eq + Hash + Clone,
        FN: FnMut(&N) -> IN,
        IN: IntoIterator<Item = N>,
        FS: FnMut(&N) -> bool,
    {
        if is_goal(node) {
            return 1;
        }

        if let Some(&count) = cache.get(node) {
            return count;
        }

        let total_paths: usize = successors(node)
            .into_iter()
            .map(|next| count_paths(&next, successors, is_goal, cache))
            .sum();

        cache
            .insert(node.clone(), total_paths)
            .unwrap_or(total_paths)
    }

    let mut cache: HashMap<N, usize> = HashMap::new();
    count_paths(&start, &mut successors, &mut is_goal, &mut cache)
}

pub fn part1(input: InputType) -> OutType {
    count_paths(
        "you",
        |&device| input.get(device).unwrap().iter().copied(),
        |&device| device == "out",
    )
}

pub fn part2(input: InputType) -> OutType {
    static EMPTY_STATE: Vec<(&str, bool, bool)> = Vec::new();

    let mut expanded: HashMap<(&str, bool, bool), Vec<(&str, bool, bool)>> = HashMap::new();

    for &node in input.keys() {
        for seen_dac in [false, true] {
            for seen_fft in [false, true] {
                let state = (node, seen_dac, seen_fft);

                let successors: Vec<_> = input
                    .get(node)
                    .unwrap()
                    .iter()
                    .map(|&next| (next, seen_dac || next == "dac", seen_fft || next == "fft"))
                    .collect();

                expanded.insert(state, successors);
            }
        }
    }

    count_paths(
        ("svr", false, false),
        |&state| expanded.get(&state).unwrap_or(&EMPTY_STATE).iter().copied(),
        |&(node, seen_dac, seen_fft)| node == "out" && seen_dac && seen_fft,
    )
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const TEST_INPUT1: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    const TEST_INPUT2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

    generate_test!(TEST_INPUT1, 1, 5);

    generate_test!(TEST_INPUT2, 2, 2);

    generate_test! { 2025, 11, 1, 428}
    generate_test! { 2025, 11, 2, 331468292364745}
}
