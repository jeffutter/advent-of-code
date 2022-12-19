use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn part1(blueprints: Vec<Blueprint>) -> i32 {
    let states: Vec<State> = blueprints
        .into_iter()
        .map(|blueprint| State::new(blueprint, 24))
        .collect();

    let mut max_quality: i32 = 0;

    for state in states {
        let mut q: VecDeque<State> = VecDeque::new();
        q.push_front(state.clone());
        let mut bp_max_quality = 0;
        let mut seen = HashSet::new();
        let mut potential_best = 0;

        while let Some(state) = q.pop_front() {
            let remaining_moves = (state.max_steps - state.steps) as i32;
            let upper_bounds: i32 = state.geode
                + state.geode_robots * remaining_moves
                + (0i32..=remaining_moves).reduce(|acc, i| acc + i).unwrap();

            if upper_bounds <= potential_best {
                continue;
            }

            potential_best = potential_best.max(state.geode);

            if !seen.insert((
                state.steps,
                state.ore,
                state.clay,
                state.obsidian,
                state.geode,
                state.ore_robots,
                state.clay_robots,
                state.obsidian_robots,
                state.geode_robots,
            )) {
                continue;
            }

            bp_max_quality = bp_max_quality.max(state.quality_level());
            for next_state in state.next_states() {
                q.push_front(next_state);
            }
        }

        max_quality += bp_max_quality;
    }

    max_quality
}

pub fn part2(blueprints: Vec<Blueprint>) -> i32 {
    let states: Vec<State> = blueprints
        .into_iter()
        .map(|blueprint| State::new(blueprint, 32))
        .collect();

    let mut sum_geodes: i32 = 1;

    for state in states.iter().take(3) {
        let mut q: VecDeque<State> = VecDeque::new();
        q.push_front(state.clone());
        let mut bp_max_geodes = 0;
        let mut seen = HashSet::new();
        let mut potential_best = 0;

        while let Some(state) = q.pop_front() {
            let remaining_moves = (state.max_steps - state.steps) as i32;
            let upper_bounds: i32 = state.geode
                + state.geode_robots * remaining_moves
                + (0i32..=remaining_moves).reduce(|acc, i| acc + i).unwrap();

            if upper_bounds <= potential_best {
                continue;
            }

            potential_best = potential_best.max(state.geode);

            if !seen.insert((
                state.steps,
                state.ore,
                state.clay,
                state.obsidian,
                state.geode,
                state.ore_robots,
                state.clay_robots,
                state.obsidian_robots,
                state.geode_robots,
            )) {
                continue;
            }

            bp_max_geodes = bp_max_geodes.max(state.geode);
            for next_state in state.next_states() {
                q.push_front(next_state);
            }
        }

        sum_geodes *= bp_max_geodes;
    }

    sum_geodes
}

pub fn parse<'a>(data: &'a str) -> Vec<Blueprint> {
    let (rest, blueprints) = blueprints(data).unwrap();
    assert_eq!("", rest.trim());

    blueprints
}

fn blueprints(s: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(newline, blueprint)(s)
}

fn blueprint(s: &str) -> IResult<&str, Blueprint> {
    let (
        rest,
        (
            _,
            id,
            _,
            ore_robot_cost,
            _,
            clay_robot_cost,
            _,
            obsidian_robot_cost,
            _,
            geode_robot_cost,
            _,
        ),
    ) = tuple((
        tag("Blueprint "),
        parser::from_dig,
        tag(": Each ore robot costs "),
        costs,
        tag(". Each clay robot costs "),
        costs,
        tag(". Each obsidian robot costs "),
        costs,
        tag(". Each geode robot costs "),
        costs,
        tag("."),
    ))(s)?;

    let costs = ore_robot_cost
        .iter()
        .chain(clay_robot_cost.iter())
        .chain(obsidian_robot_cost.iter())
        .chain(geode_robot_cost.iter());

    let max_ore_cost = costs
        .clone()
        .filter_map(|cost| match cost {
            Cost::Ore(c) => Some(c),
            _ => None,
        })
        .max()
        .unwrap()
        .clone();

    let max_clay_cost = costs
        .clone()
        .filter_map(|cost| match cost {
            Cost::Clay(c) => Some(c),
            _ => None,
        })
        .max()
        .unwrap()
        .clone();

    let max_obsidian_cost = costs
        .filter_map(|cost| match cost {
            Cost::Obsidian(c) => Some(c),
            _ => None,
        })
        .max()
        .unwrap()
        .clone();

    Ok((
        rest,
        Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
            max_ore_cost,
            max_clay_cost,
            max_obsidian_cost,
        },
    ))
}

fn costs(s: &str) -> IResult<&str, Vec<Cost>> {
    separated_list1(tag(" and "), cost)(s)
}

