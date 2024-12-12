use std::collections::{HashMap, HashSet, VecDeque};

use util::{Direction, Pos};

type InputType = HashMap<char, HashSet<Pos<usize>>>;
type OutType = usize;

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let mut maps: InputType = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            maps.entry(char)
                .and_modify(|c| {
                    c.insert(Pos::new(x, y));
                })
                .or_insert_with(|| {
                    let mut map = HashSet::new();
                    map.insert(Pos::new(x, y));
                    map
                });
        }
    }
    maps
}

fn needed_fence(mut map: HashSet<Pos<usize>>) -> usize {
    let mut total = 0;

    while let Some(p) = map.clone().iter().next() {
        let included_points = find_shapes(p, &mut map);

        let area = included_points.len();
        let perimeter: usize = included_points
            .iter()
            .map(|p| {
                4 - p
                    .successors_4_unsigned()
                    .iter()
                    .filter(|p| included_points.contains(p))
                    .count()
            })
            .sum();

        total += area * perimeter;
    }

    total
}

fn find_shapes(start: &Pos<usize>, map: &mut HashSet<Pos<usize>>) -> HashSet<Pos<usize>> {
    let mut included_points: HashSet<Pos<usize>> = HashSet::new();

    let mut work: VecDeque<Pos<usize>> = VecDeque::new();
    work.push_front(start.clone());

    while let Some(p) = work.pop_front() {
        if included_points.contains(&p) {
            continue;
        }
        if !map.contains(&p) {
            continue;
        }

        map.remove(&p);
        included_points.insert(p.clone());

        work.extend(p.successors_4_unsigned());
    }

    included_points
}

fn count_exterrior_corners(map: &HashSet<Pos<usize>>, p: &Pos<usize>) -> usize {
    const CORNER_DIRECTIONS: [(Direction, Direction); 4] = [
        (Direction::N, Direction::E),
        (Direction::E, Direction::S),
        (Direction::S, Direction::W),
        (Direction::W, Direction::N),
    ];

    CORNER_DIRECTIONS
        .iter()
        .map(|(p1, p2)| (p.translate(p1), p.translate(p2)))
        .filter(|(p1, p2)| {
            !p1.clone().map(|p1| map.contains(&p1)).unwrap_or(false)
                && !p2.clone().map(|p2| map.contains(&p2)).unwrap_or(false)
        })
        .count()
}

fn count_interrior_corners(map: &HashSet<Pos<usize>>, p: &Pos<usize>) -> usize {
    const CORNER_DIRECTIONS: [(Direction, Direction, (Direction, Direction)); 4] = [
        (Direction::N, Direction::E, (Direction::N, Direction::E)),
        (Direction::E, Direction::S, (Direction::E, Direction::S)),
        (Direction::S, Direction::W, (Direction::S, Direction::W)),
        (Direction::W, Direction::N, (Direction::W, Direction::N)),
    ];

    CORNER_DIRECTIONS
        .iter()
        .map(|(p1, p2, (p3, p4))| {
            (
                p.translate(p1),
                p.translate(p2),
                p.translate(p3).and_then(|x| x.translate(p4)),
            )
        })
        .filter(|(p1, p2, p3)| {
            p1.clone().map(|p1| map.contains(&p1)).unwrap_or(false)
                && p2.clone().map(|p2| map.contains(&p2)).unwrap_or(false)
                && !p3.clone().map(|p3| map.contains(&p3)).unwrap_or(false)
        })
        .count()
}

fn needed_fence_sides(mut map: HashSet<Pos<usize>>) -> usize {
    let mut total = 0;

    while let Some(p) = map.clone().iter().next() {
        let included_points = find_shapes(p, &mut map);

        let area = included_points.len();
        let sides: usize = included_points
            .iter()
            .map(|p| {
                count_exterrior_corners(&included_points, p)
                    + count_interrior_corners(&included_points, p)
            })
            .sum();

        total += area * sides;
    }

    total
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    input.into_values().map(needed_fence).sum()
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    input.into_values().map(needed_fence_sides).sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#,
        1,
        1930
    );

    #[test]
    fn example_21() {
        let data = parse(
            r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#,
        );
        assert_eq!(part2(data), 236)
    }

    #[test]
    fn example_22() {
        let data = parse(
            r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#,
        );
        assert_eq!(part2(data), 368)
    }

    generate_test!(
        r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#,
        2,
        1206
    );

    generate_test! { 2024, 12, 1, 1387004}
    generate_test! { 2024, 12, 2, 844198}
}
