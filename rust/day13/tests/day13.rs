const DATA: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

#[test]
fn day13p01_sample() {
    assert_eq!(day13::part1(DATA.to_string()), 17)
}

#[test]
fn day13p01() {
    assert_eq!(day13::part1(util::read_input("../..", 13)), 689)
}

#[test]
fn day13p02_sample() {
    assert_eq!(
        day13::part2(DATA.to_string()),
        "
#####
#...#
#...#
#...#
#####
"
    )
}

#[test]
fn day13p02() {
    assert_eq!(
        day13::part2(util::read_input("../..", 13)),
        "
###..#....###...##....##..##..#....#..#
#..#.#....#..#.#..#....#.#..#.#....#..#
#..#.#....###..#.......#.#....#....#..#
###..#....#..#.#.......#.#.##.#....#..#
#.#..#....#..#.#..#.#..#.#..#.#....#..#
#..#.####.###...##...##...###.####..##.
"
    )
}
