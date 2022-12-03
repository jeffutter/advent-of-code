#[test]
fn day03p01() {
    let input = util::read_input("../..", 2022, 3);
    let data = day03::parse(&input);
    assert_eq!(day03::part1(data), 8109)
}

#[test]
fn day03p02() {
    let input = util::read_input("../..", 2022, 3);
    let data = day03::parse(&input);
    assert_eq!(day03::part2(data), 2738)
}
