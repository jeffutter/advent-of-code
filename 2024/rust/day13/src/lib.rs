use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    sequence::tuple,
};
use parser::FromDig;
use util::Pos;

type InputType = Vec<Machine>;
type OutType = i64;

#[derive(Debug)]
pub struct Machine {
    buttona: (i64, i64),
    buttonb: (i64, i64),
    prize: Pos<i64>,
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let (rest, machines) = separated_list1(
        many1(newline),
        map(
            tuple((
                tag("Button A: X+"),
                <i64 as FromDig>::from_dig,
                tag(", Y+"),
                <i64 as FromDig>::from_dig,
                newline,
                tag("Button B: X+"),
                <i64 as FromDig>::from_dig,
                tag(", Y+"),
                <i64 as FromDig>::from_dig,
                newline,
                tag("Prize: X="),
                <i64 as FromDig>::from_dig,
                tag(", Y="),
                <i64 as FromDig>::from_dig,
            )),
            |(_, ax, _, ay, _, _, bx, _, by, _, _, px, _, py)| Machine {
                buttona: (ax, ay),
                buttonb: (bx, by),
                prize: Pos::new(px, py),
            },
        ),
    )(data)
    .unwrap();
    assert_eq!("", rest.trim());
    machines
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    input
        .iter()
        .map(|machine| {
            let x1 = machine.buttona.0;
            let x2 = machine.buttona.1;
            let y1 = machine.buttonb.0;
            let y2 = machine.buttonb.1;
            let z1 = machine.prize.x;
            let z2 = machine.prize.y;

            let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
            let a = (z1 - b * y1) / x1;
            if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
                return 0;
            }
            a * 3 + b
        })
        .sum()
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    input
        .iter()
        .map(|machine| {
            let x1 = machine.buttona.0;
            let x2 = machine.buttona.1;
            let y1 = machine.buttonb.0;
            let y2 = machine.buttonb.1;
            let z1 = machine.prize.x + 10000000000000;
            let z2 = machine.prize.y + 10000000000000;

            let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
            let a = (z1 - b * y1) / x1;
            if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
                return 0;
            }
            a * 3 + b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    #[test]
    fn example_0() {
        let data = parse(
            r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"#,
        );
        assert_eq!(part1(data), 280)
    }

    generate_test!(
        r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#,
        1,
        480
    );

    generate_test! { 2024, 13, 1, 33209}
    generate_test! { 2024, 13, 2, 83102355665474}
}
