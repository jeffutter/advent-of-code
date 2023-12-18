use std::collections::HashMap;

use itertools::Itertools;
use util::{Direction, Pos};

pub struct CityMap {
    blocks: HashMap<Pos<i32>, u32>,
    max: Pos<i32>,
    min: Pos<i32>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Crucible {
    pos: Pos<i32>,
    in_a_row: usize,
    facing: Option<Direction>,
}

impl Crucible {
    fn next(
        &self,
        min: &Pos<i32>,
        max: &Pos<i32>,
        min_in_a_row: usize,
        max_in_a_row: usize,
    ) -> Vec<Self> {
        let facing = self.facing.as_ref();
        match self.facing.clone() {
            None => {
                vec![Direction::N, Direction::E, Direction::S, Direction::W]
            }
            Some(Direction::N) => {
                vec![Direction::N, Direction::E, Direction::W]
            }
            Some(Direction::E) => {
                vec![Direction::N, Direction::E, Direction::S]
            }
            Some(Direction::S) => {
                vec![Direction::E, Direction::S, Direction::W]
            }
            Some(Direction::W) => {
                vec![Direction::N, Direction::S, Direction::W]
            }
        }
        .iter()
        .filter(|direction| {
            if facing.is_none() {
                return true;
            }
            if self.in_a_row < min_in_a_row {
                return *direction == facing.unwrap();
            }
            if self.in_a_row >= max_in_a_row {
                return *direction != facing.unwrap();
            }
            true
        })
        .map(|direction| {
            let in_a_row = if facing.map(|f| f == direction).unwrap_or(false) {
                self.in_a_row + 1
            } else {
                1
            };

            (direction, self.pos.translate(direction), in_a_row)
        })
        .filter_map(|x| x.1.map(|pos| (x.0, pos, x.2)))
        .filter(|(_, pos, _)| pos.x >= min.x && pos.y >= min.y && pos.x <= max.x && pos.y <= max.y)
        .map(|(direction, pos, in_a_row)| Crucible {
            facing: Some(direction.clone()),
            pos,
            in_a_row,
        })
        .collect_vec()
    }
}

pub fn parse<'a>(data: &'a str) -> CityMap {
    let mut blocks = HashMap::new();
    let mut max = Pos::new(0, 0);

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            max.x = max.x.max(x as i32);
            max.y = max.y.max(y as i32);

            blocks.insert(Pos::new(x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }

    CityMap {
        blocks,
        max,
        min: Pos::new(0, 0),
    }
}

pub fn part1<'a>(map: CityMap) -> u32 {
    let start = Crucible {
        pos: Pos::new(0, 0),
        in_a_row: 0,
        facing: None,
    };

    pathfinding::directed::dijkstra::dijkstra(
        &start,
        |crucible| {
            crucible
                .next(&map.min, &map.max, 0, 3)
                .into_iter()
                .map(|crucible| (crucible.clone(), *map.blocks.get(&crucible.pos).unwrap()))
        },
        |crucible| crucible.pos == map.max,
    )
    .unwrap()
    .1
}

pub fn part2<'a>(map: CityMap) -> u32 {
    let start = Crucible {
        pos: Pos::new(0, 0),
        in_a_row: 0,
        facing: None,
    };

    pathfinding::directed::dijkstra::dijkstra(
        &start,
        |crucible| {
            crucible
                .next(&map.min, &map.max, 4, 10)
                .into_iter()
                .map(|crucible| (crucible.clone(), *map.blocks.get(&crucible.pos).unwrap()))
        },
        |crucible| crucible.pos == map.max && crucible.in_a_row >= 4,
    )
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 102);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 94);
    }

    #[test]
    fn test_sample_3() {
        let data = parse(
            &r#"111111111111
999999999991
999999999991
999999999991
999999999991"#,
        );
        assert_eq!(part2(data), 71);
    }

    generate_test! { 2023, 17, 1, 635}
    generate_test! { 2023, 17, 2, 734}
}
