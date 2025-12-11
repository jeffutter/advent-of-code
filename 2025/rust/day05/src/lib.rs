use std::ops::Range;

use parser::{dig_pair, FromDig};
use winnow::{ModalResult, Parser, ascii::newline, combinator::separated, token::rest};

type InputType = (Vec<Range<usize>>, Vec<usize>);
type OutType = usize;

#[derive(Debug)]
struct Ranges {
    ranges: Vec<Range<usize>>,
}

impl Ranges {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn insert(&mut self, range: Range<usize>) {
        // Find all ranges that intersect with the new range and merge them
        let mut merged_range = range;
        let mut new_ranges = Vec::new();

        for existing in self.ranges.drain(..) {
            if range_intersects(&existing, &merged_range) {
                merged_range = merge_range(&existing, &merged_range);
            } else {
                new_ranges.push(existing);
            }
        }

        new_ranges.push(merged_range);
        self.ranges = new_ranges;
    }

    fn contains(&self, u: &usize) -> bool {
        self.ranges.iter().any(|r| r.contains(u))
    }

    fn size(&self) -> usize {
        self.ranges.iter().map(|r| r.len()).sum()
    }
}

fn collect_ranges(ranges: Vec<Range<usize>>) -> Ranges {
    ranges.into_iter().fold(Ranges::new(), |mut r, i| {
        r.insert(i);
        r
    })
}

fn range_intersects(r1: &Range<usize>, r2: &Range<usize>) -> bool {
    r1.contains(&r2.start) || r1.contains(&r2.end) || r2.contains(&r1.start) || r2.contains(&r1.end)
}

fn merge_range(r1: &Range<usize>, r2: &Range<usize>) -> Range<usize> {
    (r1.start.min(r2.start))..(r1.end.max(r2.end))
}

fn parse_range(s: &mut &str) -> ModalResult<Range<usize>> {
    dig_pair("-").map(|(l, r)| l..(r + 1)).parse_next(s)
}

fn parse_ranges(s: &mut &str) -> ModalResult<Vec<Range<usize>>> {
    separated(0.., parse_range, newline).parse_next(s)
}

fn parse_numbers(s: &mut &str) -> ModalResult<Vec<usize>> {
    separated(0.., <usize as FromDig>::from_dig, newline).parse_next(s)
}

pub fn parse(data: &str) -> InputType {
    (
        parse_ranges,
        newline,
        newline,
        parse_numbers,
        rest,
    )
        .map(|(fresh, _, _, available, _)| (fresh, available))
        .parse(data)
        .unwrap()
}

pub fn part1((fresh, available): InputType) -> OutType {
    let rs = collect_ranges(fresh);

    available.iter().filter(|a| rs.contains(a)).count()
}

pub fn part2((fresh, _): InputType) -> OutType {
    collect_ranges(fresh).size()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    generate_test!(SAMPLE_INPUT, 1, 3);

    generate_test!(SAMPLE_INPUT, 2, 14);

    generate_test! { 2025, 5, 1, 652}
    generate_test! { 2025, 5, 2, 341753674214273}
}
