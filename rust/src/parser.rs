use nom::{character::complete::digit1, combinator::map_res, IResult};

pub fn from_dig(s: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| i32::from_str_radix(s, 10))(s)
}
