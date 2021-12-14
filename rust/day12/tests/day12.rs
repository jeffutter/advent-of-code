const DATA: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

#[test]
fn day12p01_sample() {
    assert_eq!(day12::part1(DATA.to_string()), 10)
}

#[test]
fn day12p01() {
    assert_eq!(day12::part1(util::read_input("../..", 12)), 4885)
}

#[test]
fn day12p02_sample() {
    assert_eq!(day12::part2(DATA.to_string()), 36)
}

#[test]
fn day12p02() {
    assert_eq!(day12::part2(util::read_input("../..", 12)), 117095)
}