fn cost(s: &str) -> IResult<&str, Cost> {
    let (rest, (n, _, t)) = tuple((parser::from_dig, tag(" "), alpha1))(s)?;

    let cost = match t {
        "ore" => Cost::Ore(n),
        "clay" => Cost::Clay(n),
        "obsidian" => Cost::Obsidian(n),
        _ => unimplemented!(),
    };

    Ok((rest, cost))
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Blueprint {
    id: i32,
    ore_robot_cost: Vec<Cost>,
    clay_robot_cost: Vec<Cost>,
    obsidian_robot_cost: Vec<Cost>,
    geode_robot_cost: Vec<Cost>,
    max_ore_cost: i32,
    max_clay_cost: i32,
    max_obsidian_cost: i32,
}

impl fmt::Debug for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BP: {} - OreCost: {}, ClayCost: {}, ObsidianCost: {}, GeodeCost: {}",
            self.id,
            self.ore_robot_cost
                .iter()
                .map(|cost| format!("{:?}", cost))
                .collect::<Vec<_>>()
                .join(","),
            self.clay_robot_cost
                .iter()
                .map(|cost| format!("{:?}", cost))
                .collect::<Vec<_>>()
                .join(","),
            self.obsidian_robot_cost
                .iter()
                .map(|cost| format!("{:?}", cost))
                .collect::<Vec<_>>()
                .join(","),
            self.geode_robot_cost
                .iter()
                .map(|cost| format!("{:?}", cost))
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Cost {
    Ore(i32),
    Clay(i32),
    Obsidian(i32),
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct State {
    blueprint: Blueprint,
    steps: usize,
    max_steps: usize,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl State {
    pub fn new(blueprint: Blueprint, max_steps: usize) -> Self {
        Self {
            blueprint,
            steps: 0,
            max_steps,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn next_states(&self) -> impl Iterator<Item = State> + '_ {
        self.available_commands().into_iter().map(|command| {
            let mut new_state = self.clone();
            new_state.step(command);
            new_state
        })
    }

    fn available_commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();
        if self.steps >= self.max_steps {
            return commands;
        }

        if self.can_afford(&self.blueprint.geode_robot_cost) {
            commands.push(Command::BuildGeodeRobot);
            return commands;
        }

        if self.can_afford(&self.blueprint.ore_robot_cost)
            && self.ore_robots + 1 <= self.blueprint.max_ore_cost
        {
            commands.push(Command::BuildOreRobot);
        }

        if self.can_afford(&self.blueprint.clay_robot_cost)
            && self.clay_robots + 1 <= self.blueprint.max_clay_cost
        {
            commands.push(Command::BuildClayRobot);
        }

        if self.can_afford(&self.blueprint.obsidian_robot_cost)
            && self.obsidian_robots + 1 <= self.blueprint.max_obsidian_cost
        {
            commands.push(Command::BuildObsidianRobot);
        }

        commands.push(Command::Wait);

        commands
    }

    fn step(&mut self, command: Command) {
        self.mine();
        self.process_command(command);
        self.steps += 1;
    }

    fn mine(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }

    fn process_command(&mut self, command: Command) {
        match command {
            Command::BuildOreRobot => {
                self.apply_costs(self.blueprint.ore_robot_cost.clone());
                self.ore_robots += 1;
            }
            Command::BuildClayRobot => {
                self.apply_costs(self.blueprint.clay_robot_cost.clone());
                self.clay_robots += 1;
            }
            Command::BuildObsidianRobot => {
                self.apply_costs(self.blueprint.obsidian_robot_cost.clone());
                self.obsidian_robots += 1;
            }
            Command::BuildGeodeRobot => {
                self.apply_costs(self.blueprint.geode_robot_cost.clone());
                self.geode_robots += 1;
            }
            Command::Wait => (),
        }
    }

    fn can_afford(&self, costs: &Vec<Cost>) -> bool {
        costs.iter().all(|cost| match cost {
            Cost::Ore(cost) => self.ore >= *cost,
            Cost::Clay(cost) => self.clay >= *cost,
            Cost::Obsidian(cost) => self.obsidian >= *cost,
        })
    }

    fn apply_costs(&mut self, costs: Vec<Cost>) {
        for cost in costs {
            match cost {
                Cost::Ore(cost) => self.ore -= cost,
                Cost::Clay(cost) => self.clay -= cost,
                Cost::Obsidian(cost) => self.obsidian -= cost,
            }
        }
    }

    fn quality_level(&self) -> i32 {
        self.geode * self.blueprint.id
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BP: {}, Step: {}/{}, ore_robots: {}, clay_robots: {}, obsidian_robots: {}, geode_robots: {}, ore: {}, clay: {}, obsiaidn: {}, geodes: {}", self.blueprint.id, self.steps, self.max_steps, self.ore_robots, self.clay_robots, self.obsidian_robots, self.geode_robots, self.obsidian, self.clay, self.obsidian, self.geode)
    }
}

enum Command {
    BuildOreRobot,
    BuildClayRobot,
    BuildObsidianRobot,
    BuildGeodeRobot,
    Wait,
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;

    #[test]
    fn test1() {
        let parsed = parse(INPUT);
        let res = part1(parsed);
        assert_eq!(33, res)
    }
}
