const DATA: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

#[test]
fn day14p01_sample() {
    assert_eq!(day14::part1(DATA.to_string()), 1588)
}

#[test]
fn day14p01() {
    assert_eq!(day14::part1(util::read_input("../..", 14)), 2797)
}

#[test]
fn day14p02_sample() {
    assert_eq!(day14::part2(DATA.to_string()), 2188189693529)
}

#[test]
fn day14p02() {
    assert_eq!(day14::part2(util::read_input("../..", 14)), 2926813379532)
}
