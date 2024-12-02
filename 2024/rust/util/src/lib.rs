use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::US::Eastern;
use std::fmt::{Debug, Display};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::RangeBounds;
use std::path::Path;
use ureq::AgentBuilder;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction3 {
    N,
    E,
    S,
    W,
    I,
    O,
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
        self.translate_n(direction, 1)
    }

    pub fn translate_n(&self, direction: &Direction, n: usize) -> Option<Self> {
        let (x, y) = match direction {
            Direction::N => (
                self.x,
                self.y.checked_sub(&num_traits::cast::<usize, T>(n)?)?,
            ),
            Direction::E => (
                self.x.checked_add(&num_traits::cast::<usize, T>(n)?)?,
                self.y,
            ),
            Direction::S => (
                self.x,
                self.y.checked_add(&num_traits::cast::<usize, T>(n)?)?,
            ),
            Direction::W => (
                self.x.checked_sub(&num_traits::cast::<usize, T>(n)?)?,
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
        + std::ops::Add<T, Output = T>
        + std::cmp::PartialOrd,
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

    pub fn contained_by(&self, min: &Self, max: &Self) -> bool {
        self.x >= min.x && self.x <= max.x && self.y >= min.y && self.y <= max.y
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point3<T>
where
    T: Display,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T>
where
    T: Display
        + Copy
        + num_traits::NumCast
        + num_traits::ops::checked::CheckedSub
        + num_traits::ops::checked::CheckedAdd,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn translate(&self, direction: &Direction3) -> Option<Self> {
        self.translate_n(direction, 1)
    }

    pub fn translate_n(&self, direction: &Direction3, n: usize) -> Option<Self> {
        let (x, y, z) = match direction {
            Direction3::N => (
                self.x,
                self.y.checked_sub(&num_traits::cast::<usize, T>(n)?)?,
                self.z,
            ),
            Direction3::S => (
                self.x,
                self.y.checked_add(&num_traits::cast::<usize, T>(n)?)?,
                self.z,
            ),
            Direction3::W => (
                self.x.checked_sub(&num_traits::cast::<usize, T>(n)?)?,
                self.y,
                self.z,
            ),
            Direction3::E => (
                self.x.checked_add(&num_traits::cast::<usize, T>(n)?)?,
                self.y,
                self.z,
            ),
            Direction3::I => (
                self.x,
                self.y,
                self.z.checked_add(&num_traits::cast::<usize, T>(n)?)?,
            ),
            Direction3::O => (
                self.x,
                self.y,
                self.z.checked_sub(&num_traits::cast::<usize, T>(n)?)?,
            ),
        };

        Some(Self::new(x, y, z))
    }
}

impl<T> Debug for Point3<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{},{})", self.x, self.y, self.z))
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cube<T>
where
    T: Copy + Display,
{
    pub min: Point3<T>,
    pub max: Point3<T>,
}

impl<T> Cube<T>
where
    T: Copy
        + Display
        + Ord
        + num_traits::NumCast
        + num_traits::ops::checked::CheckedSub
        + num_traits::ops::checked::CheckedAdd,
{
    pub fn new(min: Point3<T>, max: Point3<T>) -> Self {
        Self { min, max }
    }

    pub fn intersect(&self, other: &Self) -> bool {
        self.max_x() >= other.min_x()
            && self.min_x() <= other.max_x()
            && self.max_y() >= other.min_y()
            && self.min_y() <= other.max_y()
            && self.max_z() >= other.min_z()
            && self.min_z() <= other.max_z()
    }

    pub fn z_range(&self) -> impl RangeBounds<T> {
        self.min_z()..=self.max_z()
    }

    pub fn y_range(&self) -> impl RangeBounds<T> {
        self.min_y()..=self.max_y()
    }

    pub fn x_range(&self) -> impl RangeBounds<T> {
        self.min_x()..=self.max_x()
    }

    pub fn collision(&self, other: &Self) -> bool {
        let overlap_z = other.z_range().contains(&self.min_z())
            || other.z_range().contains(&self.max_z())
            || self.z_range().contains(&other.min_z())
            || self.z_range().contains(&other.max_z());

        let overlap_x = other.x_range().contains(&self.min_x())
            || other.x_range().contains(&self.max_x())
            || self.x_range().contains(&other.min_x())
            || self.x_range().contains(&other.max_x());

        let overlap_y = other.y_range().contains(&self.min_y())
            || other.y_range().contains(&self.max_y())
            || self.y_range().contains(&other.min_y())
            || self.y_range().contains(&other.max_y());

        overlap_z && overlap_x && overlap_y
    }

    pub fn max_x(&self) -> T {
        self.min.x.max(self.max.x)
    }

    pub fn min_x(&self) -> T {
        self.min.x.min(self.max.x)
    }

    pub fn max_y(&self) -> T {
        self.min.y.max(self.max.y)
    }

    pub fn min_y(&self) -> T {
        self.min.y.min(self.max.y)
    }

    pub fn max_z(&self) -> T {
        self.min.z.max(self.max.z)
    }

    pub fn min_z(&self) -> T {
        self.min.z.min(self.max.z)
    }

    pub fn translate(&self, direction: &Direction3) -> Option<Self> {
        self.translate_n(direction, 1)
    }

    pub fn translate_n(&self, direction: &Direction3, n: usize) -> Option<Self> {
        let min = self.min.translate_n(direction, n)?;
        let max = self.max.translate_n(direction, n)?;
        Some(Self::new(min, max))
    }
}

impl<T> Debug for Cube<T>
where
    T: Display + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{:?}, {:?}]", self.min, self.max))
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
                let input = util::read_input(2024, day);
                let parsed = $mod_name::parse(&input);
                $mod_name::part1(parsed)
              });
              println!("Day{:0>2}-01 {: >10}μs:\t{}", day, duration.as_micros().to_formatted_string(&Locale::en), res);

              let (res, duration) = measure_time(|| {
                let input = util::read_input(2024, day);
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
