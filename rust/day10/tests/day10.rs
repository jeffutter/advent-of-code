const DATA: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

#[test]
fn day10p01_sample() {
    assert_eq!(day10::part1(DATA.to_string()), 26397)
}

#[test]
fn day10p01() {
    assert_eq!(day10::part1(util::read_input("../..", 10)), 316851)
}

#[test]
fn day10p02_sample() {
    assert_eq!(day10::part2(DATA.to_string()), 288957)
}

#[test]
fn day10p02() {
    assert_eq!(day10::part2(util::read_input("../..", 10)), 2182912364)
}
