use parser::separated_digits;
use pathfinding::prelude::dijkstra;
use winnow::{
    ModalResult, Parser,
    ascii::{newline, space1},
    combinator::{delimited, repeat, separated, terminated},
    token::one_of,
};

type InputType = Vec<Machine>;
type OutType = usize;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Button(Vec<usize>);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Machine {
    target_lights: Vec<bool>,
    lights: Vec<bool>,
    buttons: Vec<Button>,
    joltage_requirements: Vec<usize>,
    joltages: Vec<usize>,
}

impl Machine {
    fn press_button_light(&mut self, n: &usize) {
        for light in &self.buttons[*n].0 {
            self.lights[*light] = !self.lights[*light];
        }
    }

    fn is_complete_lights(&self) -> bool {
        self.lights == self.target_lights
    }

    fn press_button_joltage(&mut self, n: &usize) {
        for joltage in &self.buttons[*n].0 {
            self.joltages[*joltage] += 1;
        }
    }

    fn is_complete_joltage(&self) -> bool {
        self.joltages == self.joltage_requirements
    }

    fn is_over_joltage(&self) -> bool {
        self.joltages
            .iter()
            .zip(self.joltage_requirements.iter())
            .any(|(a, b)| a > b)
    }
}

fn parse_lights(s: &mut &str) -> ModalResult<Vec<bool>> {
    delimited(
        "[",
        repeat(
            1..,
            one_of(['#', '.']).map(|x| match x {
                '#' => true,
                '.' => false,
                _ => unreachable!(),
            }),
        ),
        "]",
    )
    .parse_next(s)
}

fn parse_buttons(s: &mut &str) -> ModalResult<Vec<Button>> {
    separated(
        1..,
        delimited("(", separated_digits(",").map(Button), ")"),
        space1,
    )
    .parse_next(s)
}

fn parse_joltage_requirements(s: &mut &str) -> ModalResult<Vec<usize>> {
    delimited("{", separated_digits(","), "}").parse_next(s)
}

fn parse_machine(s: &mut &str) -> ModalResult<Machine> {
    (
        parse_lights,
        space1,
        parse_buttons,
        space1,
        parse_joltage_requirements,
    )
        .map(
            |(target_lights, _, buttons, _, joltage_requirements)| Machine {
                target_lights: target_lights.clone(),
                buttons,
                joltage_requirements: joltage_requirements.clone(),
                lights: vec![false; target_lights.len()],
                joltages: vec![0; joltage_requirements.len()],
            },
        )
        .parse_next(s)
}

pub fn parse(data: &str) -> InputType {
    terminated(
        separated(1.., parse_machine, newline),
        repeat::<_, _, Vec<_>, _, _>(0.., newline),
    )
    .parse(data)
    .unwrap()
}

pub fn part1(input: InputType) -> OutType {
    input
        .iter()
        .map(|machine| {
            dijkstra(
                machine,
                |machine| {
                    let machine = machine.clone();
                    (0..machine.buttons.len()).map(move |n| {
                        let mut m = machine.clone();
                        m.press_button_light(&n);
                        (m, 1)
                    })
                },
                |machine| machine.is_complete_lights(),
            )
            .unwrap()
            .1
        })
        .sum()
}

pub fn part2(input: InputType) -> OutType {
    // input
    //     .iter()
    //     .map(|machine| {
    //         let (_path, n) = dijkstra(
    //             machine,
    //             |machine| {
    //                 let machine = machine.clone();
    //                 (0..machine.buttons.len())
    //                     .map(move |n| {
    //                         let mut m = machine.clone();
    //                         m.press_button_joltage(&n);
    //                         (m, 1)
    //                     })
    //                     .filter(|(machine, _)| !machine.is_over_joltage())
    //             },
    //             |machine| machine.is_complete_joltage(),
    //         )
    //         .unwrap();
    //
    //         n
    //     })
    //     .sum()
    1
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const TEST_INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    generate_test!(TEST_INPUT, 1, 7);

    generate_test!(TEST_INPUT, 2, 33);

    generate_test! { 2025, 10, 1, 428}
    generate_test! { 2025, 10, 2, 0}
}
