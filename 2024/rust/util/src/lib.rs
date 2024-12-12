use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::US::Eastern;
use num_traits::AsPrimitive;
use std::fmt::Write as _;
use std::fmt::{Debug, Display};
use std::fs;
use std::fs::File;
use std::io::Write as _;
use std::marker::PhantomData;
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
pub enum Direction8 {
    NW,
    N,
    NE,
    E,
    W,
    SW,
    S,
    SE,
}

impl Direction8 {
    pub fn all() -> impl Iterator<Item = &'static Direction8> {
        [
            Direction8::NW,
            Direction8::N,
            Direction8::NE,
            Direction8::E,
            Direction8::W,
            Direction8::SW,
            Direction8::S,
            Direction8::SE,
        ]
        .iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction3D {
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
    T: Display,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pos<T>
where
    T: Copy
        + Display
        + num_traits::NumCast
        + num_traits::ops::checked::CheckedAdd
        + num_traits::ops::checked::CheckedSub
        + std::cmp::PartialEq
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>,
{
    pub fn successors_4_unsigned(&self) -> Vec<Self> {
        let &Self { x, y } = self;
        let one: T = num_traits::cast(1).unwrap();
        let zero: T = num_traits::cast(0).unwrap();
        match (x, y) {
            (x, y) if x == zero && y == zero => {
                vec![Self::new(x + one, y), Self::new(x, y + one)]
            }
            (x, _) if x == zero => vec![
                Self::new(x + one, y),
                Self::new(x, y - one),
                Self::new(x, y + one),
            ],
            (_, y) if y == zero => vec![
                Self::new(x - one, y),
                Self::new(x + one, y),
                Self::new(x, y + one),
            ],
            (_, _) => vec![
                Self::new(x - one, y),
                Self::new(x + one, y),
                Self::new(x, y - one),
                Self::new(x, y + one),
            ],
        }
    }

    pub fn successors_8_unsigned(&self) -> Vec<Self> {
        let &Self { x, y } = self;
        let one: T = num_traits::cast(1).unwrap();
        let zero: T = num_traits::cast(0).unwrap();
        match (x, y) {
            (x, y) if x == zero && y == zero => vec![
                Self::new(x + one, y),
                Self::new(x, y + one),
                Self::new(x + one, y + one),
            ],
            (x, _) if x == zero => vec![
                Self::new(x, y - one),
                Self::new(x + one, y - one),
                Self::new(x + one, y),
                Self::new(x, y + one),
                Self::new(x + one, y + one),
            ],
            (_, y) if y == zero => vec![
                Self::new(x - one, y),
                Self::new(x + one, y),
                Self::new(x - one, y + one),
                Self::new(x, y + one),
                Self::new(x + one, y + one),
            ],
            (_, _) => vec![
                Self::new(x - one, y - one),
                Self::new(x, y - one),
                Self::new(x + one, y - one),
                Self::new(x - one, y),
                Self::new(x + one, y),
                Self::new(x - one, y + one),
                Self::new(x, y + one),
                Self::new(x + one, y + one),
            ],
        }
    }

    pub fn translate(&self, direction: &Direction) -> Option<Self> {
        self.translate_n(direction, 1)
    }

    pub fn translate8(&self, direction: &Direction8) -> Option<Self> {
        match direction {
            Direction8::NW => self
                .translate(&Direction::N)
                .and_then(|p| p.translate(&Direction::W)),
            Direction8::N => self.translate(&Direction::N),
            Direction8::NE => self
                .translate(&Direction::N)
                .and_then(|p| p.translate(&Direction::E)),
            Direction8::E => self.translate(&Direction::E),
            Direction8::W => self.translate(&Direction::W),
            Direction8::SW => self
                .translate(&Direction::S)
                .and_then(|p| p.translate(&Direction::W)),
            Direction8::S => self.translate(&Direction::S),
            Direction8::SE => self
                .translate(&Direction::S)
                .and_then(|p| p.translate(&Direction::E)),
        }
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
        + std::cmp::PartialOrd
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>,
{
    pub fn manhattan_distance(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let self_x: f64 = num_traits::cast(self.x).unwrap();
        let self_y: f64 = num_traits::cast(self.y).unwrap();
        let other_x: f64 = num_traits::cast(other.y).unwrap();
        let other_y: f64 = num_traits::cast(other.y).unwrap();

        ((other_x - self_x).abs().powi(2) + (other_y - self_y).abs().powi(2)).sqrt()
    }

    pub fn successors_4(&self) -> Vec<Self> {
        let &Self { x, y } = self;
        let one: T = num_traits::cast(1).unwrap();
        vec![
            Self::new(x - one, y),
            Self::new(x + one, y),
            Self::new(x, y - one),
            Self::new(x, y + one),
        ]
    }

    pub fn successors_8(&self) -> Vec<Self> {
        let &Self { x, y } = self;
        let one: T = num_traits::cast(1).unwrap();
        vec![
            Self::new(x - one, y - one),
            Self::new(x, y - one),
            Self::new(x + one, y - one),
            Self::new(x - one, y),
            Self::new(x + one, y),
            Self::new(x - one, y + one),
            Self::new(x, y + one),
            Self::new(x + one, y + one),
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

    pub fn translate(&self, direction: &Direction3D) -> Option<Self> {
        self.translate_n(direction, 1)
    }

    pub fn translate_n(&self, direction: &Direction3D, n: usize) -> Option<Self> {
        let (x, y, z) = match direction {
            Direction3D::N => (
                self.x,
                self.y.checked_sub(&num_traits::cast::<usize, T>(n)?)?,
                self.z,
            ),
            Direction3D::S => (
                self.x,
                self.y.checked_add(&num_traits::cast::<usize, T>(n)?)?,
                self.z,
            ),
            Direction3D::W => (
                self.x.checked_sub(&num_traits::cast::<usize, T>(n)?)?,
                self.y,
                self.z,
            ),
            Direction3D::E => (
                self.x.checked_add(&num_traits::cast::<usize, T>(n)?)?,
                self.y,
                self.z,
            ),
            Direction3D::I => (
                self.x,
                self.y,
                self.z.checked_add(&num_traits::cast::<usize, T>(n)?)?,
            ),
            Direction3D::O => (
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

    pub fn translate(&self, direction: &Direction3D) -> Option<Self> {
        self.translate_n(direction, 1)
    }

    pub fn translate_n(&self, direction: &Direction3D, n: usize) -> Option<Self> {
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

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BitMap<T> {
    pub cols: Vec<u128>,
    pub rows: Vec<u128>,
    phantom: PhantomData<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> BitMap<T>
where
    T: Display + AsPrimitive<usize>,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cols: vec![0; width],
            rows: vec![0; height],
            width,
            height,
            phantom: PhantomData,
        }
    }

    pub fn present(&self, &Pos { x, y }: &Pos<T>) -> bool {
        let y = y.as_();
        let x = x.as_();
        if let Some(col) = self.cols.get(x) {
            return col & (1 << y) > 0;
        }
        false
    }

    pub fn set(&mut self, &Pos { x, y }: &Pos<T>) {
        let y = y.as_();
        let x = x.as_();
        let col = self.cols.get_mut(x).unwrap();
        *col |= 1 << y;
        let row = self.rows.get_mut(y).unwrap();
        *row |= 1 << x;
    }

    pub fn unset(&mut self, &Pos { x, y }: &Pos<T>) {
        let y = y.as_();
        let x = x.as_();

        let col = self.cols.get_mut(x).unwrap();
        *col &= !(1 << y);
        let row = self.rows.get_mut(y).unwrap();
        *row &= !(1 << x);
    }

    pub fn iter(&self) -> BitMapIterator<T> {
        BitMapIterator {
            row_idx: 0,
            rows: self.rows.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T> Debug for BitMap<T>
where
    T: Display + AsPrimitive<usize> + std::convert::From<usize>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.width {
            for x in 0..self.height {
                let point: Pos<T> = Pos::new(x.into(), y.into());
                if self.present(&point) {
                    f.write_char('x')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct BitMapIterator<T> {
    row_idx: usize,
    rows: Vec<u128>,
    phantom: PhantomData<T>,
}

impl<T> Iterator for BitMapIterator<T>
where
    T: Display + std::convert::TryFrom<u32> + std::convert::TryFrom<usize> + std::fmt::Debug,
    <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<u32>>::Error: std::fmt::Debug,
{
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.row_idx >= self.rows.len() {
                return None;
            }

            let row = self.rows[self.row_idx];
            let least_sig = row & row.wrapping_neg();
            let res = least_sig.trailing_zeros();
            if res == 128 {
                self.row_idx += 1;
                continue;
            }
            self.rows[self.row_idx] &= !(least_sig);

            let pos = Pos::new(
                T::try_from(res).unwrap(),
                T::try_from(self.row_idx).unwrap(),
            );

            return Some(pos);
        }
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
    ($input:expr, $part:expr, $result:expr) => {
        $crate::paste::item! {
            use super::*;

            #[test]
            fn [<example_ $part>]() {
                let data = parse($input);
                assert_eq!([<part $part>](data), $result)
            }

        }
    };

    ($year:expr, $day:expr, $part:expr, $result:expr) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitmap_iter() {
        let mut bm = BitMap::new(10, 10);
        bm.set(&Pos::new(5, 5));
        assert_eq!(vec![Pos::new(5, 5)], bm.iter().collect::<Vec<_>>());
        bm.set(&Pos::new(1, 1));
        assert_eq!(
            vec![Pos::new(1, 1), Pos::new(5, 5)],
            bm.iter().collect::<Vec<_>>()
        );
        bm.set(&Pos::new(1, 2));
        assert_eq!(
            vec![Pos::new(1, 1), Pos::new(1, 2), Pos::new(5, 5)],
            bm.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn bitmap_unset() {
        let mut bm = BitMap::new(10, 10);
        let pos = Pos::new(5, 5);
        bm.set(&pos);
        assert!(bm.present(&pos));
        bm.unset(&pos);
        assert!(!bm.present(&pos));
        assert!(!bm.present(&pos));
    }

    #[test]
    fn bitmap() {
        let map: BitMap<usize> = BitMap {
            rows: vec![
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                67110912,
                67110912,
                17112763393,
                8522828801,
                255211775190703847597530955590737594391,
                255211775190703847597530955590989253855,
                170141183460469231731687303723366747903,
                170141183460469231731687303716941074431,
                170141183467510015705122904033651068927,
                255211775200452625406903325240787472383,
                319014718990778318323029118750939942911,
                297747071057987550599202196984910843871,
                297747071056904353064827489243333922687,
                297747071055975898035363454037159579647,
                127605887595351923798765477786913083391,
                170141183460469231731687303715884109823,
                4095,
                4095,
                2047,
                1532,
                184,
                48,
                0,
                0,
            ],
            cols: vec![
                42202988866171078964695874163900940288,
                19938419936773738093557105904205168640,
                41206067869332392060018018868690681856,
                34559927890407812695498983567288958976,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                77095223755525120628420809496259985408,
                337623910929368631717566993311207522304,
                338870062175416990348414312430220345344,
                338620831926207318622244848606417780736,
                339950059921992234495148655666698125312,
                339950059921992234495148655666698125312,
                68787548781869396422772015369507831808,
                31569164899891751981465417681658183680,
                10301516967333098015004504717172670464,
                4319990986300976586937372945911119872,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                106338239662793269832304564822427566080,
                148873535527910577765226390751398592515,
                297747071055821155530452781502797185025,
                319014718988379809496913694467282698241,
                319014718988379809496913694467282698240,
                42535295865117307932921825928971026432,
                63802943797675961899382738893456539648,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                85070591730234615865843651857942052864,
                255211775190703847597530955573826158599,
                301734755043175903149164202683638218759,
                338953138925153547590470800371487866891,
                338953138925153547590470800371487867007,
                338953138925153547590470800371487867007,
                338953138925153547590470800371487867135,
                337623910929368631717566993311207522815,
                338953138925153547590470800371487867903,
                164824271477329568240072075474762728447,
                337623910929368631717566993311207522559,
                337623910929368631717566993311207522814,
                329648542954659136480144150949525455103,
                334965454937798799971759379190646833279,
                338620831926207318622244848606417780991,
                340199290171201906221318119490500689983,
            ],
            width: 140,
            height: 140,
            phantom: PhantomData,
        };

        let present = map
            .iter()
            .map(|p| {
                println!("{:?}", p);
                (p.clone(), map.present(&p))
            })
            .collect::<Vec<_>>();
        println!("{:?}", present);

        assert!(map.iter().any(|p| p == Pos::new(11, 116)));
        assert!(map.present(&Pos::new(11, 116)));
    }
}
