use std::collections::{HashMap, HashSet, VecDeque};

use enum_dispatch::enum_dispatch;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pulse {
    High,
    Low,
}

#[enum_dispatch]
pub trait ModuleControl {
    fn id(&self) -> String;
    fn accept_pulse(&mut self, from: String, pulse: Pulse);
    fn process_pulses(&mut self) -> Vec<(String, Pulse)>;
    fn connections(&self) -> Vec<String>;
    fn insert_upstream(&mut self, name: String);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlipFlopModule {
    state: bool,
    id: String,
    connections: Vec<String>,
    incoming_pulses: HashMap<String, Pulse>,
}

impl ModuleControl for FlipFlopModule {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn accept_pulse(&mut self, from: String, pulse: Pulse) {
        if pulse == Pulse::Low {
            self.incoming_pulses.insert(from, pulse);
        }
    }

    fn process_pulses(&mut self) -> Vec<(String, Pulse)> {
        let (new_state, outgoing) = self.incoming_pulses.iter().fold(
            (self.state, Vec::new()),
            |(state, mut outputs), (_, _)| {
                for id in self.connections.iter() {
                    if state {
                        outputs.push((id.clone(), Pulse::Low));
                    } else {
                        outputs.push((id.clone(), Pulse::High));
                    }
                }

                (!state, outputs)
            },
        );

        self.incoming_pulses.clear();
        self.state = new_state;

        outgoing
    }

    fn connections(&self) -> Vec<String> {
        self.connections.iter().cloned().collect_vec()
    }

    fn insert_upstream(&mut self, _name: String) {}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConjunctionModule {
    state: HashMap<String, Pulse>,
    id: String,
    connections: Vec<String>,
    incoming_pulses: HashMap<String, Pulse>,
}

impl ModuleControl for ConjunctionModule {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn accept_pulse(&mut self, from: String, pulse: Pulse) {
        self.incoming_pulses.insert(from, pulse);
    }

    fn process_pulses(&mut self) -> Vec<(String, Pulse)> {
        for (id, pulse) in self.incoming_pulses.iter() {
            self.state.insert(id.clone(), pulse.clone());
        }

        self.incoming_pulses.clear();

        if let Ok(pulse) = self.state.values().all_equal_value() {
            let new_pulse = if pulse == &Pulse::High {
                Pulse::Low
            } else {
                Pulse::High
            };

            self.connections
                .iter()
                .map(|id| (id.clone(), new_pulse.clone()))
                .collect_vec()
        } else {
            self.connections
                .iter()
                .map(|id| (id.clone(), Pulse::High))
                .collect_vec()
        }
    }

    fn connections(&self) -> Vec<String> {
        self.connections.iter().cloned().collect_vec()
    }

