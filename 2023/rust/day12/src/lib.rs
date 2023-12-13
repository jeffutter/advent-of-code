use std::collections::HashMap;

use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl std::fmt::Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
        }
    }
}

pub struct Row {
    id: usize,
    springs: Vec<Spring>,
    groups: Vec<u32>,
}

fn debug_fmt_springs(springs: &[Spring]) -> String {
    springs.iter().map(|x| format!("{:?}", x)).join("")
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Row")
            .field("id", &self.id)
            .field("springs", &debug_fmt_springs(&self.springs))
            .field("groups", &self.groups)
            .finish()
    }
}

impl Row {
    fn count_ways(&self) -> (usize, Vec<Vec<Spring>>) {
        let (count, solutions) = count_ways(&self.springs, &self.groups);
        // println!(
        //     "Row: {:?} - {}: \n{}",
        //     &self,
        //     count,
        //     solutions.iter().map(|w| debug_fmt_springs(w)).join("\n")
        // );
        (count, solutions)
    }
}

fn count_ways(springs: &[Spring], groups: &[u32]) -> (usize, Vec<Vec<Spring>>) {
    do_count_ways(springs, groups, false, &[])
}

fn do_count_ways(
    remaining_springs: &[Spring],
    remaining_groups: &[u32],
    prev_complete: bool,
    visited: &[Spring],
) -> (usize, Vec<Vec<Spring>>) {
    // println!(
    //     "CountWays: {:?}, {:?}, {:?}",
    //     debug_fmt_springs(remaining_springs),
    //     remaining_groups,
    //     prev_complete,
    // );

    // No more groups remaining and no more damaged springs. Done. Success.
    if remaining_groups.len() == 0
        && remaining_springs
            .iter()
            .all(|s| *s == Spring::Operational || *s == Spring::Unknown)
    {
        return (1, [visited.to_vec()].to_vec());
    }

    // No more groups remaining springs, no more remaining groups, remaining springs aren't enough
    // to complete remaining group. Done. Fail.
    if remaining_springs.len() == 0
        || remaining_groups.len() == 0
        || remaining_springs.len() < (remaining_groups[0] as usize)
    {
        return (0, Vec::new());
    }

    // Just completed a group and next spring is Damaged. Done. Fail.
    if prev_complete && remaining_springs[0] == Spring::Damaged {
        return (0, Vec::new());
    }

    // Next spring is Operational or next spring is Unknown and we just
    // finished a damaged block. Skip it. Continue.
    if remaining_springs[0] == Spring::Operational
        || (prev_complete && remaining_springs[0] == Spring::Unknown)
    {
        return do_count_ways(
            &remaining_springs[1..],
            remaining_groups,
            false,
            &[&[Spring::Operational], visited].concat(),
        );
    }

    // Split off a set of springs matching the size of the next group
    let group_len = remaining_groups[0] as usize;
    let (next_group, rest) = remaining_springs.split_at(group_len);

    // If all of the springs are Damaged or Unknown
    if next_group
        .iter()
        .all(|s| *s == Spring::Damaged || *s == Spring::Unknown)
    {
        // Take one path where they were all Damaged, thus completing a group
        let (count_a, solutions_a) = do_count_ways(
            rest,
            &remaining_groups[1..],
            true,
            &[&[Spring::Damaged].repeat(group_len), visited].concat(),
        );

        let (next, rest) = remaining_springs.split_first().unwrap();

        let (count_b, solutions_b) = match next {
            Spring::Operational => unreachable!(),
            Spring::Damaged => (0, Vec::new()),
            // Take another path where the first one was Operational
            Spring::Unknown => do_count_ways(
                rest,
                remaining_groups,
                false,
                &[&[Spring::Operational], visited].concat(),
            ),
        };

        // Combine results of both codepaths
        return (count_a + count_b, [solutions_a, solutions_b].concat());
    }

    // All other cases failed, treat this unknown as operational and skip it
    let (next, rest) = remaining_springs.split_first().unwrap();
    match next {
        Spring::Operational => unreachable!(),
        Spring::Damaged => (0, Vec::new()),
        Spring::Unknown => do_count_ways(
            rest,
            remaining_groups,
            prev_complete,
            &[&[Spring::Operational], visited].concat(),
        ),
    }
}

pub struct Rows {
    rows: Vec<Row>,
}

impl Rows {
    fn count_ways(&self) -> usize {
        self.rows.iter().map(|row| row.count_ways().0).sum()
    }

