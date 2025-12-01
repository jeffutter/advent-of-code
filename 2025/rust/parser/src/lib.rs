use std::{fmt::Display, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    multi::many1,
    multi::separated_list1,
    IResult, Parser,
};
use num_traits::Signed;
use util::Pos;

pub trait FromDig {
    type Num;

    fn from_dig(s: &str) -> IResult<&str, Self::Num>;
}

impl FromDig for u32 {
    type Num = u32;
    fn from_dig(s: &str) -> IResult<&str, Self::Num> {
        map_res(digit1, |s: &str| s.parse::<u32>()).parse(s)
    }
}

impl FromDig for i32 {
    type Num = i32;
    fn from_dig(s: &str) -> IResult<&str, Self::Num> {
        map_res(digit1, |s: &str| s.parse::<i32>()).parse(s)
    }
}

impl FromDig for i64 {
    type Num = i64;
    fn from_dig(s: &str) -> IResult<&str, Self::Num> {
        map_res(digit1, |s: &str| s.parse::<i64>()).parse(s)
    }
}

impl FromDig for i128 {
    type Num = i128;
    fn from_dig(s: &str) -> IResult<&str, Self::Num> {
        map_res(digit1, |s: &str| s.parse::<i128>()).parse(s)
    }
}

impl FromDig for usize {
    type Num = usize;
    fn from_dig(s: &str) -> IResult<&str, Self::Num> {
        map_res(digit1, |s: &str| s.parse::<usize>()).parse(s)
    }
}

pub fn point<'a, T>(sep: &str) -> impl Fn(&'a str) -> IResult<&'a str, Pos<T>> + '_
where
    T: Display + FromDig<Num = T>,
{
    move |s: &'a str| {
        let (rest, (l, _, r)) =
            (<T as FromDig>::from_dig, tag(sep), <T as FromDig>::from_dig).parse(s)?;
        Ok((rest, Pos::new(l, r)))
    }
}

pub fn signed_point<'a, T>(sep: &str) -> impl Fn(&'a str) -> IResult<&'a str, Pos<T>> + '_
where
    T: Display + Signed + FromStr,
{
    move |s: &'a str| {
        let (rest, (l, _, r)) = (signed_dig, tag(sep), signed_dig).parse(s)?;
        Ok((rest, Pos::new(l, r)))
    }
}

pub fn dig_pair<'a, T>(sep: &str) -> impl Fn(&'a str) -> IResult<&'a str, (T, T)> + '_
where
    T: Display + FromDig<Num = T>,
{
    move |s: &'a str| {
        let (rest, (l, _, r)) =
            (<T as FromDig>::from_dig, tag(sep), <T as FromDig>::from_dig).parse(s)?;
        Ok((rest, (l, r)))
    }
}

pub fn signed_dig_pair<'a, T>(sep: &str) -> impl Fn(&'a str) -> IResult<&'a str, (T, T)> + '_
where
    T: Display + Signed + FromStr,
{
    move |s: &'a str| {
        let (rest, (l, _, r)) = (signed_dig, tag(sep), signed_dig).parse(s)?;
        Ok((rest, (l, r)))
    }
}

pub fn separated_digits<'a, T>(sep: &str) -> impl Fn(&'a str) -> IResult<&'a str, Vec<T>> + '_
where
    T: Display + FromDig<Num = T>,
{
    move |s: &'a str| separated_list1(many1(alt((tag(sep), tag(" ")))), <T as FromDig>::from_dig).parse(s)
}

pub fn signed_dig<T>(s: &str) -> IResult<&str, T>
where
    T: Signed + FromStr,
{
    map_res(recognize((opt(char('-')), digit1)), |s: &str| {
        s.parse::<T>()
    }).parse(s)
}
