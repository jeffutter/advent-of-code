use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::take,
    character::streaming::line_ending,
    combinator::map,
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

fn single_digit(s: &str) -> IResult<&str, usize> {
    map_res(take(1usize), |s: &str| usize::from_str_radix(s, 10))(s)
}

fn parse_map(s: &str) -> IResult<&str, HashMap<(usize, usize), usize>> {
    map(separated_list1(line_ending, many1(single_digit)), |rows| {
        let mut y = 0;
        let mut hm: HashMap<(usize, usize), usize> = HashMap::new();
        for row in rows {
            let mut x = 0;
            for v in row {
                hm.insert((x, y), v);
                x += 1;
            }
            y += 1;
        }
        hm
    })(s)
}

fn surrounding(
    map: &HashMap<(usize, usize), usize>,
    point: (usize, usize),
) -> HashMap<(usize, usize), usize> {
    let mut surrounding: HashMap<(usize, usize), usize> = HashMap::new();
    let x = point.0 as i32;
    let y = point.1 as i32;

    [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
        .iter()
        .for_each(|point| {
            usize::try_from(point.0)
                .and_then(|x| usize::try_from(point.1).and_then(|y| Ok((x, y))))
                .ok()
                .and_then(|point| map.get(&point).and_then(|v| surrounding.insert(point, *v)));
        });

    surrounding
}

fn find_lows(map: &HashMap<(usize, usize), usize>) -> HashMap<(usize, usize), usize> {
    let mut lows: HashMap<(usize, usize), usize> = HashMap::new();

    for (point, v) in map {
        let surround = surrounding(&map, *point);

        if surround.iter().all(|(_point, surr_v)| surr_v > &v) {
            lows.insert(*point, *v);
        }
    }

    lows
}

pub fn part1(data: String) -> usize {
    let (_rest, map) = parse_map(&data).unwrap();

    find_lows(&map).values().map(|x| x + 1).sum()
}

fn basin_size<'a>(
    map: &HashMap<(usize, usize), usize>,
    point: (usize, usize),
    visited: &'a mut HashSet<(usize, usize)>,
) -> &'a HashSet<(usize, usize)> {
    if visited.insert(point) {
        let surround = surrounding(map, point);

        for (point, _) in surround {
            basin_size(map, point, visited);
        }
    }

    visited
}

pub fn part2(data: String) -> i32 {
    let (_rest, mut map) = parse_map(&data).unwrap();

    for (point, v) in map.clone().into_iter() {
        if v >= 9usize {
            map.remove(&point);
        }
    }

    let lows = find_lows(&map);

    let mut sizes = lows
        .iter()
        .map(|(point, _)| {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            basin_size(&map, *point, &mut visited);
            visited.len() as i32
        })
        .collect::<Vec<i32>>();

    sizes.sort();

    sizes.iter().rev().take(3).product()
}
