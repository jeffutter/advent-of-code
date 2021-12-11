const DATA: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";

#[test]
fn day09p01_sample() {
    assert_eq!(day09::part1(DATA.to_string()), 15)
}

#[test]
fn day09p01() {
    assert_eq!(day09::part1(util::read_input("../../inputs", 9)), 566)
}

#[test]
fn day09p02_sample() {
    assert_eq!(day09::part2(DATA.to_string()), 1134)
}

#[test]
fn day09p02() {
    assert_eq!(day09::part2(util::read_input("../../inputs", 9)), 891684)
}
