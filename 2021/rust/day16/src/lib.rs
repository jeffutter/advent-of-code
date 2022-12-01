use itertools::Itertools;
use nom::{
    bits::complete::{tag, take},
    branch::alt,
    combinator::map,
    multi::{many0, many_m_n},
    sequence::{preceded, tuple},
    IResult,
};
use Op::*;
use Type::*;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Packet(i64, Type<i64>);

#[derive(Debug, Eq, PartialEq, Clone)]
enum Op {
    SUM,
    PRO,
    MIN,
    MAX,
    GT,
    LT,
    EQ,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Type<T> {
    Literal(T),
    Operator(Op, Vec<Packet>),
}

fn literal_data(s: (&[u8], usize)) -> IResult<(&[u8], usize), i64> {
    let (rest, (mut bytes, last4)): (_, (Vec<u8>, u8)) = tuple((
        many0(preceded(tag(1, 1usize), take(4usize))),
        preceded(tag(0, 1usize), take(4usize)),
    ))(s)?;

    bytes.push(last4);

    let b = bytes.iter().fold(0i64, |acc, bs| (acc << 4) | *bs as i64);

    Ok((rest, b))
}

fn fifteen_bit_op(s: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
    let (mut rest, tot_len): ((&[u8], usize), usize) = preceded(tag(0, 1usize), take(15usize))(s)?;
    let end_len = (rest.0.len() * 8 - rest.1) - tot_len;
    let mut packets = vec![];
    while rest.0.len() * 8 - rest.1 > end_len {
        let (b, pkt) = packet(rest)?;
        rest = b;
        packets.push(pkt);
    }

    Ok((rest, packets))
}

fn eleven_bit_op(s: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
    let (rest, size): (_, usize) = preceded(tag(1, 1usize), take(11usize))(s)?;
    many_m_n(size, size, packet)(rest)
}

fn operator_data(s: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
    alt((fifteen_bit_op, eleven_bit_op))(s)
}

fn operator(s: (&[u8], usize)) -> IResult<(&[u8], usize), Type<i64>> {
    map(
        tuple((
            alt((
                map(tag(0b000, 3usize), |_| SUM),
                map(tag(0b001, 3usize), |_| PRO),
                map(tag(0b010, 3usize), |_| MIN),
                map(tag(0b011, 3usize), |_| MAX),
                map(tag(0b101, 3usize), |_| GT),
                map(tag(0b110, 3usize), |_| LT),
                map(tag(0b111, 3usize), |_| EQ),
            )),
            operator_data,
        )),
        |(op, packets)| Operator(op, packets),
    )(s)
}

fn literal(s: (&[u8], usize)) -> IResult<(&[u8], usize), Type<i64>> {
    map(preceded(tag(0b100, 3usize), literal_data), |lit| {
        Literal(lit)
    })(s)
}

fn packet(s: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    map(tuple((take(3usize), alt((literal, operator)))), |(v, t)| {
        Packet(v, t)
    })(s)
}

fn parse(data: &[u8]) -> Packet {
    packet((data, 0)).unwrap().1
}

fn decode(data: String) -> Vec<u8> {
    data.chars()
        .tuples()
        .map(|(a, b)| u8::from_str_radix(&[a, b].iter().join(""), 16).unwrap())
        .collect()
}

pub fn part1(data: String) -> i64 {
    let decoded = decode(data);
    let parsed = parse(&decoded);
    sum_versions(&parsed)
}

fn sum_versions(Packet(v, t): &Packet) -> i64 {
    match t {
        Literal(_) => *v,
        Operator(_, packets) => packets.iter().fold(*v, |acc, p| acc + sum_versions(p)),
    }
}

fn apply(Packet(_v, t): &Packet) -> i64 {
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
    let parsed = parse(&decoded);
    apply(&parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let data = "D2FE28".to_string();

        assert_eq!(decode(data), vec![210, 254, 40]);
    }

    #[test]
    fn test_parse_1() {
        let data = decode("D2FE28".to_string());

        assert_eq!(Packet(6, Literal(2021)), parse(&data))
    }

    #[test]
    fn test_parse_2() {
        let data = decode("38006F45291200".to_string());

        assert_eq!(
            Packet(
                1,
                Operator(LT, vec![Packet(6, Literal(10)), Packet(2, Literal(20))])
            ),
            parse(&data)
        )
    }

    #[test]
    fn test_parse_3() {
        let data = decode("EE00D40C823060".to_string());

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
            parse(&data)
        )
    }
}
