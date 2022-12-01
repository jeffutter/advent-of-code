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

pub fn from_dig(s: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| i32::from_str_radix(s, 10))(s)
}

pub fn separated_digits(s: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(many1(alt((tag(","), tag(" ")))), from_dig)(s)
}

pub fn signed_dig(s: &str) -> IResult<&str, i32> {
    map_res(recognize(tuple((opt(char('-')), digit1))), |s: &str| {
        i32::from_str_radix(s, 10)
    })(s)
}
