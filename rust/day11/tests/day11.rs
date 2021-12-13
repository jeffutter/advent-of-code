const DATA: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

#[test]
fn day11p01_sample() {
    assert_eq!(day11::part1(DATA.to_string()), 1656)
}

#[test]
fn day11p01() {
    assert_eq!(day11::part1(util::read_input("../..", 11)), 1717)
}

#[test]
fn day11p02_sample() {
    assert_eq!(day11::part2(DATA.to_string()), 195)
}

#[test]
fn day11p02() {
    assert_eq!(day11::part2(util::read_input("../..", 11)), 476)
}
