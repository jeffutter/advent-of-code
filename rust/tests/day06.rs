use aoc::*;

const DATA: &str = "3,4,3,1,2";

#[test]
fn day06p01_sample() {
    assert_eq!(day06::part1(DATA.to_string()), 5934)
}

#[test]
fn day06p01() {
    assert_eq!(day06::part1(util::read_input(6)), 361169)
}

#[test]
fn day06p02_sample() {
    assert_eq!(day06::part2(DATA.to_string()), 26984457539)
}

#[test]
fn day06p02() {
    assert_eq!(day06::part2(util::read_input(6)), 1634946868992)
}
