#[test]
fn day04p01() {
    let input = util::read_input("../..", 2022, 4);
    let data = day04::parse(&input);
    assert_eq!(day04::part1(data), 588)
}

#[test]
fn day04p02() {
    let input = util::read_input("../..", 2022, 4);
    let data = day04::parse(&input);
    assert_eq!(day04::part2(data), 911)
}
