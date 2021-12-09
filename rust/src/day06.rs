use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};
use std::collections::HashMap;

fn from_dig(s: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| i32::from_str_radix(s, 10))(s)
}

fn frisky_fish(data: String, days: usize) -> usize {
    let (_rest, fish) = separated_list1(tag(","), from_dig)(&data).unwrap();

    let mut school = HashMap::new();

    for f in fish {
        school.entry(f).and_modify(|x| *x += 1).or_insert(1);
    }

    for _i in 0..days {
        let mut prev = 0;

        for k in [8, 7, 6, 5, 4, 3, 2, 1, 0] {
            let v = school.get(&k).unwrap_or(&0).clone();

            match k {
                0 => {
                    // Old 0s
                    school.entry(6).and_modify(|x| *x += v).or_insert(v);
                    // Children
                    school.entry(8).and_modify(|x| *x += v).or_insert(v);
                    // Set 0s to prev 1s
                    school.insert(0, prev);
                }
                k => {
                    // Set Ks to prev
                    school.insert(k, prev);
                    prev = v;
                }
            }
        }
    }
    school.values().sum::<usize>()
}

pub fn part1(data: String) -> usize {
    frisky_fish(data, 80)
}

pub fn part2(data: String) -> usize {
    frisky_fish(data, 256)
}
