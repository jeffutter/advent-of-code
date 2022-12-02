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
