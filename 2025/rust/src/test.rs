#[macro_export]
macro_rules! generate_tests {
    ($year:expr, $day:expr, $result1:expr, $result2:expr) => {
        let mod = format!("day{:0>2}", day);

        #[test]
        fn part1() {
            let input = util::read_input("../..", $year, $day);
            let data = $mod::parse(&input);
            assert_eq!($mod::part1(data), $result1)
        }

        #[test]
        fn part1() {
            let input = util::read_input("../..", $year, $day);
            let data = $mod::parse(&input);
            assert_eq!($mod::part2(data), $result2)
        }

    };
}
