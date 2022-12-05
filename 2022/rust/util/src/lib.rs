use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::US::Eastern;
use std::fs;
use std::fs::File;
use std::io::Write;
use ureq::AgentBuilder;

pub fn read_input(prefix: &str, year: i32, day: u32) -> String {
    let utc_now: DateTime<Utc> = chrono::Utc::now();
    let start = Eastern.with_ymd_and_hms(year, 12, day, 0, 0, 0).unwrap();

    if start >= utc_now {
        panic!("It's not time yet, can't fetch: {}", day);
    }

    let filename = [prefix, &format!("inputs/day{:0>2}", day)].join("/");
    let cookiepath = [prefix, "cookie"].join("/");

    if !std::path::Path::new(&filename).exists() {
        let session_cookie = fs::read_to_string(cookiepath).unwrap();

        let url = format!("https://adventofcode.com/{:0>4}/day/{}/input", 2022, day);

        let body = AgentBuilder::new()
            .build()
            .get(&url)
            .set("COOKIE", &format!("session={}", session_cookie.trim()))
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        let mut writer = File::create(&filename).unwrap();
        write!(writer, "{}", body).unwrap();
    }

    std::fs::read_to_string(&filename).unwrap()
}

#[macro_export]
macro_rules! generate_tests {
    ($year:expr, $mod: ident, $day:expr, $result1:expr, $result2:expr) => {
        use $mod::parse;
        use $mod::part1;
        use $mod::part2;

        #[test]
        fn test_part1() {
            let input = util::read_input("../..", $year, $day);
            let data = $mod::parse(&input);
            assert_eq!($mod::part1(data), $result1)
        }

        #[test]
        fn test_part2() {
            let input = util::read_input("../..", $year, $day);
            let data = $mod::parse(&input);
            assert_eq!($mod::part2(data), $result2)
        }
    };
}
