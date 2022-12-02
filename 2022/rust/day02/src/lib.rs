use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissors,
            _ => unimplemented!(),
        }
    }

    fn beaten_by(&self) -> Self {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn draw_by(&self) -> Self {
        self.clone()
    }

    fn beats(&self) -> Self {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        match (self, other) {
            (Choice::Rock, Choice::Paper) => Some(Ordering::Less),
            (Choice::Rock, Choice::Scissors) => Some(Ordering::Greater),
            (Choice::Paper, Choice::Scissors) => Some(Ordering::Less),
            (Choice::Paper, Choice::Rock) => Some(Ordering::Greater),
            (Choice::Scissors, Choice::Rock) => Some(Ordering::Less),
            (Choice::Scissors, Choice::Paper) => Some(Ordering::Greater),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&mut self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

#[derive(Debug)]
enum Action {
    Win,
    Lose,
    Draw,
}

impl Action {
    fn from_char(c: char) -> Action {
        match c {
            'X' => Action::Lose,
            'Y' => Action::Draw,
            'Z' => Action::Win,
            _ => unimplemented!(),
        }
    }

    fn to_choice(&self, their_play: Choice) -> Choice {
        match self {
            Action::Win => their_play.beaten_by(),
            Action::Lose => their_play.beats(),
            Action::Draw => their_play.draw_by(),
        }
    }
}

#[derive(Debug)]
struct Round {
    player1: Choice,
    player2: Choice,
}

impl Round {
    fn from_str<'a>(s: &'a str) -> Self {
        match s.chars().collect::<Vec<char>>()[..] {
            [p1, ' ', p2] => Self {
                player1: Choice::from_char(p1),
                player2: Choice::from_char(p2),
            },
            _ => unimplemented!("{}", s),
        }
    }

    fn cheat_from_str<'a>(s: &'a str) -> Self {
        match s.chars().collect::<Vec<char>>()[..] {
            [p1, ' ', p2] => {
                let their_choice = Choice::from_char(p1);
                let action = Action::from_char(p2);

                Self {
                    player1: Choice::from_char(p1),
                    player2: action.to_choice(their_choice),
                }
            }
            _ => unimplemented!("{}", s),
        }
    }

    fn grade(&self) -> i32 {
        self.player2.score() + self.winner().score()
    }

    fn winner(&self) -> Outcome {
        use Outcome::*;

        match self.player1.partial_cmp(&self.player2) {
            Some(Ordering::Equal) => Draw,
            Some(Ordering::Greater) => Lose,
            Some(Ordering::Less) => Win,
            None => unreachable!(),
        }
    }
}

pub fn part1(data: String) -> i32 {
    data.lines()
        .map(|l| Round::from_str(l))
        .map(|round| round.grade())
        .sum()
}

pub fn part2(data: String) -> i32 {
    data.lines()
        .map(|l| Round::cheat_from_str(l))
        .map(|round| round.grade())
        .sum()
}
