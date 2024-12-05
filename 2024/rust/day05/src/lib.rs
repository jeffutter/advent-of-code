use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many0, many1, separated_list1},
    sequence::tuple,
    IResult,
};
use parser::FromDig;

type InputType = (PageOrderRules, Vec<PageUpdate>);
type OutType = usize;
type PageOrder = (usize, usize);
type PageUpdate = Vec<usize>;
type PageOrderRule = HashMap<usize, HashSet<usize>>;

pub struct PageOrderRules {
    after: PageOrderRule,
    before: PageOrderRule,
}

impl PageOrderRules {
    fn new() -> Self {
        Self {
            after: HashMap::new(),
            before: HashMap::new(),
        }
    }

    fn insert(&mut self, a: usize, b: usize) {
        self.before
            .entry(b)
            .and_modify(|v| {
                v.insert(a);
            })
            .or_insert([a].into());

        self.after
            .entry(a)
            .and_modify(|v| {
                v.insert(b);
            })
            .or_insert([b].into());
    }

    fn cmp(&self, a: &usize, b: &usize) -> Ordering {
        if self.before.get(a).unwrap_or(&HashSet::new()).contains(b) {
            return Ordering::Less;
        }

        if self.after.get(a).unwrap_or(&HashSet::new()).contains(b) {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

fn parse_page_order(s: &str) -> IResult<&str, PageOrder> {
    map(
        tuple((
            <usize as FromDig>::from_dig,
            tag("|"),
            <usize as FromDig>::from_dig,
        )),
        |(a, _, b)| (a, b),
    )(s)
}

fn parse_page_order_rules(s: &str) -> IResult<&str, PageOrderRules> {
    let (rest, page_orders) = separated_list1(newline, parse_page_order)(s)?;

    let page_order_rules =
        page_orders
            .into_iter()
            .fold(PageOrderRules::new(), |mut page_order_rules, (a, b)| {
                page_order_rules.insert(a, b);
                page_order_rules
            });

    Ok((rest, page_order_rules))
}

fn parse_page_update(s: &str) -> IResult<&str, PageUpdate> {
    separated_list1(tag(","), <usize as FromDig>::from_dig)(s)
}

fn parse_page_updates(s: &str) -> IResult<&str, Vec<PageUpdate>> {
    separated_list1(newline, parse_page_update)(s)
}

fn parse_input(s: &str) -> IResult<&str, InputType> {
    let (rest, page_order_rules) = parse_page_order_rules(s)?;
    let (rest, _) = many1(newline)(rest)?;
    let (rest, page_updates) = parse_page_updates(rest)?;
    let (rest, _) = many0(newline)(rest)?;
    assert_eq!(rest, "");
    Ok((rest, (page_order_rules, page_updates)))
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let (_rest, x) = parse_input(data).unwrap();
    x
}

fn is_correct_order(page_order_rules: &PageOrderRules, pages: &[usize]) -> bool {
    pages.windows(3).all(|x| {
        page_order_rules.cmp(&x[0], &x[1]) == Ordering::Greater
            && page_order_rules.cmp(&x[2], &x[1]) == Ordering::Less
    })
}

#[allow(unused_variables)]
pub fn part1((page_order_rules, page_updates): InputType) -> OutType {
    page_updates
        .iter()
        .filter(|page_update| is_correct_order(&page_order_rules, page_update))
        .map(|page_update| page_update.get(page_update.len() / 2).unwrap())
        .sum()
}

fn correct_order(page_order_rules: &PageOrderRules, page_update: &mut [usize]) {
    page_update.sort_by(|a, b| page_order_rules.cmp(a, b))
}

#[allow(unused_variables)]
pub fn part2((page_order_rules, page_updates): InputType) -> OutType {
    let por_ref = &page_order_rules;

    page_updates
        .into_iter()
        .filter(|page_update| !is_correct_order(por_ref, page_update))
        .update(|page_update| correct_order(por_ref, page_update))
        .map(|page_update| *page_update.get(page_update.len() / 2).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#,
        1,
        143
    );

    generate_test!(
        r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#,
        2,
        123
    );

    generate_test! { 2024, 5, 1, 5452}
    generate_test! { 2024, 5, 2, 4598}
}
