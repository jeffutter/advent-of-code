use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    sequence::{preceded, tuple},
};
use parser::{separated_digits, FromDig};

type InputType = Computer;
type OutType = String;

#[derive(Debug, Clone)]
pub struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    program: Vec<i64>,
    instruction_pointer: usize,
}

impl Computer {
    fn step(&mut self) -> (bool, Option<i64>) {
        let mut output = None;

        let opcode = self.program[self.instruction_pointer];
        let literal_operand = self.program[self.instruction_pointer + 1];
        let combo_operand: i64 = match self.program[self.instruction_pointer + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => unimplemented!(),
            _ => unreachable!(),
        };

        match opcode {
            // adv
            0 => self.register_a /= 2i64.pow(combo_operand as u32),
            // bxl
            1 => self.register_b ^= literal_operand,
            // bst
            2 => self.register_b = combo_operand % 8,
            // jnz
            3 => {
                if self.register_a > 0 {
                    self.instruction_pointer = literal_operand as usize;
                    return (true, output);
                }
            }
            // bxc
            4 => self.register_b ^= self.register_c,
            // out
            5 => output = Some(combo_operand % 8),
            // bdv
            6 => self.register_b = self.register_a / (2i64.pow(combo_operand as u32)),
            // cdv
            7 => self.register_c = self.register_a / (2i64.pow(combo_operand as u32)),
            _ => unreachable!(),
        }

        self.instruction_pointer += 2;

        (self.instruction_pointer < self.program.len(), output)
    }

    fn run(&mut self) -> Vec<i64> {
        let mut output = Vec::new();

        loop {
            let (cont, out) = self.step();
            if let Some(out) = out {
                output.push(out);
            }
            if !cont {
                break;
            }
        }

        output
    }
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let (rest, computer) = map(
        tuple((
            preceded(tag("Register A: "), <i64 as FromDig>::from_dig),
            newline,
            preceded(tag("Register B: "), <i64 as FromDig>::from_dig),
            newline,
            preceded(tag("Register C: "), <i64 as FromDig>::from_dig),
            newline,
            newline,
            preceded(tag("Program: "), separated_digits(",")),
        )),
        |(a, _, b, _, c, _, _, program)| Computer {
            register_a: a,
            register_b: b,
            register_c: c,
            program,
            instruction_pointer: 0,
        },
    )(data)
    .unwrap();
    assert_eq!("", rest.trim());
    computer
}

#[allow(unused_variables)]
pub fn part1(mut computer: InputType) -> OutType {
    computer
        .run()
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[allow(unused_variables)]
pub fn part2(computer: InputType) -> i64 {
    let orig_computer = computer.clone();

    let mut factors = vec![0; orig_computer.program.len()];

    loop {
        let mut init_a = 0;
        for (i, f) in factors.iter().enumerate() {
            init_a += 8u64.pow(i as u32) * f
        }

        let mut computer = computer.clone();
        computer.register_a = init_a as i64;

        let output = computer.run();
        if output == computer.program {
            return init_a as i64;
        }

        for i in (0..computer.program.len()).rev() {
            if output.len() < i {
                factors[i] += 1;
                break;
            }
            if output[i] != computer.program[i] {
                factors[i] += 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#,
        1,
        "4,6,3,5,6,3,5,2,1,0"
    );

    generate_test!(
        r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#,
        2,
        117440
    );

    generate_test! { 2024, 17, 1, "1,6,7,4,3,0,5,0,6"}
    generate_test! { 2024, 17, 2, 216148338630253}
}
