use std::{fmt::Display, str::FromStr};

use num_traits::{Signed, Unsigned};
use util::{AbsDiff, Point3, Pos};
use winnow::{
    Parser,
    ascii::digit1,
    combinator::{alt, opt, repeat, separated},
    error::Result,
    token::literal,
};

pub trait FromDig {
    type Num;

    fn from_dig(input: &mut &str) -> Result<Self::Num>;
}

impl FromDig for u32 {
    type Num = u32;
    fn from_dig(input: &mut &str) -> Result<Self::Num> {
        digit1.try_map(|s: &str| s.parse::<u32>()).parse_next(input)
    }
}

impl FromDig for i32 {
    type Num = i32;
    fn from_dig(input: &mut &str) -> Result<Self::Num> {
        digit1.try_map(|s: &str| s.parse::<i32>()).parse_next(input)
    }
}

impl FromDig for i64 {
    type Num = i64;
    fn from_dig(input: &mut &str) -> Result<Self::Num> {
        digit1.try_map(|s: &str| s.parse::<i64>()).parse_next(input)
    }
}

impl FromDig for i128 {
    type Num = i128;
    fn from_dig(input: &mut &str) -> Result<Self::Num> {
        digit1
            .try_map(|s: &str| s.parse::<i128>())
            .parse_next(input)
    }
}

impl FromDig for usize {
    type Num = usize;
    fn from_dig(input: &mut &str) -> Result<Self::Num> {
        digit1
            .try_map(|s: &str| s.parse::<usize>())
            .parse_next(input)
    }
}

pub fn point<'a, T>(sep: &str) -> impl FnMut(&mut &'a str) -> Result<Pos<T>> + '_
where
    T: Display + FromDig<Num = T>,
{
    move |input: &mut &'a str| {
        let l = <T as FromDig>::from_dig(input)?;
        literal(sep).parse_next(input)?;
        let r = <T as FromDig>::from_dig(input)?;
        Ok(Pos::new(l, r))
    }
}

pub fn signed_point<'a, T>(sep: &str) -> impl FnMut(&mut &'a str) -> Result<Pos<T>> + '_
where
    T: Display + Signed + FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    move |input: &mut &'a str| {
        let l = signed_dig(input)?;
        literal(sep).parse_next(input)?;
        let r = signed_dig(input)?;
        Ok(Pos::new(l, r))
    }
}

pub fn point3<'a, T>(sep: &str) -> impl FnMut(&mut &'a str) -> Result<Point3<T>> + '_
where
    T: Display
        + FromDig<Num = T>
        + std::marker::Copy
        + num_traits::NumCast
        + num_traits::CheckedSub
        + num_traits::CheckedAdd
        + num_traits::CheckedMul
        + AbsDiff<T, T>,
{
    move |input: &mut &'a str| {
        let (x, _, y, _, z) = (
            <T as FromDig>::from_dig,
            literal(sep),
            <T as FromDig>::from_dig,
            literal(sep),
            <T as FromDig>::from_dig,
        )
            .parse_next(input)?;

        Ok(Point3::new(x, y, z))
    }
}

pub fn dig_pair<'a, T>(sep: &str) -> impl FnMut(&mut &'a str) -> Result<(T, T)> + '_
where
    T: Display + FromDig<Num = T>,
{
    move |input: &mut &'a str| {
        let l = <T as FromDig>::from_dig(input)?;
        literal(sep).parse_next(input)?;
        let r = <T as FromDig>::from_dig(input)?;
        Ok((l, r))
    }
}

pub fn signed_dig_pair<'a, T>(sep: &str) -> impl FnMut(&mut &'a str) -> Result<(T, T)> + '_
where
    T: Display + Signed + FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    move |input: &mut &'a str| {
        let l = signed_dig(input)?;
        literal(sep).parse_next(input)?;
        let r = signed_dig(input)?;
        Ok((l, r))
    }
}

pub fn separated_digits<'a, T>(sep: &str) -> impl FnMut(&mut &'a str) -> Result<Vec<T>> + '_
where
    T: Display + FromDig<Num = T>,
{
    let sep_str = sep.to_string();
    move |input: &mut &'a str| {
        separated(
            1..,
            |i: &mut &'a str| <T as FromDig>::from_dig(i),
            repeat::<_, _, String, _, _>(1.., alt((literal(sep_str.as_str()), literal(" ")))),
        )
        .parse_next(input)
    }
}

pub fn signed_dig<T>(input: &mut &str) -> Result<T>
where
    T: Signed + FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    (opt('-'), digit1)
        .take()
        .try_map(|s: &str| s.parse::<T>())
        .parse_next(input)
}

pub fn dig<T>(input: &mut &str) -> Result<T>
where
    T: Unsigned + FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    digit1
        .take()
        .try_map(|s: &str| s.parse::<T>())
        .parse_next(input)
}
