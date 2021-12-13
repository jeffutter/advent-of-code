const DATA: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

#[test]
fn day05p01_sample() {
    assert_eq!(day05::part1(DATA.to_string()), 5)
}

#[test]
fn day05p01() {
    assert_eq!(day05::part1(util::read_input("../..", 5)), 6564)
}

#[test]
fn day05p02_sample() {
    assert_eq!(day05::part2(DATA.to_string()), 12)
}

#[test]
fn day05p02() {
    assert_eq!(day05::part2(util::read_input("../..", 5)), 19172)
}
