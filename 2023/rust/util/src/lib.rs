use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::US::Eastern;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use ureq::AgentBuilder;

pub fn read_input(year: i32, day: u32) -> String {
    let utc_now: DateTime<Utc> = chrono::Utc::now();
    let start = Eastern.with_ymd_and_hms(year, 12, day, 0, 0, 0).unwrap();

    if start >= utc_now {
        panic!("It's not time yet, can't fetch: {}", day);
    }

    let workspace_root = std::env!("CARGO_WORKSPACE_DIR");

    let file = Path::new(&workspace_root)
        .join(year.to_string())
        .join(format!("inputs/day{:0>2}", day));

    let cookie = Path::new(&workspace_root)
        .join(year.to_string())
        .join("cookie");

    if !file.exists() {
        let session_cookie = fs::read_to_string(cookie).expect("Cookie Not Found");

        let url = format!("https://adventofcode.com/{:0>4}/day/{}/input", year, day);

        let body = AgentBuilder::new()
            .build()
            .get(&url)
            .set("COOKIE", &format!("session={}", session_cookie.trim()))
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        let mut writer = File::create(&file).unwrap();
        write!(writer, "{}", body).unwrap();
    }

    std::fs::read_to_string(&file).unwrap()
}

pub extern crate num_format;

#[macro_export]
macro_rules! generate_main {
    ($($mod_name:ident)*) => {
        use util;
        use std::time::{Duration, Instant};
        use $crate::num_format::{Locale, ToFormattedString};

        fn measure_time<T, F: Fn() -> T>(func: F) -> (T, Duration) {
            let start = Instant::now();
            let res = func();
            let duration = start.elapsed();
            (res, duration)
        }

        $(
            use $mod_name;
        )*

        fn main() {

            $(
              let day_s = stringify!($mod_name).trim_start_matches("day");
              let day = day_s.parse::<u32>().unwrap();

              let (res, duration) = measure_time(|| {
                let input = util::read_input(2022, day);
                let parsed = $mod_name::parse(&input);
                $mod_name::part1(parsed)
              });
              println!("Day{:0>2}-01 {: >10}μs:\t{}", day, duration.as_micros().to_formatted_string(&Locale::en), res);

              let (res, duration) = measure_time(|| {
                let input = util::read_input(2022, day);
                let parsed = $mod_name::parse(&input);
                $mod_name::part2(parsed)
              });
              println!("Day{:0>2}-02 {: >10}μs:\t{}", day, duration.as_micros().to_formatted_string(&Locale::en), res);
            )*
        }
    };
}

#[macro_export]
macro_rules! generate_tests {
    ($year:expr, $mod: ident, $day:expr, $result1:expr, $result2:expr) => {
        use $mod::parse;
        use $mod::part1;
        use $mod::part2;

        #[test]
        fn test_part1() {
            let input = util::read_input($year, $day);
            let data = $mod::parse(&input);
            assert_eq!($mod::part1(data), $result1)
        }

        #[test]
        fn test_part2() {
            let input = util::read_input($year, $day);
            let data = $mod::parse(&input);
            assert_eq!($mod::part2(data), $result2)
        }
    };
}
