use util::{BitMap, Direction, Pos};

type InputType = Map;
type OutType = usize;

type Mover = Box<dyn FnOnce(&mut Map)>;

#[derive(Clone)]
pub struct Map {
    robot: Pos<usize>,
    boxesl: BitMap<usize>,
    boxesr: BitMap<usize>,
    walls: BitMap<usize>,
    moves: Vec<Direction>,
    width: usize,
    height: usize,
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Pos::new(x, y);

                if self.robot == pos {
                    buf.push('@');
                    continue;
                }

                if self.walls.contains(&pos) {
                    buf.push('#');
                    continue;
                }

                if self.boxesl.contains(&pos) {
                    buf.push('[');
                    continue;
                }
                if self.boxesr.contains(&pos) {
                    buf.push(']');
                    continue;
                }

                buf.push('.');
            }

            buf.push('\n');
        }

        f.write_str(&buf)?;

        Ok(())
    }
}

impl Map {
    #[allow(dead_code)]
    fn validate(&self) -> bool {
        self.boxesl.iter().all(|l| {
            let r = l.translate(&Direction::E).unwrap();
            self.boxesr.contains(&r)
        }) && self.boxesr.iter().all(|r| {
            let l = r.translate(&Direction::W).unwrap();
            self.boxesl.contains(&l)
        })
    }

    fn build_robot_mover(&self, next: Pos<usize>) -> Mover {
        Box::new(move |map: &mut Map| {
            map.robot = next;
        })
    }

    fn build_box_mover(&self, left_p: Pos<usize>, left_next: Pos<usize>) -> Mover {
        let right_p = left_p.translate(&Direction::E).unwrap();
        let right_next = left_next.translate(&Direction::E).unwrap();

        Box::new(move |map: &mut Map| {
            map.boxesl.remove(&left_p);
            map.boxesr.remove(&right_p);
            map.boxesl.insert(&left_next);
            map.boxesr.insert(&right_next);
        })
    }

    fn maybe_move_box(
        &self,
        left_p: &Pos<usize>,
        d: &Direction,
    ) -> Option<Box<dyn Iterator<Item = Mover>>> {
        let right_p = left_p.translate(&Direction::E).unwrap();
        let left_next = left_p.translate(d).unwrap();
        let right_next = right_p.translate(d).unwrap();

        if self.walls.contains(&left_next) || self.walls.contains(&right_next) {
            return None;
        }

        let self_move = std::iter::once(self.build_box_mover(left_p.clone(), left_next.clone()));

        if [Direction::E, Direction::W].contains(d) {
            let next_box_left_p = left_p.translate_n(d, 2).unwrap();

            if self.boxesl.contains(&next_box_left_p) {
                if let Some(moves) = self.maybe_move_box(&next_box_left_p, d) {
                    return Some(Box::new(moves.into_iter().chain(self_move)));
                }
                return None;
            }
            return Some(Box::new(self_move));
        }

        // Vertically stacked boxes
        if self.boxesl.contains(&left_next) {
            if let Some(moves) = self.maybe_move_box(&left_next, d) {
                return Some(Box::new(moves.into_iter().chain(self_move)));
            }
            return None;
        }

        // Test both of these, if there's a box, try to move it
        if self.boxesr.contains(&left_next) || self.boxesl.contains(&right_next) {
            let next_left_box_left = left_next.translate(&Direction::W).unwrap();
            let next_right_box_left = right_next;
            let mut mvs = vec![];

            if self.boxesr.contains(&left_next) {
                if let Some(moves) = self.maybe_move_box(&next_left_box_left, d) {
                    mvs.push(moves);
                } else {
                    return None;
                }
            }
            if self.boxesl.contains(&next_right_box_left) {
                if let Some(moves) = self.maybe_move_box(&next_right_box_left, d) {
                    mvs.push(moves);
                } else {
                    return None;
                }
            }
            mvs.push(Box::new(self_move));

            return Some(Box::new(mvs.into_iter().flatten()));
        }

        Some(Box::new(self_move))
    }

