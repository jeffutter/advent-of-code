use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::{DefaultHasher, Hasher},
    iter,
    sync::LazyLock,
};

use itertools::Itertools;

type InputType = Vec<Vec<char>>;
type OutType = usize;

static KEYPAD_SUCCESSORS: LazyLock<HashMap<char, Vec<(char, char)>>> = LazyLock::new(|| {
    vec![
        ('7', vec![('4', 'v'), ('8', '>')]),
        ('8', vec![('5', 'v'), ('9', '>'), ('7', '<')]),
        ('9', vec![('6', 'v'), ('8', '<')]),
        ('4', vec![('1', 'v'), ('5', '>'), ('7', '^')]),
        ('5', vec![('2', 'v'), ('6', '>'), ('4', '<'), ('8', '^')]),
        ('6', vec![('3', 'v'), ('5', '<'), ('9', '^')]),
        ('1', vec![('2', '>'), ('4', '^')]),
        ('2', vec![('3', '>'), ('1', '<'), ('5', '^'), ('0', 'v')]),
        ('3', vec![('2', '<'), ('6', '^'), ('A', 'v')]),
        ('0', vec![('A', '>'), ('2', '^')]),
        ('A', vec![('0', '<'), ('3', '^')]),
    ]
    .into_iter()
    .collect()
});

static DPAD_SUCCESSORS: LazyLock<HashMap<char, Vec<(char, char)>>> = LazyLock::new(|| {
    vec![
        ('^', vec![('v', 'v'), ('A', '>')]),
        ('A', vec![('>', 'v'), ('^', '<')]),
        ('<', vec![('v', '>')]),
        ('v', vec![('<', '<'), ('^', '^'), ('>', '>')]),
        ('>', vec![('v', '<'), ('A', '^')]),
    ]
    .into_iter()
    .collect()
});

type PadPaths = LazyLock<HashMap<(char, char), Vec<Vec<char>>>>;

static KEYPAD_PATHS: PadPaths = LazyLock::new(|| {
    KEYPAD_SUCCESSORS
        .keys()
        .cartesian_product(KEYPAD_SUCCESSORS.keys())
        .map(|(&from, &to)| {
            (
                (from, to),
                find_shortest_paths(&KEYPAD_SUCCESSORS, from, to),
            )
        })
        .collect()
});

static DPAD_PATHS: PadPaths = LazyLock::new(|| {
    DPAD_SUCCESSORS
        .keys()
        .cartesian_product(DPAD_SUCCESSORS.keys())
        .map(|(&from, &to)| ((from, to), find_shortest_paths(&DPAD_SUCCESSORS, from, to)))
        .collect()
});

fn find_shortest_paths(
    pad: &HashMap<char, Vec<(char, char)>>,
    from: char,
    to: char,
) -> Vec<Vec<char>> {
    let mut queue = VecDeque::new();
    queue.push_back((from, Vec::new(), HashSet::new()));

    let mut paths = Vec::new();
    let mut shortest = usize::MAX;

    while let Some((node, path, mut visited)) = queue.pop_front() {
        if node == to {
            if path.len() < shortest {
                paths = Vec::new();
            }

            if path.len() <= shortest {
                shortest = path.len();
                paths.push(path);
            }
            continue;
        }

        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        for (next, dir) in pad.get(&node).unwrap() {
            let mut path = path.clone();
            path.push(*dir);
            queue.push_back((*next, path, visited.clone()));
        }
    }

    paths
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    data.lines().map(|l| l.chars().collect()).collect()
}

fn find_shortest_sequence<'a>(
    sequence: impl Iterator<Item = &'a char> + Clone,
    depth: usize,
    number: bool,
    cache: &mut HashMap<(u64, usize, bool), usize>,
) -> usize {
    let mut vec_seq = DefaultHasher::new();
    for x in sequence.clone() {
        vec_seq.write_u8(*x as u8);
    }
    let vec_seq = vec_seq.finish();

    if let Some(v) = cache.get(&(vec_seq, depth, number)) {
        return *v;
    }

    let paths = if number { &KEYPAD_PATHS } else { &DPAD_PATHS };

    let res = iter::once(&'A')
        .chain(sequence)
        .tuple_windows()
        .map(|(&a, &b)| {
            let shortest_paths = paths.get(&(a, b)).unwrap();

            match depth {
                0 => shortest_paths[0].len() + 1,
                _ => shortest_paths
                    .iter()
                    .map(|path| {
                        find_shortest_sequence(
                            path.iter().chain(iter::once(&'A')),
                            depth - 1,
                            false,
                            cache,
                        )
                    })
                    .min()
                    .unwrap(),
            }
        })
        .sum::<usize>();

    cache.insert((vec_seq, depth, number), res);

    res
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    let mut cache = HashMap::new();
    input
        .iter()
        .map(|line| {
            find_shortest_sequence(line.iter(), 2, true, &mut cache)
                * line
                    .iter()
                    .collect::<String>()
                    .trim_end_matches('A')
                    .parse::<usize>()
                    .unwrap()
        })
        .sum()
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    let mut cache = HashMap::new();
    input
        .iter()
        .map(|line| {
            find_shortest_sequence(line.iter(), 25, true, &mut cache)
                * line
                    .iter()
                    .collect::<String>()
                    .trim_end_matches('A')
                    .parse::<usize>()
                    .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"029A
980A
179A
456A
379A"#,
        1,
        126384
    );

    generate_test! { 2024, 21, 1, 203734}
    generate_test! { 2024, 21, 2, 246810588779586}
}
