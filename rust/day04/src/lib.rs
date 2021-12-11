use parser::separated_digits;
use std::collections::HashMap;

use nom::{
    character::complete::line_ending,
    character::complete::space0,
    multi::many1,
    multi::{count, fold_many1},
    sequence::terminated,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct BingoCard {
    num_idx: HashMap<i32, (i32, i32)>,
    row_counts: HashMap<i32, i32>,
    column_counts: HashMap<i32, i32>,
}

impl BingoCard {
    fn new() -> BingoCard {
        BingoCard {
            num_idx: HashMap::new(),
            row_counts: HashMap::new(),
            column_counts: HashMap::new(),
        }
    }

    fn add(&mut self, col: i32, row: i32, v: i32) -> &mut Self {
        self.num_idx.insert(v, (col, row));
        Self::incr(&mut self.row_counts, col);
        Self::incr(&mut self.column_counts, col);

        self
    }

    fn incr(hm: &mut HashMap<i32, i32>, n: i32) {
        hm.entry(n).and_modify(|x| *x += 1).or_insert(1);
    }

    fn claim(&mut self, v: i32) -> bool {
        match self.num_idx.remove(&v) {
            None => false,
            Some((col, row)) => {
                self.column_counts.entry(col).and_modify(|x| *x -= 1);
                self.row_counts.entry(row).and_modify(|x| *x -= 1);

                self.column_counts[&col] == 0 || self.row_counts[&row] == 0
            }
        }
    }

    fn sum_remaining(&self) -> i32 {
        self.num_idx.keys().sum()
    }
}

fn sep_num_list(s: &str) -> IResult<&str, Vec<i32>> {
    preceded(space0, separated_digits)(s)
}

fn card(s: &str) -> IResult<&str, BingoCard> {
    let mut i = 0;

    let x = terminated(
        fold_many1(
            terminated(sep_num_list, line_ending),
            || BingoCard::new(),
            |mut card: BingoCard, row: Vec<i32>| {
                row.iter().enumerate().for_each(|(j, v)| {
                    card.add(j as i32, i, *v);
                });

                i += 1;
                card
            },
        ),
        line_ending,
    )(s);

    x
}

fn cards(s: &str) -> IResult<&str, Vec<BingoCard>> {
    many1(card)(s)
}

fn header(s: &str) -> IResult<&str, Vec<i32>> {
    terminated(sep_num_list, count(line_ending, 2))(s)
}

pub fn part1(data: String) -> i32 {
    let (_rest, (header, mut tabs)) = tuple((header, cards))(&data).unwrap();

    for i in header {
        for card in tabs.iter_mut() {
            if card.claim(i) {
                return card.sum_remaining() * i;
            }
        }
    }

    0
}

pub fn part2(data: String) -> i32 {
    let (_rest, (header, mut tabs)) = tuple((header, cards))(&data).unwrap();

    for i in header {
        let mut j = 0;
        let mut max = tabs.len();
        let mut last_found: Option<BingoCard> = None;
        while j < max {
            match tabs[j].claim(i) {
                true => {
                    last_found = Some(tabs.remove(j));
                    max -= 1;
                }
                false => {
                    j += 1;
                }
            }
        }
        if tabs.len() == 0 {
            return last_found.unwrap().sum_remaining() * i;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_parser() {
        let data = "14 21 17 24  4";

        assert_eq!(sep_num_list(&data).unwrap(), ("", vec![14, 21, 17, 24, 4]));

        let data = "1,2,3,4";

        assert_eq!(sep_num_list(&data).unwrap(), ("", vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_card_parser() {
        let data = "\
14 21 17 24  4
2  0 12  3  7

";

        let mut bc = BingoCard::new();

        bc.add(0, 0, 14)
            .add(1, 0, 21)
            .add(2, 0, 17)
            .add(3, 0, 24)
            .add(4, 0, 4)
            .add(0, 1, 2)
            .add(1, 1, 0)
            .add(2, 1, 12)
            .add(3, 1, 3)
            .add(4, 1, 7);

        let (_, bcc) = card(&data).unwrap();

        assert_eq!(bcc, bc);
    }
}
