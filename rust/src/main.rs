mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod util;

fn main() {
    println!("Day01-01: {}", day01::part1(util::read_input(1)));
    println!("Day01-02: {}", day01::part2(util::read_input(1)));
    println!("Day02-01: {}", day02::part1(util::read_input(2)));
    println!("Day02-02: {}", day02::part2(util::read_input(2)));
    println!("Day03-01: {}", day03::part1(util::read_input(3)));
    println!("Day03-02: {}", day03::part2(util::read_input(3)));
    println!("Day04-01: {}", day04::part1(util::read_input(4)));
    println!("Day04-02: {}", day04::part2(util::read_input(4)));
    println!("Day05-01: {}", day05::part1(util::read_input(5)));
    println!("Day05-02: {}", day05::part2(util::read_input(5)));
}
