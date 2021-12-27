use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

fn parse(data: String) -> Vec<(i32, i32, i32)> {
    let (_rest, players) = separated_list1(
        line_ending,
        preceded(
            tag("Player "),
            separated_pair(
                parser::from_dig,
                tag(" starting position: "),
                parser::from_dig,
            ),
        ),
    )(&data)
    .unwrap();

    players.iter().map(|(pl, p)| (*pl, *p, 0)).collect()
}

pub fn part1(data: String) -> i32 {
    let mut players = parse(data);
    let mut die = (1..=100).cycle();
    let (mut player, mut rolls, mut losing_score) = (0, 0, 0);

    while let Some((_pl, p, score)) = players.get_mut(player) {
        let roll: i32 = (0..3).map(|_| die.next().unwrap()).sum();

        rolls += 3;

        *p = 1 + (*p + roll - 1) % 10;

        *score += *p;

        if *score >= 1000 {
            break;
        }
        losing_score = *score;

        player = if player == 0 { 1 } else { 0 };
    }

    losing_score * rolls
}

const FREQUENCIES: [(u8, u8); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn game(
    pos: i32,
    wins_per_round: &mut HashMap<u8, (i64, i64)>,
    round: u8,
    score: i32,
    multiplier: i64,
) {
    if score >= 21 {
        wins_per_round
            .entry(round)
            .and_modify(|(wins, _losses)| *wins += multiplier)
            .or_insert((multiplier, 0));
        return;
    }
    wins_per_round
        .entry(round)
        .and_modify(|(_wins, losses)| *losses += multiplier)
        .or_insert((0, multiplier));

    for (roll, times) in FREQUENCIES {
        let newpos = 1 + (pos + (roll as i32) - 1) % 10;
        game(
            newpos,
            wins_per_round,
            round + 1,
            score + newpos,
            multiplier * (times as i64),
        );
    }
}

pub fn part2(data: String) -> i64 {
    let players = parse(data);
    let (_, p1, _) = players[0];
    let (_, p2, _) = players[1];

    let mut p1_wins = HashMap::new();
    let mut p2_wins = HashMap::new();

    game(p1, &mut p1_wins, 0, 0, 1);
    game(p2, &mut p2_wins, 0, 0, 1);

    let max_p1_round = p1_wins.keys().max().unwrap();
    let max_p2_round = p2_wins.keys().max().unwrap();

    let mut p1_score: i64 = 0;
    let mut p2_score: i64 = 0;

    for i in 0..=*max_p1_round {
        if let Some((p1_wins, _p1_losses)) = p1_wins.get(&i) {
            if let Some((_p2_wins, p2_losses)) = i.checked_sub(1).and_then(|j| p2_wins.get(&j)) {
                p1_score += p1_wins * p2_losses
            }
        }
    }

    for i in 0..=*max_p2_round {
        if let Some((p2_wins, _p2_losses)) = p2_wins.get(&i) {
            if let Some((_p1_wins, p1_losses)) = p1_wins.get(&i) {
                p2_score += p2_wins * p1_losses
            }
        }
    }

    p1_score.max(p2_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "\
Player 1 starting position: 4
Player 2 starting position: 8"
            .to_string();

        assert_eq!(parse(data), vec![(1, 4, 0), (2, 8, 0)])
    }
}
