#[test]
fn day05p01() {
    let input = util::read_input("../..", 2022, 5);
    let data = day05::parse(&input);
    assert_eq!(day05::part1(data), 0)
}

#[test]
fn day05p02() {
    let input = util::read_input("../..", 2022, 5);
    let data = day05::parse(&input);
    assert_eq!(day05::part2(data), 0)
}