    fn unfold(&self) -> Self {
        let new_rows = self
            .rows
            .iter()
            .map(|row| {
                let new_springs = Itertools::intersperse(
                    std::iter::repeat(row.springs.clone()),
                    vec![Spring::Unknown],
                )
                .take(9)
                .flatten()
                .collect_vec();

                Row {
                    id: row.id,
                    springs: new_springs,
                    groups: row.groups.repeat(5),
                }
            })
            .collect_vec();

        Rows { rows: new_rows }
    }
}

pub fn parse<'a>(data: &'a str) -> Rows {
    let mut rows = Vec::new();

    for (id, line) in data.lines().enumerate() {
        let mut springs = Vec::new();
        let (spring_str, groups) = line.split_whitespace().collect_tuple().unwrap();
        for c in spring_str.chars() {
            let spring = match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => unimplemented!(),
            };

            springs.push(spring);
        }

        let groups: Vec<u32> = groups.split(",").map(|s| s.parse().unwrap()).collect_vec();

        rows.push(Row {
            id,
            springs,
            groups,
        });
    }

    Rows { rows }
}

pub fn part1<'a>(rows: Rows) -> usize {
    rows.count_ways()
}

pub fn part2<'a>(rows: Rows) -> usize {
    rows.unfold().count_ways()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_sample_1a() {
        let data = parse("???.### 1,1,3");
        let x = data.rows.first().unwrap();
        let (count, solutions) = count_ways(&x.springs, &x.groups);
        println!(
            "Solutions:\n{}",
            solutions.iter().map(|w| debug_fmt_springs(w)).join("\n")
        );
        assert_eq!(count, 1);
    }

    #[test]
    fn test_sample_1b() {
        let data = parse(".??..??...?##. 1,1,3");
        let x = data.rows.first().unwrap();
        let (count, solutions) = count_ways(&x.springs, &x.groups);
        println!(
            "Solutions:\n{}",
            solutions.iter().map(|w| debug_fmt_springs(w)).join("\n")
        );
        assert_eq!(count, 4);
    }

    #[test]
    fn test_sample_1g() {
        let data = parse("?.????##??.?#???. 2,3");
        let x = data.rows.first().unwrap();
        let (count, solutions) = count_ways(&x.springs, &x.groups);
        println!(
            "Solutions:\n{}",
            solutions.iter().map(|w| debug_fmt_springs(w)).join("\n")
        );
        assert_eq!(count, 2);
    }

    #[test]
    fn test_sample_1h() {
        let data = parse("?#.# 2,1");
        let x = data.rows.first().unwrap();
        let (count, solutions) = count_ways(&x.springs, &x.groups);
        println!(
            "Solutions:\n{}",
            solutions.iter().map(|w| debug_fmt_springs(w)).join("\n")
        );
        assert_eq!(count, 1);
    }

    #[test]
    fn test_sample_1i() {
        let data = parse("#??# 2");
        let x = data.rows.first().unwrap();
        let (count, solutions) = count_ways(&x.springs, &x.groups);
        println!(
            "Solutions:\n{}",
            solutions.iter().map(|w| debug_fmt_springs(w)).join("\n")
        );
        assert_eq!(count, 0);
    }

    #[test]
    fn test_sample_1j() {
        let data = parse(".??#?#. 3");
        let x = data.rows.first().unwrap();
        let (count, solutions) = count_ways(&x.springs, &x.groups);
        println!(
            "Solutions:\n{}",
            solutions.iter().map(|w| debug_fmt_springs(w)).join("\n")
        );
        assert_eq!(count, 1);
    }

    #[test]
    fn test_sample_1k() {
        let data = parse("???# 1");
        let x = data.rows.first().unwrap();
        let (count, solutions) = count_ways(&x.springs, &x.groups);
        println!(
            "Solutions:\n{}",
            solutions.iter().map(|w| debug_fmt_springs(w)).join("\n")
        );
        assert_eq!(count, 1);
    }

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 21);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 525152);
    }

    #[test]
    fn test_sample_2a() {
        let data = parse("???.### 1,1,3").unfold();
        let x = data.rows.first().unwrap();
        let (count, solutions) = count_ways(&x.springs, &x.groups);
        println!(
            "Solutions:\n{}",
            solutions.iter().map(|w| debug_fmt_springs(w)).join("\n")
        );
        assert_eq!(count, 1);
    }

    #[test]
    fn test_sample_2b() {
        let data = parse(".??..??...?##. 1,1,3").unfold();
        let x = data.rows.first().unwrap();
        let (count, solutions) = count_ways(&x.springs, &x.groups);
        println!(
            "Solutions:\n{}",
            solutions.iter().map(|w| debug_fmt_springs(w)).join("\n")
        );
        assert_eq!(count, solutions.iter().unique().count());
        assert_eq!(count, 16384);
    }

    generate_test! { 2023, 12, 1, 7344}
    generate_test! { 2023, 12, 2, 0}
}
