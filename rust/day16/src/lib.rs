use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    multi::{many0, many1, many_m_n},
    sequence::{preceded, tuple},
    IResult,
};
use std::fmt;
use std::fmt::Debug;
use Op::*;
use Type::*;

#[derive(Debug, Eq, PartialEq)]
struct Packet(i64, Type<i64>);

#[derive(Debug, Eq, PartialEq)]
enum Op {
    SUM,
    PRO,
    MIN,
    MAX,
    GT,
    LT,
    EQ,
}

#[derive(Eq, PartialEq)]
enum Type<T> {
    Literal(T),
    Operator(Op, Vec<Packet>),
}

impl Debug for Type<i64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal(v) => write!(f, "{}", v)?,
            Operator(t, v) => write!(f, "({:?}, {:?})", t, v)?,
        };

        Ok(())
    }
}

fn literal_data(s: &str) -> IResult<&str, i64> {
    let (rest, (mut bytes, last4)) = tuple((
        many0(preceded(tag("1"), take(4usize))),
        preceded(tag("0"), take(4usize)),
    ))(s)?;

    bytes.push(last4);

    let d = i64::from_str_radix(&bytes.concat(), 2).unwrap();

    Ok((rest, d))
}

fn fifteen_bit_op(s: &str) -> IResult<&str, Vec<Packet>> {
    let (rest, size_str) = preceded(tag("0"), take(15usize))(s)?;
    let size = usize::from_str_radix(size_str, 2).unwrap();
    let (rest, data) = take(size)(rest)?;
    let (_, types) = many1(packet)(data)?;

    Ok((rest, types))
}

fn eleven_bit_op(s: &str) -> IResult<&str, Vec<Packet>> {
    let (rest, size_str) = preceded(tag("1"), take(11usize))(s)?;
    let size = usize::from_str_radix(size_str, 2).unwrap();
    many_m_n(size, size, packet)(rest)
}

fn operator_data(s: &str) -> IResult<&str, Vec<Packet>> {
    alt((fifteen_bit_op, eleven_bit_op))(s)
}

fn operator(s: &str) -> IResult<&str, Type<i64>> {
    let (rest, (op, packets)) = tuple((
        alt((
            tag("000"),
            tag("001"),
            tag("010"),
            tag("011"),
            // tag("100"),
            tag("101"),
            tag("110"),
            tag("111"),
        )),
        operator_data,
    ))(s)?;

    let operation = match op {
        "000" => SUM,
        "001" => PRO,
        "010" => MIN,
        "011" => MAX,
        "101" => GT,
        "110" => LT,
        "111" => EQ,
        _ => unimplemented!(),
    };

    Ok((rest, Operator(operation, packets)))
}
fn literal(s: &str) -> IResult<&str, Type<i64>> {
    let (rest, lit) = preceded(tag("100"), literal_data)(s)?;

    Ok((rest, Literal(lit)))
}

fn packet(s: &str) -> IResult<&str, Packet> {
    let (rest, (v_str, t)) = tuple((take(3usize), alt((literal, operator))))(s)?;

    let v = i64::from_str_radix(v_str, 2).unwrap();

    Ok((rest, Packet(v, t)))
}

fn parse(data: String) -> Packet {
    let (_rest, packet) = packet(&data).unwrap();
    packet
}

fn decode(data: String) -> String {
    data.chars()
        .enumerate()
        .scan(String::new(), |acc, (i, c)| {
            if (i + 1) % 2 == 0 {
                acc.push(c);
                let d = u8::from_str_radix(acc, 16).unwrap();
                *acc = String::new();
                Some(format!("{:08b}", d))
            } else {
                acc.push(c);
                Some("".to_string())
            }
        })
        .collect()
}

pub fn part1(data: String) -> i64 {
    let decoded = decode(data);
    let parsed = parse(decoded);
    sum_versions(&parsed)
}

fn sum_versions(packet: &Packet) -> i64 {
    let Packet(v, t) = packet;

    match t {
        Literal(_) => *v,
        Operator(_, packets) => packets.iter().fold(*v, |acc, p| acc + sum_versions(p)),
    }
}

fn apply(packet: &Packet) -> i64 {
    let Packet(_v, t) = packet;

    match t {
        Literal(v) => *v,
        Operator(SUM, vs) => vs.iter().map(|p| apply(p)).sum(),
        Operator(PRO, vs) => vs.iter().map(|p| apply(p)).product(),
        Operator(MIN, vs) => vs.iter().map(|p| apply(p)).min().unwrap(),
        Operator(MAX, vs) => vs.iter().map(|p| apply(p)).max().unwrap(),
        Operator(GT, vs) => {
            let mut i = vs.iter();
            let a = i.next().unwrap();
            let b = i.next().unwrap();

            match apply(a) > apply(b) {
                true => 1,
                false => 0,
            }
        }
        Operator(LT, vs) => {
            let mut i = vs.iter();
            let a = i.next().unwrap();
            let b = i.next().unwrap();

            match apply(a) < apply(b) {
                true => 1,
                false => 0,
            }
        }
        Operator(EQ, vs) => {
            let mut i = vs.iter();
            let a = i.next().unwrap();
            let b = i.next().unwrap();

            match apply(a) == apply(b) {
                true => 1,
                false => 0,
            }
        }
    }
}

pub fn part2(data: String) -> i64 {
    let decoded = decode(data);
    let parsed = parse(decoded);
    apply(&parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let data = "D2FE28".to_string();
        let res = "110100101111111000101000".to_string();

        assert_eq!(decode(data), res);
    }

    #[test]
    fn test_parse_1() {
        let data = "110100101111111000101000".to_string();

        assert_eq!(Packet(6, Literal(2021)), parse(data))
    }

    #[test]
    fn test_parse_2() {
        let data = "00111000000000000110111101000101001010010001001000000000".to_string();

        assert_eq!(
            Packet(
                1,
                Operator(LT, vec![Packet(6, Literal(10)), Packet(2, Literal(20))])
            ),
            parse(data)
        )
    }

    #[test]
    fn test_parse_3() {
        let data = "11101110000000001101010000001100100000100011000001100000".to_string();

        assert_eq!(
            Packet(
                7,
                Operator(
                    MAX,
                    vec![
                        Packet(2, Literal(1)),
                        Packet(4, Literal(2)),
                        Packet(1, Literal(3))
                    ]
                )
            ),
            parse(data)
        )
    }
}
