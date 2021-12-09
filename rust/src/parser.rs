use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    multi::many1, multi::separated_list1, IResult,
};

pub fn from_dig(s: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| i32::from_str_radix(s, 10))(s)
}

pub fn separated_digits(s: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(many1(alt((tag(","), tag(" ")))), from_dig)(s)
}
