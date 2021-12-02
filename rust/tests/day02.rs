use aoc::*;

#[test]
fn day02p01_sample() {
    let data = "\
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2\
    ";
    assert_eq!(day02::part1(data.to_string()), 150)
}

#[test]
fn day02p01() {
    assert_eq!(day02::part1(util::read_input(2)), 1654760)
}

#[test]
fn day02p02_sample() {
    let data = "\
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2\
    ";
    assert_eq!(day02::part2(data.to_string()), 900)
}

#[test]
fn day02p02() {
    assert_eq!(day02::part2(util::read_input(2)), 1956047400)
}
