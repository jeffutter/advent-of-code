use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::Itertools;

type InputType<'a> = Box<dyn Iterator<Item = (&'a str, &'a str)> + 'a>;
type OutType = usize;

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType<'_> {
    Box::new(data.lines().map(|l| {
        let (left, right) = l.split_once("-").unwrap();
        (left, right)
    }))
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (left, right) in input {
        connections.entry(left).or_default().insert(right);
        connections.entry(right).or_default().insert(left);
    }

    connections
        .iter()
        .filter(|(k, v)| k.starts_with("t"))
        .flat_map(|(k, v)| {
            iter::once(k)
                .chain(v.iter())
                .combinations_with_replacement(3)
                .filter(|x| {
                    let a = x[0];
                    let b = x[1];
                    let c = x[2];

                    let a_hs = connections.get(a).unwrap();
                    let b_hs = connections.get(b).unwrap();
                    let c_hs = connections.get(c).unwrap();

                    a_hs.contains(b)
                        && a_hs.contains(c)
                        && b_hs.contains(a)
                        && b_hs.contains(c)
                        && c_hs.contains(a)
                        && c_hs.contains(b)
                })
        })
        .map(|v| v.into_iter().sorted().collect_vec())
        .unique()
        .count()

    // connections
    //     .keys()
    //     .combinations_with_replacement(3)
    //     .filter(|x| {
    //         let a = x[0];
    //         let b = x[1];
    //         let c = x[2];
    //
    //         let a_hs = connections.get(a).unwrap();
    //         let b_hs = connections.get(b).unwrap();
    //         let c_hs = connections.get(c).unwrap();
    //
    //         (a_hs.contains(b)
    //             && a_hs.contains(c)
    //             && b_hs.contains(a)
    //             && b_hs.contains(c)
    //             && c_hs.contains(a)
    //             && c_hs.contains(b))
    //             && (a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
    //     })
    //     .count()
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> String {
    String::from("")
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#,
        1,
        7
    );

    generate_test!(
        r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#,
        2,
        "co,de,ka,ta"
    );

    generate_test! { 2024, 23, 1, 998}
    generate_test! { 2024, 23, 2, ""}
}
