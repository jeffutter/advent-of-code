mod day01;
mod day02;
mod util;

fn main() {
    println!("Day01-01: {}", day01::part1(util::read_input(1)));
    println!("Day01-02: {}", day01::part2(util::read_input(1)));
    println!("Day02-01: {}", day02::part1(util::read_input(2)));
    println!("Day02-02: {}", day02::part2(util::read_input(2)));
}
