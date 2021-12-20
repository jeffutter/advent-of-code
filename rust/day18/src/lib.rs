use std::fmt::Debug;
use std::ops::DerefMut;
use std::{fmt, ops::Deref};

use nom::{
    branch::alt,
    bytes::complete::tag,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Eq, PartialEq, Clone)]
struct SNumber(i32, usize);

impl Debug for SNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl SNumber {
    pub fn new(val: i32, depth: usize) -> Self {
        Self(val, depth)
    }
}

#[derive(Eq, PartialEq)]
struct SNumbers(Vec<SNumber>);

impl Deref for SNumbers {
    type Target = Vec<SNumber>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SNumbers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SNumbers {
    pub fn from_vec(vec: Vec<SNumber>) -> Self {
        Self(vec)
    }

    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Debug for SNumbers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut prev_depth: i32 = -1;
        let mut i = 0;

        while i < self.len() {
            let snum = &self[i];
            let next = self.get(i + 1);
            let change: i32 = (snum.1 as i32) - prev_depth;
            prev_depth = snum.1 as i32;

            match change {
                n if n > 0 => {
                    for _ in 0..change {
                        write!(f, "[")?;
                    }
                }
                n if n < 0 => {
                    for _ in 0..(change.abs() + 1) {
                        write!(f, "]")?;
                    }
                    write!(f, ",")?;
                }
                0 => (),
                _ => unreachable!(),
            }

            if let Some(next) = next {
                if snum.1 == next.1 {
                    write!(f, "{},{}", snum.0, next.0)?;
                    i += 1;
                } else {
                    write!(f, "{},", snum.0)?;
                }
            } else {
                write!(f, "{},", snum.0)?;
                for _ in 0..prev_depth {
                    write!(f, "]")?;
                }
            }
            i += 1;
        }

        Ok(())
    }
}

fn snum(depth: usize) -> impl Fn(&str) -> IResult<&str, SNumbers> {
    move |s| {
        let (rest, num) = parser::from_dig(s)?;
        Ok((rest, SNumbers::from_vec(vec![SNumber::new(num, depth)])))
    }
}

fn pair(depth: usize) -> impl Fn(&str) -> IResult<&str, SNumbers> {
    move |s| {
        let (rest, (mut a, mut b)) = delimited(
            tag("["),
            separated_pair(
                alt((snum(depth), pair(depth + 1))),
                tag(","),
                alt((snum(depth), pair(depth + 1))),
            ),
            tag("]"),
        )(s)?;

        a.append(&mut b);

        Ok((rest, a))
    }
}

fn parse(data: String) -> SNumbers {
    let (_rest, snums) = pair(0)(&data).unwrap();
    snums
}

fn reduce(snums: &mut SNumbers) -> &SNumbers {
    let mut i = 0;
    let mut j = snums.len();
    let mut changed = false;

    while i < j {
        let prev = i
            .checked_sub(1)
            .and_then(|im| snums.get(im).and_then(|x| Some(x.clone())));
        let a = snums.get(i).and_then(|x| Some(x.clone()));
        let b = snums.get(i + 1).and_then(|x| Some(x.clone()));
        let next = snums.get(i + 2).and_then(|x| Some(x.clone()));

        if let Some(a) = a {
            if let Some(b) = b {
                if a.1 == b.1 && a.1 >= 4 {
                    // println!("before explode: {:?}", snums);
                    snums[i] = SNumber::new(0, a.1 - 1);

                    if let Some(_) = prev {
                        snums[i - 1].0 += a.0;
                    }

                    if let Some(_) = next {
                        snums[i + 2].0 += b.0;
                    }

                    snums.remove(i + 1);
                    j -= 1;
                    i = 0;
                    changed = true;
                    // println!("after explode: {:?}", snums);
                }
            }
        }

        i += 1;
    }

    i = 0;
    j = snums.len();

    while i < j {
        let x = snums[i].clone();
        if x.0 >= 10 {
            // println!("before split: {:?}", snums);
            let a = (x.0 as f64 / 2f64).floor() as i32;
            let b = (x.0 as f64 / 2f64).ceil() as i32;

            snums[i].0 = a;
            snums[i].1 += 1;
            snums.insert(i + 1, SNumber::new(b, x.1));
            i += 1;
            j += 1;
            changed = true;
            // println!("after split: {:?}", snums);
        }
        i += 1
    }

    if changed {
        reduce(snums)
    } else {
        snums
    }
}

