use nom::{character::complete::newline, multi::separated_list1};
use parser::point;
use util::{BitMap, Pos};

type InputType = Vec<Pos<usize>>;
type OutType = usize;

pub struct Map {
    bytes: BitMap<usize>,
    width: usize,
    height: usize,
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let (rest, points) = separated_list1(newline, point(","))(data).unwrap();
    assert_eq!("", rest.trim());
    points
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    let width = 71;
    let height = 71;

    let mut bytes = BitMap::new(width, height);
    for p in input.iter().take(1024) {
        bytes.insert(p);
    }

    let map = Map {
        bytes,
        width,
        height,
    };

    pathfinding::directed::dijkstra::dijkstra(
        &Pos::new(0, 0),
        |p| {
            p.successors_4_unsigned()
                .into_iter()
                .filter(|p| p.x < map.width && p.y < map.height && !map.bytes.contains(p))
                .map(|p| (p, 1))
        },
        |p| p.x == map.width - 1 && p.y == map.height - 1,
    )
    .unwrap()
    .1
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> String {
    let width = 71;
    let height = 71;

    let mut bytes = BitMap::new(width, height);

    for p in input.iter().take(1024) {
        bytes.insert(p);
    }

    let mut solution_points = BitMap::from_iter(
        pathfinding::directed::dijkstra::dijkstra(
            &Pos::new(0, 0),
            |p| {
                p.successors_4_unsigned()
                    .into_iter()
                    .filter(|p| p.x < width && p.y < height && !bytes.contains(p))
                    .map(|p| (p, 1))
            },
            |p| p.x == width - 1 && p.y == height - 1,
        )
        .unwrap()
        .0
        .iter(),
        width,
        height,
    );

    for p in input.iter().skip(1024) {
        bytes.insert(p);

        // If it didn't fall on the solution path, skip it
        if !solution_points.contains(p) {
            continue;
        }

        let res = pathfinding::directed::dijkstra::dijkstra(
            &Pos::new(0, 0),
            |p| {
                p.successors_4_unsigned()
                    .into_iter()
                    .filter(|p| p.x < width && p.y < height && !bytes.contains(p))
                    .map(|p| (p, 1))
            },
            |p| p.x == width - 1 && p.y == height - 1,
        );

        if res.is_none() {
            return format!("{:?}", p);
        } else {
            // It fell on the solution path, but didn't block it, update the path
            solution_points = BitMap::from_iter(res.unwrap().0.iter(), width, height);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test! { 2024, 18, 1, 322}
    generate_test! { 2024, 18, 2, "(60,21)"}
}
