use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, newline},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug)]
pub struct Game {
    id: i32,
    draws: Vec<Draw>,
}

fn parse_draw(s: &str) -> IResult<&str, Draw> {
    let (rest, colors) = separated_list1(
        tag(","),
        tuple((
            delimited(multispace0, parser::from_dig, multispace0),
            alpha1,
        )),
    )(s)?;

    let mut hm = HashMap::new();
    for (count, color) in colors {
        hm.insert(color, count);
    }

    Ok((
        rest,
        Draw {
            red: *hm.get("red").unwrap_or(&0),
            blue: *hm.get("blue").unwrap_or(&0),
            green: *hm.get("green").unwrap_or(&0),
        },
    ))
}

fn parse_game(s: &str) -> IResult<&str, Game> {
    let (rest, id) = preceded(tag("Game "), terminated(parser::from_dig, tag(":")))(s)?;
    let (rest, draws) = separated_list1(tag(";"), parse_draw)(rest)?;

    Ok((rest, Game { id, draws }))
}

pub fn parse<'a>(data: &'a str) -> impl Iterator<Item = Game> {
    let (_, games) = separated_list1(newline, parse_game)(data).unwrap();
    games.into_iter()
}

pub fn part1<'a>(input: impl Iterator<Item = Game>) -> i32 {
    input
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.red <= 12 && draw.green <= 13 && draw.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}

pub fn part2<'a>(input: impl Iterator<Item = Game>) -> i32 {
    input
        .map(|game| {
            let min_red = game.draws.iter().map(|draw| draw.red).max().unwrap_or(1);
            let min_green = game.draws.iter().map(|draw| draw.green).max().unwrap_or(1);
            let min_blue = game.draws.iter().map(|draw| draw.blue).max().unwrap_or(1);

            min_red * min_blue * min_green
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test! { 2023, 2, 1, 2416}
    generate_test! { 2023, 2, 2, 63307}
}
