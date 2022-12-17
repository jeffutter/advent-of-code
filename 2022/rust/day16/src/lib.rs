use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn part1(v: Valves) -> u16 {
    let mm: usize = 1 << v.positive_flow().count();

    let opt = generate_matrix(&v);

    opt[29][v.idx_by_name("AA").unwrap()][mm - 1]
}

fn generate_matrix(v: &Valves) -> Vec<Vec<Vec<u16>>> {
    let mut adj = vec![vec![0usize; 0]; v.len()];
    let mut flow = vec![0u16; v.len()];
    for vx in v.iter() {
        let i = v.idx_by_name(vx.name).unwrap();
        flow[i] = vx.flow_rate;
        for w in vx.connections.iter() {
            adj[i].push(v.idx_by_name(w).unwrap());
        }
    }

    let mm: usize = 1 << v.positive_flow().count();
    let mut opt: Vec<Vec<Vec<u16>>> = vec![vec![vec![0; mm]; v.len()]; 30];

    for t in 1..30 {
        for i in 0..v.len() {
            let ii: u64 = 1 << i;
            for x in 0..mm {
                let mut o = opt[t][i][x];
                if ii & (x as u64) != 0 && t >= 2 {
                    let val = opt[t - 1][i][((x as u64) - ii) as usize] + flow[i] * (t as u16);
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

pub fn part2(v: Valves) -> u16 {
    let mm: usize = 1 << v.positive_flow().count();
    let opt = generate_matrix(&v);

    let mut best = 0;
    for x in 0..mm / 2 {
        let y = mm - 1 - x;
        best = best.max(
            opt[25][v.idx_by_name("AA").unwrap()][x] + opt[25][v.idx_by_name("AA").unwrap()][y],
        );
    }
    best
}

pub fn parse<'a>(data: &'a str) -> Valves<'a> {
    let (rest, valves) = separated_list1(newline, valve)(data).unwrap();
    assert_eq!(rest.trim(), "");

    let mut vs = Valves::new();

    for valve in valves {
        vs.insert(valve);
    }

    vs
}

fn valve(s: &str) -> IResult<&str, Valve> {
    let (rest, (_, name, _, flow_rate, _, connections)) = tuple((
        tag("Valve "),
        alpha1,
        tag(" has flow rate="),
        map_res(digit1, |s: &str| u16::from_str_radix(s, 10)),
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), alpha1),
    ))(s)?;
    Ok((rest, Valve::new(name, flow_rate, connections)))
}

#[derive(Debug)]
pub struct Valves<'a> {
    vs: Vec<Valve<'a>>,
    by_name: HashMap<&'a str, Valve<'a>>,
    idx_by_name: HashMap<&'a str, usize>,
}

impl<'a> Valves<'a> {
    pub fn new() -> Self {
        Self {
            vs: Vec::new(),
            by_name: HashMap::new(),
            idx_by_name: HashMap::new(),
        }
    }

    fn positive_flow(&self) -> impl Iterator<Item = &str> {
        self.vs.iter().filter_map(|x| {
            if x.flow_rate > 0 {
                return Some(x.name);
            }
            None
        })
    }

    fn insert(&mut self, v: Valve<'a>) {
        self.vs.push(v.clone());
        self.vs.sort_by_key(|x| -(x.flow_rate as i32));
        self.by_name.insert(v.name, v);

        self.idx_by_name = self
            .vs
            .iter()
            .enumerate()
            .map(|(i, x)| (x.name, i))
            .collect();
    }

    fn idx_by_name(&self, name: &str) -> Option<usize> {
        self.idx_by_name.get(name).copied()
    }

    fn iter(&self) -> impl Iterator<Item = &Valve<'a>> {
        self.vs.iter()
    }

    fn len(&self) -> usize {
        self.vs.len()
    }
}

#[derive(Clone, Debug, PartialOrd, Eq, PartialEq)]
pub struct Valve<'a> {
    name: &'a str,
    flow_rate: u16,
    connections: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    pub fn new(name: &'a str, flow_rate: u16, connections: Vec<&'a str>) -> Self {
        Self {
            name,
            flow_rate,
            connections,
        }
    }
}

impl<'a> std::cmp::Ord for Valve<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.flow_rate.cmp(&other.flow_rate)
    }
}

#[cfg(test)]
mod tests {

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

    #[test]
    fn test_ord() {
        let a = Valve::new("a", 10, vec![]);
        let b = Valve::new("b", 11, vec![]);
        let c = Valve::new("c", 12, vec![]);
        let d = Valve::new("d", 12, vec![]);

        assert_eq!(
            vec![&c, &b, &a].into_iter().sorted().collect::<Vec<_>>(),
            vec![&a, &b, &c]
        );

        assert_eq!(
            vec![&d, &c, &a].into_iter().sorted().collect::<Vec<_>>(),
            vec![&a, &c, &d]
        )
    }
}
