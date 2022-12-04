use std::ops::RangeInclusive;

use nom::{bytes::complete::tag, multi::separated_list0, sequence::tuple, IResult};

pub fn part1(assignment_pairs: impl Iterator<Item = AssignmnetPair>) -> i32 {
    assignment_pairs
        .filter(|pair| pair.full_overlap())
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(assignment_pairs: impl Iterator<Item = AssignmnetPair>) -> i32 {
    assignment_pairs
        .filter(|pair| pair.any_overlap())
        .count()
        .try_into()
        .unwrap()
}

pub struct AssignmnetPair {
    left: RangeInclusive<i32>,
    right: RangeInclusive<i32>,
}

impl AssignmnetPair {
    fn new(lmin: i32, lmax: i32, rmin: i32, rmax: i32) -> Self {
        Self {
            left: (lmin..=lmax),
            right: (rmin..=rmax),
        }
    }

    fn full_overlap(&self) -> bool {
        let left_start = self.left.start();
        let left_end = self.left.end();
        let right_start = self.right.start();
        let right_end = self.right.end();

        (self.left.contains(right_start) && self.left.contains(right_end))
            || (self.right.contains(left_start) && self.right.contains(left_end))
    }

    fn any_overlap(&self) -> bool {
        let left_start = self.left.start();
        let left_end = self.left.end();
        let right_start = self.right.start();
        let right_end = self.right.end();

        self.left.contains(right_start)
            || self.left.contains(right_end)
            || self.right.contains(left_start)
            || self.right.contains(left_end)
    }
}

fn assignment(s: &str) -> IResult<&str, (i32, i32)> {
    let (rest, (min, _, max)) = tuple((parser::from_dig, tag("-"), parser::from_dig))(s)?;
    Ok((rest, (min, max)))
}

fn assignment_pair(s: &str) -> IResult<&str, AssignmnetPair> {
    let (rest, ((lmin, lmax), _, (rmin, rmax))) = tuple((assignment, tag(","), assignment))(s)?;

    Ok((rest, AssignmnetPair::new(lmin, lmax, rmin, rmax)))
}

fn parse_assignment_pairs(s: &str) -> IResult<&str, Vec<AssignmnetPair>> {
    let (rest, assignment_pairs) = separated_list0(tag("\n"), assignment_pair)(s)?;
    Ok((rest, assignment_pairs))
}

pub fn parse<'a>(data: &'a str) -> impl Iterator<Item = AssignmnetPair> + 'a {
    let (_, pairs) = parse_assignment_pairs(data).unwrap();
    pairs.into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_overlap1() {
        assert_eq!(AssignmnetPair::new(6, 6, 4, 6).full_overlap(), true)
    }

    #[test]
    fn test_full_overlap2() {
        assert_eq!(AssignmnetPair::new(2, 8, 3, 7).full_overlap(), true)
    }

    #[test]
    fn test_full_overlap3() {
        assert_eq!(AssignmnetPair::new(1, 3, 4, 7).full_overlap(), false)
    }

    #[test]
    fn test_full_overlap4() {
        assert_eq!(AssignmnetPair::new(1, 5, 3, 7).full_overlap(), false)
    }

    #[test]
    fn test_any_overlap1() {
        assert_eq!(AssignmnetPair::new(2, 4, 4, 6).any_overlap(), true)
    }

    #[test]
    fn test_any_overlap2() {
        assert_eq!(AssignmnetPair::new(1, 3, 4, 6).any_overlap(), false)
    }

    #[test]
    fn test_any_overlap3() {
        assert_eq!(AssignmnetPair::new(1, 5, 4, 7).any_overlap(), true)
    }
}
