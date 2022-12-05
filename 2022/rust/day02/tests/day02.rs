#[test]
fn day02p01() {
    let input = util::read_input("../..", 2022, 2);
    let data = day02::parse(&input);
    assert_eq!(day02::part1(data), 15691)
}

#[test]
fn day02p02() {
    let input = util::read_input("../..", 2022, 2);
    let data = day02::parse(&input);
    assert_eq!(day02::part2(data), 12989)
}