    fn maybe_move(&self, p: &Pos<usize>, d: &Direction) -> Option<Box<dyn Iterator<Item = Mover>>> {
        let next = p.translate(d).unwrap();

        if self.walls.contains(&next) {
            return None;
        }

        let self_move = std::iter::once(self.build_robot_mover(next.clone()));

        if self.boxesl.contains(&next) {
            if let Some(moves) = self.maybe_move_box(&next, d) {
                return Some(Box::new(moves.into_iter().chain(self_move)));
            }
            return None;
        }

        if self.boxesr.contains(&next) {
            let next_box_left = next.translate(&Direction::W).unwrap();
            if let Some(moves) = self.maybe_move_box(&next_box_left, d) {
                return Some(Box::new(moves.into_iter().chain(self_move)));
            }
            return None;
        }

        Some(Box::new(self_move))
    }
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let width = data.lines().next().unwrap().len();
    let height = data
        .lines()
        .filter(|l| !l.is_empty() && l.starts_with('#'))
        .count();
    let mut robot = None;
    let mut boxesl = BitMap::new(width, height);
    let mut walls = BitMap::new(width, height);
    let mut moves = Vec::new();

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => walls.insert(&Pos::new(x, y)),
                'O' => boxesl.insert(&Pos::new(x, y)),
                '@' => robot = Some(Pos::new(x, y)),
                '<' => moves.push(Direction::W),
                '>' => moves.push(Direction::E),
                '^' => moves.push(Direction::N),
                'v' => moves.push(Direction::S),
                _ => unimplemented!(),
            }
        }
    }

    Map {
        robot: robot.unwrap(),
        boxesl,
        boxesr: BitMap::new(width, height),
        walls,
        moves,
        width,
        height,
    }
}

fn maybe_move_box(p: &Pos<usize>, d: &Direction, map: &mut Map) -> bool {
    let next = p.translate(d).unwrap();

    if map.walls.contains(&next) {
        return false;
    }

    if map.boxesl.contains(&next) {
        if maybe_move_box(&next, d, map) {
            map.boxesl.remove(p);
            map.boxesl.insert(&next);
            return true;
        }
        return false;
    }

    map.boxesl.remove(p);
    map.boxesl.insert(&next);
    true
}

#[allow(unused_variables)]
pub fn part1(mut map: InputType) -> OutType {
    for mv in map.moves.clone() {
        let next = map.robot.translate(&mv).unwrap();

        if map.walls.contains(&next) {
            continue;
        }

        if map.boxesl.contains(&next) {
            if maybe_move_box(&next, &mv, &mut map) {
                map.robot = next;
            }
            continue;
        }

        map.robot = next;
    }

    map.boxesl.iter().map(|p| 100 * p.y + p.x).sum()
}

#[allow(unused_variables)]
pub fn part2(
    Map {
        robot,
        boxesl,
        boxesr,
        walls,
        moves,
        width,
        height,
    }: InputType,
) -> OutType {
    let new_robot = Pos::new(robot.x * 2, robot.y);
    let mut new_boxesl = BitMap::new(width * 2, height);
    let mut new_boxesr = BitMap::new(width * 2, height);
    for p in boxesl.iter() {
        new_boxesl.insert(&Pos::new(p.x * 2, p.y));
        new_boxesr.insert(&Pos::new(p.x * 2 + 1, p.y));
    }
    let mut new_walls = BitMap::new(width * 2, height);
    for p in walls.iter() {
        new_walls.insert(&Pos::new(p.x * 2, p.y));
        new_walls.insert(&Pos::new(p.x * 2 + 1, p.y));
    }
    let mut map = Map {
        robot: new_robot,
        boxesl: new_boxesl,
        boxesr: new_boxesr,
        walls: new_walls,
        width: width * 2,
        height,
        moves,
    };

    for mv in map.moves.clone() {
        if let Some(moves) = map.maybe_move(&map.robot, &mv) {
            let backup = map.clone();

            for f in moves {
                f(&mut map);
            }
        }
    }

    map.boxesl.iter().map(|p| 100 * p.y + p.x).sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#,
        1,
        10092
    );

    #[test]
    fn example_2_0() {
        let data = parse(
            r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#,
        );
        assert_eq!(part2(data), 618)
    }

    #[test]
    fn example_x1() {
        let data = parse(
            r#"##########
#........#
#....O#..#
#....OO..#
#...OO...#
#....OO@.#
#....O...#
#........#
##########

<vv<<<^"#,
        );
        assert_eq!(part2(data), 2580)
    }

    generate_test!(
        r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#,
        2,
        9021
    );

    generate_test! { 2024, 15, 1, 1486930}
    generate_test! { 2024, 15, 2, 1492011}
}
