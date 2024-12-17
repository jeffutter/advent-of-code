use itertools::Itertools;
use util::{BitMap, Direction, Pos};

type InputType = Maze;
type OutType = usize;

#[derive(Clone)]
pub struct Maze {
    walls: BitMap<usize>,
    p: Pos<usize>,
    e: Pos<usize>,
    facing: Direction,
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let width = data.lines().next().unwrap().len();
    let height = data
        .lines()
        .filter(|l| !l.is_empty() && l.starts_with('#'))
        .count();
    let mut p = None;
    let mut e = None;
    let mut walls = BitMap::new(width, height);

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => walls.insert(&Pos::new(x, y)),
                'S' => p = Some(Pos::new(x, y)),
                'E' => e = Some(Pos::new(x, y)),
                _ => unimplemented!(),
            }
        }
    }

    Maze {
        walls,
        p: p.unwrap(),
        e: e.unwrap(),
        facing: Direction::E,
    }
}

fn successors(
    maze: &Maze,
    p: &Pos<usize>,
    facing: &Direction,
) -> Vec<((Pos<usize>, Direction), usize)> {
    let mut possible_moves: Vec<((Pos<usize>, Direction), usize)> = Vec::new();

    let next = p.translate(facing).unwrap();

    if !maze.walls.contains(&next) {
        possible_moves.push(((next.clone(), facing.clone()), 1));
    }

    let possible_directions = match facing {
        Direction::N => [Direction::E, Direction::W],
        Direction::E => [Direction::N, Direction::S],
        Direction::S => [Direction::E, Direction::W],
        Direction::W => [Direction::S, Direction::N],
    };

    for direction in possible_directions {
        possible_moves.push(((p.clone(), direction.clone()), 1000));
    }

    possible_moves
}

#[allow(unused_variables)]
pub fn part1(maze: InputType) -> OutType {
    pathfinding::directed::astar::astar(
        &(maze.p.clone(), maze.facing.clone()),
        |(p, facing): &(Pos<usize>, Direction)| successors(&maze, p, facing),
        |(p, _d)| p.x.abs_diff(maze.e.x) + p.y.abs_diff(maze.e.y),
        |(p, _d)| *p == maze.e,
    )
    .unwrap()
    .1
}

#[allow(unused_variables)]
pub fn part2(maze: InputType) -> OutType {
    pathfinding::directed::astar::astar_bag(
        &(maze.p.clone(), maze.facing.clone()),
        |(p, facing): &(Pos<usize>, Direction)| successors(&maze, p, facing),
        |(p, _d)| p.x.abs_diff(maze.e.x) + p.y.abs_diff(maze.e.y),
        |(p, _d)| *p == maze.e,
    )
    .unwrap()
    .0
    .flat_map(|paths| paths.into_iter().map(|(point, _)| point))
    .unique()
    .count()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#,
        1,
        7036
    );

    generate_test!(
        r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#,
        2,
        45
    );

    generate_test! { 2024, 16, 1, 103512}
    generate_test! { 2024, 16, 2, 554}
}
