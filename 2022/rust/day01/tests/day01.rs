#[test]
fn day01p01() {
    let input = util::read_input("../..", 2022, 1);
    let data = day01::parse(&input);
    assert_eq!(day01::part1(data), 64929)
}

#[test]
fn day01p02() {
    let input = util::read_input("../..", 2022, 1);
    let data = day01::parse(&input);
    assert_eq!(day01::part2(data), 193697)
}
