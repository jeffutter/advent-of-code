use std::collections::HashMap;

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
    let valves =
        v.0.values()
            .sorted_by_key(|x| x.flow_rate)
            .rev()
            .collect_vec();
    let positive_flow = valves
        .iter()
        .filter_map(|x| {
            if x.flow_rate > 0 {
                return Some(x.name);
            }
            None
        })
        .collect_vec();
    let lab2idx = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name, i))
        .collect::<HashMap<_, _>>();
    let mm: usize = 1 << positive_flow.len();

    let opt = generate_matrix(v);

    opt[29][lab2idx["AA"]][mm - 1]
}

fn generate_matrix(v: Valves) -> Vec<Vec<Vec<u32>>> {
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

    let mm: usize = 1 << positive_flow.len();
    let mut opt: Vec<Vec<Vec<u32>>> = vec![vec![vec![0; mm]; v.0.len()]; 30];

    for t in 1..30 {
        for i in 0..v.0.len() {
            let ii: u64 = 1 << i;
            for x in 0..mm {
                let mut o = opt[t][i][x];
                if ii & (x as u64) != 0 && t >= 2 {
                    let val = opt[t - 1][i][((x as u64) - ii) as usize] + flow[i] * (t as u32);
                    o = o.max(val);
                }
                for &j in adj[i].iter() {
                    let val = opt[t - 1][j][x];
                    o = o.max(val);
                }
                opt[t][i][x] = o;
            }
        }
    }

    opt
}

pub fn part2(v: Valves) -> u32 {
    let valves =
        v.0.values()
            .sorted_by_key(|x| x.flow_rate)
            .rev()
            .collect_vec();
    let positive_flow = valves
        .iter()
        .filter_map(|x| {
            if x.flow_rate > 0 {
                return Some(x.name);
            }
            None
        })
        .collect_vec();
    let lab2idx = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name, i))
        .collect::<HashMap<_, _>>();
    let mm: usize = 1 << positive_flow.len();
    let opt = generate_matrix(v);

    let mut best = 0;
    for x in 0..mm / 2 {
        let y = mm - 1 - x;
        best = best.max(opt[25][lab2idx["AA"]][x] + opt[25][lab2idx["AA"]][y]);
    }
    best
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
