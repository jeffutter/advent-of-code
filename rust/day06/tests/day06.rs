const DATA: &str = "3,4,3,1,2";

#[test]
fn day06p01_sample() {
    assert_eq!(day06::part1(DATA.to_string()), 5934u64)
}

#[test]
fn day06p01() {
    assert_eq!(day06::part1(util::read_input("../..", 6)), 361169u64)
}

#[test]
fn day06p02_sample() {
    assert_eq!(day06::part2(DATA.to_string()), 26984457539u64)
}

#[test]
fn day06p02() {
    assert_eq!(day06::part2(util::read_input("../..", 6)), 1634946868992u64)
}
