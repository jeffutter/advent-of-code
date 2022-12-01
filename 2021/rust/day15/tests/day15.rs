const DATA: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

#[test]
fn day15p01_sample() {
    assert_eq!(day15::part1(DATA.to_string()), 40)
}

#[test]
fn day15p01() {
    assert_eq!(day15::part1(util::read_input("../..", 15)), 673)
}

#[test]
fn day15p02_sample() {
    assert_eq!(day15::part2(DATA.to_string()), 315)
}

#[test]
fn day15p02() {
    assert_eq!(day15::part2(util::read_input("../..", 15)), 2893)
}
