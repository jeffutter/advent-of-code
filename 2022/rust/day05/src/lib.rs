use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, digit1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::fmt;

pub fn part1((mut stacks, moves): (Stacks, Vec<Move>)) -> String {
    stacks
        .process_moves(moves, Stack::take_9000)
        .top_crates()
        .into_iter()
        .collect()
}

pub fn part2((mut stacks, moves): (Stacks, Vec<Move>)) -> String {
    stacks
        .process_moves(moves, Stack::take_9001)
        .top_crates()
        .into_iter()
        .collect()
}

pub fn parse<'a>(data: &'a str) -> (Stacks, Vec<Move>) {
    let (_, (stacks, moves)) = parse_data(data).unwrap();

    (stacks, moves)
}

fn parse_data(s: &str) -> IResult<&str, (Stacks, Vec<Move>)> {
    let (rest, stacks) = parse_stacks(s)?;
    let (rest, _) = take_until("move")(rest)?;
    let (rest, moves) = parse_moves(rest)?;

    Ok((rest, (stacks, moves)))
}

fn parse_stacks(s: &str) -> IResult<&str, Stacks> {
    let (rest, rows) = separated_list1(newline, parse_row)(s)?;

    let num_cols = rows.iter().map(|row| row.len()).max().unwrap();
    let num_rows = rows.len();

    let mut stacks = Stacks::new(num_cols, num_rows);

    for (r, row) in rows.iter().rev().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if let Some(col) = col {
                stacks.insert(r, c, *col);
            }
        }
    }
    Ok((rest, stacks))
}

fn parse_row(s: &str) -> IResult<&str, Vec<Option<char>>> {
    let (rest, crates) = separated_list1(tag(" "), parse_crate)(s)?;
    Ok((rest, crates))
}

fn parse_crate(s: &str) -> IResult<&str, Option<char>> {
    let (rest, c) = alt((
        map(tag("   "), |_| None),
        map(tuple((tag("["), anychar, tag("]"))), |(_, c, _)| Some(c)),
    ))(s)?;

    Ok((rest, c))
}

fn parse_moves(s: &str) -> IResult<&str, Vec<Move>> {
    let (rest, moves) = separated_list1(newline, parse_move)(s)?;
    Ok((rest, moves))
}

fn parse_move(s: &str) -> IResult<&str, Move> {
    let (rest, (_, num, _, from, _, to)) = tuple((
        tag("move "),
        parser::from_dig,
        tag(" from "),
        map(digit1, |s: &str| s.parse().unwrap()),
        tag(" to "),
        map(digit1, |s: &str| s.parse().unwrap()),
    ))(s)?;
    Ok((rest, Move { num, from, to }))
}

#[derive(Debug)]
pub struct Stack(Vec<Option<char>>);

impl Stack {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::new();
        v.resize(size, None);
        Self(v)
    }

    fn insert(&mut self, row: usize, val: char) -> &mut Self {
        self.0[row] = Some(val);
        self
    }

    fn insert_last(&mut self, val: char) -> &mut Self {
        let mut row = 0;
        let mut found = false;
        while row <= (self.0.len() - 1) {
            if let None = self.0[row] {
                found = true;
                break;
            }
            row += 1;
        }

        if found {
            self.insert(row, val);
        } else {
            self.0.push(Some(val))
        }

        self
    }

    fn take_9000(&mut self, mut num: i32) -> Vec<char> {
        let mut res: Vec<char> = Vec::new();

        while num > 0 {
            let mut row = self.0.len() - 1;
            loop {
                if let Some(val) = self.0[row] {
                    self.0[row] = None;
                    res.push(val);
                    num -= 1;
                    if num == 0 {
                        break;
                    }
                }
                if row == 0 {
                    break;
                }
                row -= 1;
            }
        }

        res
    }

    fn take_9001(&mut self, mut num: i32) -> Vec<char> {
        let mut res: Vec<char> = Vec::new();

        while num > 0 {
            let mut row = self.0.len() - 1;
            loop {
                if let Some(val) = self.0[row] {
                    self.0[row] = None;
                    res.insert(0, val);
                    num -= 1;
                    if num == 0 {
                        break;
                    }
                }
                if row == 0 {
                    break;
                }
                row -= 1;
            }
        }

        res
    }

    fn get_top(&self) -> Option<char> {
        match self.0.iter().rev().find(|v| Option::is_some(v)).cloned() {
            Some(v) => v,
            None => None,
        }
    }
}

pub struct Stacks(Vec<Stack>);

impl fmt::Debug for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        if let Some(tallest) = self.0.iter().map(|x| x.0.len()).max() {
            for i in (0..=tallest).rev() {
                for stack in &self.0 {
                    match stack.0.get(i) {
                        None => write!(f, " [ ]")?,
                        Some(None) => write!(f, " [x]")?,
                        Some(Some(c)) => write!(f, " [{}]", c)?,
                    };
                }
                write!(f, "\n")?;
            }
        } else {
            write!(f, "[EMPTY]")?;
        }

        Ok(())
    }
}

impl Stacks {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut v = Vec::new();
        v.resize_with(cols, || Stack::new(rows));
        Self(v)
    }

    fn insert(&mut self, row: usize, col: usize, val: char) -> &mut Self {
        self.0.get_mut(col).unwrap().insert(row, val);
        self
    }

    fn process_move(&mut self, m: Move, take: fn(&mut Stack, i32) -> Vec<char>) -> &mut Self {
        let Move { num, from, to } = m;
        let to = to - 1;
        let from = from - 1;

        let taken = {
            let from_stack = self.0.get_mut(from).unwrap();

            take(from_stack, num).into_iter().collect::<Vec<char>>()
        };

        let to_stack = self.0.get_mut(to).unwrap();
        for c in taken {
            to_stack.insert_last(c);
        }

        self
    }

    fn process_moves(
        &mut self,
        moves: Vec<Move>,
        take: fn(&mut Stack, i32) -> Vec<char>,
    ) -> &mut Self {
        for m in moves {
            self.process_move(m, take);
        }

        self
    }

    fn top_crates(&mut self) -> Vec<char> {
        let mut res: Vec<char> = vec![];

        for c in &self.0 {
            if let Some(char) = c.get_top() {
                res.push(char.clone());
            }
        }

        res
    }
}

#[derive(Debug)]
pub struct Move {
    num: i32,
    from: usize,
    to: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
        "#;
        let parsed = parse(data);
        let res = part1(parsed);
        assert_eq!(res, "CMZ")
    }
}
