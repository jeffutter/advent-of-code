use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::preceded, IResult,
};

pub fn part1<'a>(instructions: Vec<Instruction>) -> i32 {
    instructions
        .iter()
        .fold((1, 1, vec![]), |(cycle, x, mut signal_strengths), op| {
            let next_cycle = cycle + op.cycles();
            let next_x = match op {
                Instruction::NOOP => x,
                Instruction::ADDX(v) => x + v,
            };

            let key_cycle = (cycle..next_cycle).find_map(|x| {
                if x == 20 || (x - 20) % 40 == 0 {
                    Some(x)
                } else {
                    None
                }
            });

            if let Some(cycle) = key_cycle {
                signal_strengths.push(cycle * x);
            }

            (next_cycle, next_x, signal_strengths)
        })
        .2
        .iter()
        .sum()
}

pub fn part2<'a>(instructions: Vec<Instruction>) -> String {
    let output = instructions
        .iter()
        .scan(1, |x, op| {
            let res = Some(vec![x.clone(); op.cycles().try_into().unwrap()]);
            match op {
                Instruction::NOOP => (),
                Instruction::ADDX(v) => *x += v,
            };
            res
        })
        .flatten()
        .enumerate()
        .map(|(c, sprite_x)| {
            let pixel = (c % 40) as i32;
            let sprite_pos = (sprite_x - 1)..=(sprite_x + 1);

            if sprite_pos.contains(&pixel) {
                "#"
            } else {
                "."
            }
        })
        .chunks(40)
        .into_iter()
        .map(|mut line| line.join(""))
        .collect_vec()
        .iter()
        .join("\n");

    format!("\n{}", output)
}

pub fn parse<'a>(data: &'a str) -> Vec<Instruction> {
    let (_, instructions) = parse_instuctions(data).unwrap();
    instructions
}

fn parse_instuctions(s: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, alt((noop, addx)))(s)
}

fn addx(s: &str) -> IResult<&str, Instruction> {
    let (rest, i) = preceded(tag("addx "), parser::signed_dig)(s)?;

    Ok((rest, Instruction::ADDX(i)))
}

fn noop(s: &str) -> IResult<&str, Instruction> {
    let (rest, _) = tag("noop")(s)?;
    Ok((rest, Instruction::NOOP))
}

#[derive(Debug)]
pub enum Instruction {
    NOOP,
    ADDX(i32),
}

impl Instruction {
    fn cycles(&self) -> i32 {
        match self {
            Instruction::NOOP => 1,
            Instruction::ADDX(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        let parsed = parse(input);
        let res = part1(parsed);
        assert_eq!(13140, res)
    }
}
