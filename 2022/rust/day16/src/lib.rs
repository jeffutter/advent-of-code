use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn part1(v: Valves) -> u32 {
    // let vertices = &v.0.keys().cloned().collect::<Vec<_>>()[..];
    let valves =
        v.0.values()
            .sorted_by_key(|x| x.flow_rate)
            .rev()
            .collect_vec();

    let lab2idx = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name, i))
        .collect::<HashMap<_, _>>();

    let positive_flow = valves
        .iter()
        .filter_map(|x| {
            if x.flow_rate > 0 {
                return Some(x.name);
            }
            None
        })
        .collect_vec();

    let mut adj = vec![vec![0usize; 0]; valves.len()];
    let mut flow = vec![0u32; valves.len()];
    for v in valves {
        let i = lab2idx[v.name];
        flow[i] = v.flow_rate;
        for w in v.connections.iter() {
            adj[i].push(lab2idx[w]);
        }
    }

    let mut opt: HashMap<(u32, usize, u32), u32> = HashMap::new();
    let mm: u32 = 1 << positive_flow.len();
    for t in 1..30 {
        for i in 0..v.0.len() {
            let ii: u64 = 1 << i;
            for x in 0..mm {
                let idx = (t, i, x);
                let mut o = opt.get(&idx).unwrap_or(&0).clone();
                if ii & (x as u64) != 0 && t >= 2 {
                    let idx2 = (t - 1, i, ((x as u64) - ii) as u32);
                    let val = opt.get(&idx2).unwrap_or(&0) + flow[i] * t;
                    o = o.max(val);
                }
                for &j in adj[i].iter() {
                    let idx2 = (t - 1, j, x);
                    let val = opt.get(&idx2).unwrap_or(&0).clone();
                    o = o.max(val);
                }
                opt.insert(idx, o);
            }
        }
    }

    opt.get(&(29u32, lab2idx["AA"], mm - 1)).unwrap().clone()
}

pub fn part2(v: Valves) -> i32 {
    1
}

pub fn parse<'a>(data: &'a str) -> Valves {
    let (rest, valves) = separated_list1(newline, valve)(data).unwrap();
    assert_eq!(rest.trim(), "");

    Valves(
        valves
            .into_iter()
            .map(|valve: Valve| (valve.name.clone(), valve))
            .collect(),
    )
}

fn valve(s: &str) -> IResult<&str, Valve> {
    let (rest, (_, name, _, flow_rate, _, connections)) = tuple((
        tag("Valve "),
        alpha1,
        tag(" has flow rate="),
        parser::from_dig,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), alpha1),
    ))(s)?;
    Ok((
        rest,
        Valve {
            name,
            flow_rate: flow_rate as u32,
            connections,
            open: false,
        },
    ))
}

#[derive(Debug)]
pub struct Valves<'a>(HashMap<&'a str, Valve<'a>>);

#[derive(Debug)]
pub struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    connections: Vec<&'a str>,
    open: bool,
}

#[cfg(test)]
mod tests {
    use pathfinding::prelude::{DenseCapacity, SparseCapacity};

    use super::*;

    #[test]
    fn test1() {
        let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
        let parsed = parse(input);
        let res = part1(parsed);
        assert_eq!(1651, res)
    }
}
