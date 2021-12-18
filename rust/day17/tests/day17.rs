const DATA: &str = "\
target area: x=20..30, y=-10..-5
";

#[test]
fn day17p01_sample() {
    assert_eq!(day17::part1(DATA.to_string()), 45)
}

#[test]
fn day17p01() {
    assert_eq!(day17::part1(util::read_input("../..", 17)), 9870)
}

#[test]
fn day17p02_sample() {
    assert_eq!(day17::part2(DATA.to_string()), 112)
}

#[test]
fn day17p02() {
    assert_eq!(day17::part2(util::read_input("../..", 17)), 5523)
}