fn add<'a>(a: &'a mut SNumbers, b: &'a mut SNumbers) -> &'a SNumbers {
    // println!("{:?} + {:?}", a, b);
    for i in 0..a.len() {
        a[i].1 += 1;
    }

    for i in 0..b.len() {
        b[i].1 += 1;
    }

    a.append(b);

    // println!("= {:?}", a);

    a
}

fn magnitude(snums: SNumbers) -> i32 {
    let mut msnums = snums.clone();

    let mut ohshit = 0;

    while msnums.len() > 1 {
        println!("msnums: {:?}", msnums);
        let mut i = 0;
        let mut j = msnums.len();
        while i < j {
            // println!("msnums: {:?}", msnums);
            let a = &msnums[i].clone();
            let b = msnums.get(i + 1);
            // println!("a: {:?}, b: {:?}", a, b);

            if let Some(b) = b {
                if a.1 == b.1 {
                    msnums[i].0 = (a.0 * 3) + (b.0 * 2);
                    if msnums[i].1 > 0 {
                        msnums[i].1 -= 1;
                    }
                    msnums.remove(i + 1);
                    j -= 1;
                }
                i += 1;
            } else {
                i += 1;
            }
        }
        ohshit += 1;
        if ohshit == 200 {
            panic!()
        }
    }

    msnums[0].0
}

fn sum(data: String) -> SNumbers {
    data.lines()
        .map(|line| parse(line.to_string()))
        .reduce(|mut acc, mut line| {
            println!("  {:?}", acc);
            println!("+ {:?}", line);

            add(&mut acc, &mut line);
            reduce(&mut acc);

            println!("= {:?}\n", acc);
            acc
        })
        .unwrap()
}

pub fn part1(data: String) -> i32 {
    let added = data
        .lines()
        .map(|line| parse(line.to_string()))
        .reduce(|mut acc, mut line| {
            add(&mut acc, &mut line);
            reduce(&mut acc);
            acc
        });

    println!("Added: {:?}", added);

    magnitude(added.unwrap())
    // let mut snums = parse(data);
    // println!("Parsed: {:?}", snums);
    // reduce(&mut snums);
    // println!("reduced: {:?}", snums);
    // magnitude(snums)
}

pub fn part2(_data: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]".to_string();

        assert_eq!(
            parse(data),
            SNumbers::from_vec(vec![
                SNumber(1, 3),
                SNumber(2, 3),
                SNumber(3, 3),
                SNumber(4, 3),
                SNumber(5, 3),
                SNumber(6, 3),
                SNumber(7, 3),
                SNumber(8, 3),
                SNumber(9, 0),
            ])
        )
    }

    #[test]
    fn test_reduce() {
        let a = "[[[[4,3],4],4],[7,[[8,4],9]]]".to_string();
        let b = "[1,1]".to_string();

        let mut snumsa = parse(a);
        let mut snumsb = parse(b);

        add(&mut snumsa, &mut snumsb);

        reduce(&mut snumsa);

        assert_eq!(
            snumsa,
            SNumbers::from_vec(vec![
                SNumber(0, 3),
                SNumber(7, 3),
                SNumber(4, 2),
                SNumber(7, 3),
                SNumber(8, 3),
                SNumber(6, 3),
                SNumber(0, 3),
                SNumber(8, 0),
                SNumber(1, 0),
            ])
        )
    }

    #[test]
    fn test_magnitude_1() {
        let data = "[[1,2],[[3,4],5]]".to_string();

        assert_eq!(magnitude(parse(data)), 143)
    }

    #[test]
    fn test_magnitude_2() {
        let data = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string();

        assert_eq!(magnitude(parse(data)), 1384)
    }

    #[test]
    fn test_magnitude_3() {
        let data = "[[[[1,1],[2,2]],[3,3]],[4,4]]".to_string();

        assert_eq!(magnitude(parse(data)), 445)
    }

    #[test]
    fn test_magnitude_4() {
        let data = "[[[[3,0],[5,3]],[4,4]],[5,5]]".to_string();

        assert_eq!(magnitude(parse(data)), 791)
    }

    #[test]
    fn test_magnitude_5() {
        let data = "[[[[5,0],[7,4]],[5,5]],[6,6]]".to_string();

        assert_eq!(magnitude(parse(data)), 1137)
    }

    #[test]
    fn test_magnitude_6() {
        let data = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string();

        assert_eq!(magnitude(parse(data)), 3488)
    }

    #[test]
    fn test_sum() {
        let data = "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
"
        .to_string();
        assert_eq!(sum(data), SNumbers::new())
    }
}