    fn insert_upstream(&mut self, name: String) {
        self.state.insert(name, Pulse::Low);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BroadcastModule {
    id: String,
    connections: Vec<String>,
    incoming_pulses: HashMap<String, Pulse>,
}

impl ModuleControl for BroadcastModule {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn accept_pulse(&mut self, from: String, pulse: Pulse) {
        self.incoming_pulses.insert(from, pulse);
    }

    fn process_pulses(&mut self) -> Vec<(String, Pulse)> {
        if self.incoming_pulses.len() > 1 {
            unreachable!()
        }

        let pulse = self.incoming_pulses.get("button").unwrap();

        let outputs = self
            .connections
            .iter()
            .map(|id| (id.clone(), pulse.clone()))
            .collect_vec();

        self.incoming_pulses.clear();

        outputs
    }

    fn connections(&self) -> Vec<String> {
        self.connections.iter().cloned().collect_vec()
    }

    fn insert_upstream(&mut self, _name: String) {}
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[enum_dispatch(ModuleControl)]
pub enum Module {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    Broadcast(BroadcastModule),
}

impl Module {
    fn is_conj(&self) -> bool {
        match self {
            Module::FlipFlop(_) => false,
            Module::Conjunction(_) => true,
            Module::Broadcast(_) => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleConfiguration {
    modules: HashMap<String, Module>,
}

impl ModuleConfiguration {
    fn push_button(&mut self) -> (Vec<(String, Pulse)>, &mut Self) {
        let mut q = VecDeque::new();
        let mut pulsed = Vec::new();

        q.push_front(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        pulsed.push(("broadcaster".to_string(), Pulse::Low));

        while let Some((from, id, pulse)) = q.pop_front() {
            if let Some(module) = self.modules.get_mut(&id) {
                module.accept_pulse(from.to_string(), pulse);

                let new_pulses = module
                    .process_pulses()
                    .into_iter()
                    .map(|(to, pulse)| (id.clone(), to, pulse))
                    .collect_vec();

                for (_, to, pulse) in new_pulses.iter() {
                    pulsed.push((to.clone(), pulse.clone()));
                }

                q.extend(new_pulses);
            }
        }

        (pulsed, self)
    }

    fn prune_to(&mut self, to: &str) {
        let mut keep = HashSet::new();
        let mut q = VecDeque::new();

        keep.insert(to.to_string());
        q.push_front(to.to_string());

        while let Some(to) = q.pop_front() {
            let not_seen = self
                .parents(&to)
                .iter()
                .filter(|p| !keep.contains(*p))
                .cloned()
                .collect_vec();

            keep.extend(not_seen.clone());
            q.extend(not_seen);
        }

        self.modules.retain(|name, _module| keep.contains(name));
    }

    fn parents(&self, target: &str) -> Vec<String> {
        self.modules
            .iter()
            .filter_map(|(name, module)| {
                if module.connections().contains(&target.to_string()) {
                    return Some(name.clone());
                }
                None
            })
            .collect_vec()
    }
}

fn parse_connections(s: &str) -> IResult<&str, Vec<String>> {
    separated_list1(tag(", "), map(alpha1, |s: &str| s.to_string()))(s)
}

fn parse_flip_flop(s: &str) -> IResult<&str, Module> {
    let (rest, id) = preceded(tag("%"), map(alpha1, |s: &str| s.to_string()))(s)?;
    let (rest, _) = tag(" -> ")(rest)?;
    let (rest, connections) = parse_connections(rest)?;

    Ok((
        rest,
        Module::from(FlipFlopModule {
            id,
            connections: connections.into_iter().collect(),
            state: false,
            incoming_pulses: HashMap::new(),
        }),
    ))
}

fn parse_conjunction(s: &str) -> IResult<&str, Module> {
    let (rest, id) = preceded(tag("&"), map(alpha1, |s: &str| s.to_string()))(s)?;
    let (rest, _) = tag(" -> ")(rest)?;
    let (rest, connections) = parse_connections(rest)?;

    Ok((
        rest,
        Module::from(ConjunctionModule {
            id,
            connections: connections.into_iter().collect(),
            state: HashMap::new(),
            incoming_pulses: HashMap::new(),
        }),
    ))
}

fn parse_broadcast(s: &str) -> IResult<&str, Module> {
    let (rest, id) = tag("broadcaster")(s)?;
    let (rest, _) = tag(" -> ")(rest)?;
    let (rest, connections) = parse_connections(rest)?;

    Ok((
        rest,
        Module::from(BroadcastModule {
            id: id.to_string(),
            connections: connections.into_iter().collect(),
            incoming_pulses: HashMap::new(),
        }),
    ))
}

fn parse_module(s: &str) -> IResult<&str, Module> {
    alt((parse_flip_flop, parse_conjunction, parse_broadcast))(s)
}

pub fn parse<'a>(data: &'a str) -> ModuleConfiguration {
    let (rest, modules): (&str, Vec<Module>) =
        separated_list1(newline, parse_module)(data).unwrap();

    let con_module_names = modules
        .iter()
        .filter(|module| module.is_conj())
        .map(|module| module.id())
        .collect_vec();

    let mut modules: HashMap<String, Module> =
        modules.into_iter().map(|m| (m.id().clone(), m)).collect();

    let mut connections: Vec<(String, String)> = Vec::new();

    for (name, module) in modules.iter() {
        for connection in module.connections() {
            if con_module_names.contains(&connection) {
                connections.push((name.clone(), connection));
            }
        }
    }

    for (from, to) in connections {
        modules.get_mut(&to).unwrap().insert_upstream(from);
    }

    assert!(rest.trim().is_empty());

    ModuleConfiguration { modules }
}

pub fn part1<'a>(mut config: ModuleConfiguration) -> i32 {
    let mut high_count = 0;
    let mut low_count = 0;

    for _ in 0..1000 {
        let (pulses, _) = config.push_button();
        for (_, pulse) in pulses {
            if pulse == Pulse::High {
                high_count += 1;
            } else {
                low_count += 1;
            }
        }
    }

    high_count * low_count
}

pub fn part2<'a>(config: ModuleConfiguration) -> usize {
    let rx_parents = config.parents("rx");
    let rx_source = rx_parents.iter().next().unwrap();

    let rx_source_sources = config.parents(rx_source);

    let cycles = rx_source_sources
        .iter()
        .map(|name| {
            let mut config = config.clone();
            config.prune_to(name);

            let cycle =
                pathfinding::directed::cycle_detection::floyd(config.clone(), |mut config| {
                    let (_, config) = config.push_button();
                    config.clone()
                });

            (name, cycle)
        })
        .collect_vec();

    cycles.iter().map(|(_, (size, _elem, _idx))| size).product()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT1: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    const SAMPLE_INPUT2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn test_sample_1_1() {
        let data = parse(&SAMPLE_INPUT1);
        assert_eq!(part1(data), 32000000);
    }

    #[test]
    fn test_sample_1_2() {
        let data = parse(&SAMPLE_INPUT2);
        assert_eq!(part1(data), 11687500);
    }

    generate_test! { 2023, 20, 1, 739960225}
    generate_test! { 2023, 20, 2, 231897990075517}
}
