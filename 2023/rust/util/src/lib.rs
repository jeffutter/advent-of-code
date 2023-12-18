use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::US::Eastern;
use std::fmt::{Debug, Display};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use ureq::AgentBuilder;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos<T>
where
    T: Display,
{
    pub x: T,
    pub y: T,
}

impl<T> Pos<T>
where
    T: Copy
        + Display
        + num_traits::NumCast
        + std::ops::Sub<T, Output = T>
        + num_traits::ops::checked::CheckedSub
        + std::ops::Add<T, Output = T>
        + num_traits::ops::checked::CheckedAdd,
{
    pub fn new_unsigned(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn translate(&self, direction: &Direction) -> Option<Self> {
        let (x, y) = match direction {
            Direction::N => (
                self.x,
                self.y.checked_sub(&num_traits::cast::<usize, T>(1)?)?,
            ),
            Direction::E => (
                self.x.checked_add(&num_traits::cast::<usize, T>(1)?)?,
                self.y,
            ),
            Direction::S => (
                self.x,
                self.y.checked_add(&num_traits::cast::<usize, T>(1)?)?,
            ),
            Direction::W => (
                self.x.checked_sub(&num_traits::cast::<usize, T>(1)?)?,
                self.y,
            ),
        };

        Some(Self { x, y })
    }
}

impl<T> Pos<T>
where
    T: Copy
        + Display
        + num_traits::NumCast
        + num_traits::Signed
        + std::ops::Sub<T, Output = T>
        + std::ops::Add<T, Output = T>,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Pos<T>) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn distance(&self, other: &Pos<T>) -> f64 {
        let self_x: f64 = num_traits::cast(self.x).unwrap();
        let self_y: f64 = num_traits::cast(self.y).unwrap();
        let other_x: f64 = num_traits::cast(other.y).unwrap();
        let other_y: f64 = num_traits::cast(other.y).unwrap();

        ((other_x - self_x).abs().powi(2) + (other_y - self_y).abs().powi(2)).sqrt()
    }

    pub fn successors_4(&self) -> Vec<Pos<T>> {
        let &Pos { x, y } = self;
        let one: T = num_traits::cast(1).unwrap();
        vec![
            Pos::new(x - one, y),
            Pos::new(x + one, y),
            Pos::new(x, y - one),
            Pos::new(x, y + one),
        ]
    }
}

impl<T> Debug for Pos<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

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
                let input = util::read_input(2023, day);
                let parsed = $mod_name::parse(&input);
                $mod_name::part1(parsed)
              });
              println!("Day{:0>2}-01 {: >10}μs:\t{}", day, duration.as_micros().to_formatted_string(&Locale::en), res);

              let (res, duration) = measure_time(|| {
                let input = util::read_input(2023, day);
                let parsed = $mod_name::parse(&input);
                $mod_name::part2(parsed)
              });
              println!("Day{:0>2}-02 {: >10}μs:\t{}", day, duration.as_micros().to_formatted_string(&Locale::en), res);
            )*
        }
    };
}

pub extern crate paste;

#[macro_export]
macro_rules! generate_test {
    ($year:expr, $day:expr, $part:expr, $result:expr) => {
        // use $crate::paste;

        $crate::paste::item! {
            use super::*;

            #[test]
            fn [<test_ $part>]() {
                let input = util::read_input($year, $day);
                let data = parse(&input);
                assert_eq!([<part $part>](data), $result)
            }

        }
    };
}
