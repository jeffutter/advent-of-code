use std::collections::VecDeque;

const OPEN: &[char] = &['(', '<', '[', '{'];
const CLOSE: &[char] = &[')', '>', ']', '}'];

fn open_from_close(c: char) -> char {
    let idx = CLOSE.iter().position(|o| *o == c).unwrap();
    OPEN[idx]
}

fn close_from_open(c: char) -> char {
    let idx = OPEN.iter().position(|o| *o == c).unwrap();
    CLOSE[idx]
}

#[derive(PartialEq)]
enum NavigationValidation {
    COMPLETE,
    INVALID(char),
    INCOMPLETE(VecDeque<char>),
}

fn validate(data: &String) -> impl Iterator<Item = NavigationValidation> + '_ {
    data.lines().map(|line| {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut ch: Option<char> = None;

        for c in line.chars() {
            let f = stack.get(0);

            if !matches!(f, Some(_)) {
                stack.push_front(c);
                continue;
            }

            let first = f.unwrap();

            match (OPEN.contains(&first), OPEN.contains(&c)) {
                (false, _) => unimplemented!(),
                (true, true) => {
                    stack.push_front(c);
                }
                (true, false) => {
                    if *first == open_from_close(c) {
                        stack.pop_front();
                    } else {
                        ch = Some(c);
                        break;
                    }
                }
            }
        }

        match (ch, stack.len()) {
            (Some(c), _) => NavigationValidation::INVALID(c),
            (None, n) if n == 0 => NavigationValidation::COMPLETE,
            (None, n) if n != 0 => NavigationValidation::INCOMPLETE(stack),
            (None, _) => unimplemented!(),
        }
    })
}

pub fn part1(data: String) -> i64 {
    validate(&data)
        .filter(|c| matches!(c, NavigationValidation::INVALID(_)))
        .map(|s| {
            if let NavigationValidation::INVALID(s) = s {
                match s {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => {
                        unimplemented!()
                    }
                }
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(data: String) -> i64 {
    let mut values: Vec<i64> = validate(&data)
        .filter(|c| matches!(c, NavigationValidation::INCOMPLETE(_)))
        .map(|s| {
            if let NavigationValidation::INCOMPLETE(s) = s {
                s.iter()
                    .map(|c| match close_from_open(*c) {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => {
                            unimplemented!()
                        }
                    })
                    .fold(0, |acc, v| (acc * 5) + v)
            } else {
                unimplemented!()
            }
        })
        .collect();

    values.sort();

    values[values.len() / 2]
}
