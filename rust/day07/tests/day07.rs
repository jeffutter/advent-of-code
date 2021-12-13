const DATA: &str = "17,1,2,0,4,2,7,1,2,14";

#[test]
fn day07p01_sample() {
    assert_eq!(day07::part1(DATA.to_string()), 38)
}

#[test]
fn day07p01() {
    assert_eq!(day07::part1(util::read_input("../..", 7)), 336131)
}

#[test]
fn day07p02_sample() {
    assert_eq!(day07::part2(DATA.to_string()), 180)
}

#[test]
fn day07p02() {
    assert_eq!(day07::part2(util::read_input("../..", 7)), 92676646)
}
