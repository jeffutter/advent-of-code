use crate::parser;
use nom::{bytes::complete::tag, multi::separated_list1};
use std::collections::HashMap;

fn frisky_fish(data: String, days: usize) -> u64 {
    let (_rest, fish) = separated_list1(tag(","), parser::from_dig)(&data).unwrap();

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
    school.values().sum::<u64>()
}

pub fn part1(data: String) -> u64 {
    frisky_fish(data, 80)
}

pub fn part2(data: String) -> u64 {
    frisky_fish(data, 256)
}
