const DATA: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8
";

#[test]
fn day21p01_sample() {
    assert_eq!(day21::part1(DATA.to_string()), 739785)
}

#[test]
fn day21p01() {
    assert_eq!(day21::part1(util::read_input("../..", 21)), 734820)
}

#[test]
fn day21p02_sample() {
    assert_eq!(day21::part2(DATA.to_string()), 444356092776315)
}

#[test]
fn day21p02() {
    assert_eq!(day21::part2(util::read_input("../..", 21)), 193170338541590)
}
