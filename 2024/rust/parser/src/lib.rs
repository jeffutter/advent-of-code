use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    multi::many1,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub trait FromDig {
    type Num;

    fn from_dig(s: &str) -> IResult<&str, Self::Num>;
}

impl FromDig for u32 {
    type Num = u32;
    fn from_dig(s: &str) -> IResult<&str, Self::Num> {
        map_res(digit1, |s: &str| s.parse::<u32>())(s)
    }
}

impl FromDig for i32 {
    type Num = i32;
    fn from_dig(s: &str) -> IResult<&str, Self::Num> {
        map_res(digit1, |s: &str| s.parse::<i32>())(s)
    }
}

impl FromDig for i64 {
    type Num = i64;
    fn from_dig(s: &str) -> IResult<&str, Self::Num> {
        map_res(digit1, |s: &str| s.parse::<i64>())(s)
    }
}

impl FromDig for usize {
    type Num = usize;
    fn from_dig(s: &str) -> IResult<&str, Self::Num> {
        map_res(digit1, |s: &str| s.parse::<usize>())(s)
    }
}

pub fn separated_digits(s: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(many1(alt((tag(","), tag(" ")))), <i32 as FromDig>::from_dig)(s)
}

pub fn signed_dig(s: &str) -> IResult<&str, i32> {
    map_res(recognize(tuple((opt(char('-')), digit1))), |s: &str| {
        s.parse::<i32>()
    })(s)
}
