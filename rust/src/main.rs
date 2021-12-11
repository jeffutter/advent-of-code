use day01;
use day02;
use day03;
use day04;
use day05;
use day06;
use day07;
use day08;
use day09;
use util;

#[rustfmt::skip]
fn main() {
    println!("Day01-01: {}", day01::part1(util::read_input("../inputs", 1)));
    println!("Day01-02: {}", day01::part2(util::read_input("../inputs", 1)));
    println!("Day02-01: {}", day02::part1(util::read_input("../inputs", 2)));
    println!("Day02-02: {}", day02::part2(util::read_input("../inputs", 2)));
    println!("Day03-01: {}", day03::part1(util::read_input("../inputs", 3)));
    println!("Day03-02: {}", day03::part2(util::read_input("../inputs", 3)));
    println!("Day04-01: {}", day04::part1(util::read_input("../inputs", 4)));
    println!("Day04-02: {}", day04::part2(util::read_input("../inputs", 4)));
    println!("Day05-01: {}", day05::part1(util::read_input("../inputs", 5)));
    println!("Day05-02: {}", day05::part2(util::read_input("../inputs", 5)));
    println!("Day06-01: {}", day06::part1(util::read_input("../inputs", 6)));
    println!("Day06-02: {}", day06::part2(util::read_input("../inputs", 6)));
    println!("Day07-01: {}", day07::part1(util::read_input("../inputs", 7)));
    println!("Day07-02: {}", day07::part2(util::read_input("../inputs", 7)));
    println!("Day08-01: {}", day08::part1(util::read_input("../inputs", 8)));
    println!("Day08-02: {}", day08::part2(util::read_input("../inputs", 8)));
    println!("Day09-01: {}", day09::part1(util::read_input("../inputs", 9)));
    println!("Day09-02: {}", day09::part2(util::read_input("../inputs", 9)));
}
