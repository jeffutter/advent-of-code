#[test]
fn day03p01_sample() {
    let data = "\
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010\
    ";
    assert_eq!(day03::part1(data.to_string()), 198)
}

#[test]
fn day03p01() {
    assert_eq!(day03::part1(util::read_input("../../inputs", 3)), 1540244)
}

#[test]
fn day03p02_sample() {
    let data = "\
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010\
    ";
    assert_eq!(day03::part2(data.to_string()), 230)
}

#[test]
fn day03p02() {
    assert_eq!(day03::part2(util::read_input("../../inputs", 3)), 4203981)
}
