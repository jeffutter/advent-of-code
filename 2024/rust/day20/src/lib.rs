use util::{BitMap, Pos};

type InputType = Map;
type OutType = usize;

pub struct Map {
    walls: BitMap<usize>,
    start: Pos<usize>,
    end: Pos<usize>,
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let height = data.lines().count();
    let width = data.lines().next().unwrap().len();
    let mut start = None;
    let mut end = None;

    let mut walls = BitMap::new(width, height);
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Pos::new(x, y);

            match c {
                '.' => (),
                '#' => walls.insert(&p),
                'S' => start = Some(p),
                'E' => end = Some(p),
                c => unimplemented!("{c}"),
            }
        }
    }

    Map {
        walls,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn count_cheats(map: &Map, min_saved: usize, max_cheat: usize) -> usize {
    let path = pathfinding::directed::dfs::dfs(
        map.start.clone(),
        |p: &Pos<usize>| {
            p.successors_4_unsigned()
                .into_iter()
                .filter(|p| !map.walls.contains(p))
        },
        |p| *p == map.end,
    )
    .unwrap();

    let path_len = path.len();
    let max_end_idx = path_len - min_saved;

    path.iter()
        .enumerate()
        .take(max_end_idx)
        .flat_map(|(sidx, sp)| {
            path.iter()
                .enumerate()
                .skip(sidx + min_saved)
                .filter_map(move |(eidx, ep)| {
                    let m_dist = sp.manhattan_distance_unsigned(ep);
                    let saved = eidx - sidx - m_dist;
                    if m_dist <= max_cheat && saved >= min_saved {
                        return Some((sp, ep, saved, m_dist, sidx, eidx));
                    }

                    None
                })
        })
        .count()
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    count_cheats(&input, 100, 2)
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    count_cheats(&input, 100, 20)
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;

    #[test]
    fn ex1() {
        let map = parse(SAMPLE);
        assert_eq!(1, count_cheats(&map, 64, 2));
        assert_eq!(2, count_cheats(&map, 40, 2));
        assert_eq!(3, count_cheats(&map, 38, 2));
        assert_eq!(4, count_cheats(&map, 36, 2));
        assert_eq!(5, count_cheats(&map, 20, 2));
        assert_eq!(8, count_cheats(&map, 12, 2));
        assert_eq!(10, count_cheats(&map, 10, 2));
        assert_eq!(14, count_cheats(&map, 8, 2));
        assert_eq!(16, count_cheats(&map, 6, 2));
        assert_eq!(30, count_cheats(&map, 4, 2));
        assert_eq!(44, count_cheats(&map, 2, 2));
    }

    #[test]
    fn ex2() {
        let map = parse(SAMPLE);
        assert_eq!(3, count_cheats(&map, 76, 20));
        assert_eq!(7, count_cheats(&map, 74, 20));
        assert_eq!(29, count_cheats(&map, 72, 20));
        assert_eq!(41, count_cheats(&map, 70, 20));
        assert_eq!(55, count_cheats(&map, 68, 20));
        assert_eq!(86, count_cheats(&map, 64, 20));
        assert_eq!(106, count_cheats(&map, 62, 20));
        assert_eq!(154, count_cheats(&map, 58, 20));
        assert_eq!(193, count_cheats(&map, 56, 20));
        assert_eq!(222, count_cheats(&map, 54, 20));
        assert_eq!(253, count_cheats(&map, 52, 20));
        assert_eq!(285, count_cheats(&map, 50, 20));
    }

    generate_test! { 2024, 20, 1, 1321}
    generate_test! { 2024, 20, 2, 971737}
}
