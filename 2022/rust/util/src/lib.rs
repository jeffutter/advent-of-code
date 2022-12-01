use std::fs;
use std::fs::File;
use std::io::Write;
use ureq::AgentBuilder;

pub fn read_input(prefix: &str, day: usize) -> String {
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
