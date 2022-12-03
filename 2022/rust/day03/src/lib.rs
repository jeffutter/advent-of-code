use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::RangeInclusive;

const LOWER: RangeInclusive<char> = 'a'..='z';
const UPPER: RangeInclusive<char> = 'A'..='Z';
const VALUE: RangeInclusive<i32> = 1..=52;

lazy_static! {
    static ref VALUES: HashMap<char, i32> = LOWER.chain(UPPER).zip(VALUE).collect();
}

#[derive(Debug)]
pub struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
}

impl Rucksack {
    pub fn from_vec_char(a: Vec<char>, b: Vec<char>) -> Self {
        let set_a = HashSet::from_iter(a.iter().cloned());
        let set_b = HashSet::from_iter(b.iter().cloned());

        Self {
            compartment1: set_a,
            compartment2: set_b,
        }
    }

    fn common(&self) -> HashSet<char> {
        hash_set_intersection(self.compartment1.clone(), self.compartment2.clone())
    }

    fn all(&self) -> HashSet<char> {
        self.compartment1
            .union(&self.compartment2)
            .cloned()
            .collect()
    }
}

pub fn parse(data: &str) -> Vec<Rucksack> {
    data.lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let len = chars.len();
            let (left, right) = chars.split_at(len / 2);
            Rucksack::from_vec_char(left.to_vec(), right.to_vec())
        })
        .collect()
}

pub fn value(c: char) -> i32 {
    *VALUES.get(&c).unwrap()
}

fn hash_set_intersection<T>(s1: HashSet<T>, s2: HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Copy,
{
    hash_sets_intersection(vec![s1, s2].into_iter())
}

fn hash_sets_intersection<T>(mut sets: impl Iterator<Item = HashSet<T>>) -> HashSet<T>
where
    T: Eq + Hash + Copy,
{
    let mut s = sets.next().unwrap();
    for set in sets {
        s = s.intersection(&set).copied().collect();
    }
    s
}

pub fn part1(rucksacks: Vec<Rucksack>) -> i32 {
    rucksacks
        .iter()
        .map(|sack| sack.common().iter().map(|c| value(*c)).sum::<i32>())
        .sum()
}

pub fn part2(rucksacks: Vec<Rucksack>) -> i32 {
    rucksacks
        .iter()
        .map(|sack| sack.all())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let i = chunk.iter().cloned();
            let set = hash_sets_intersection(i);
            let c = set.iter().next().unwrap();
            value(*c)
        })
        .sum()
}
