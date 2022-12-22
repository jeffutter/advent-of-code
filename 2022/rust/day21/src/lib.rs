use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn part1(t: Tree) -> i64 {
    math(&t, "root")
}

pub fn math(t: &Tree, m: &str) -> i64 {
    let monkey = t.get(m).unwrap();
    match &monkey.op {
        Op::Add(Val::Name(x), Val::Name(y)) => math(t, &x) + math(t, &y),
        Op::Subtract(Val::Name(x), Val::Name(y)) => math(t, &x) - math(t, &y),
        Op::Multiply(Val::Name(x), Val::Name(y)) => math(t, &x) * math(t, &y),
        Op::Divide(Val::Name(x), Val::Name(y)) => math(t, &x) / math(t, &y),
        Op::Value(Val::Name(x)) => math(t, &x),
        Op::Value(Val::Num(x)) => *x,
        _ => unreachable!(),
    }
}

fn not_humn<'a>(m: &'a Monkey) -> bool {
    m.name != "humn"
}

fn leaf<'a>(m: &'a Monkey) -> bool {
    if let Op::Value(_) = m.op {
        return true;
    }
    return false;
}

fn not_humn_leaf<'a>(m: &'a &Monkey) -> bool {
    not_humn(m) && leaf(m)
}

pub fn part2(mut t: Tree) -> i64 {
    let mut cloned = t.clone();
    let mut iter = cloned.arena.values().into_iter().filter(not_humn_leaf);

    while let Some(m) = iter.next() {
        let mut parent: &mut Monkey = t.parent_mut(&m.name).unwrap();

        let val = if let Op::Value(Val::Num(x)) = m.op {
            x
        } else {
            unreachable!()
        };

        match &parent.op {
            Op::Add(Val::Name(x), Val::Name(y)) if *x == m.name => {
                parent.op = Op::Add(Val::Num(val), Val::Name(y.clone()));
                t.remove(&m.name);
            }
            Op::Add(Val::Name(x), Val::Name(y)) if y.clone() == m.name => {
                parent.op = Op::Add(Val::Name(x.clone()), Val::Num(val));
                t.remove(&m.name);
            }
            Op::Add(Val::Name(x), Val::Num(y)) if x.clone() == m.name => {
                parent.op = Op::Value(Val::Num(val + y));
                t.remove(&m.name);
            }
            Op::Add(Val::Num(x), Val::Name(y)) if y.clone() == m.name => {
                parent.op = Op::Value(Val::Num(x + val));
                t.remove(&m.name);
            }
            Op::Subtract(Val::Name(x), Val::Name(y)) if x.clone() == m.name => {
                parent.op = Op::Subtract(Val::Num(val), Val::Name(y.clone()));
                t.remove(&m.name);
            }
            Op::Subtract(Val::Name(x), Val::Name(y)) if y.clone() == m.name => {
                parent.op = Op::Subtract(Val::Name(x.clone()), Val::Num(val));
                t.remove(&m.name);
            }
            Op::Subtract(Val::Name(x), Val::Num(y)) if x.clone() == m.name => {
                parent.op = Op::Value(Val::Num(val - y));
                t.remove(&m.name);
            }
            Op::Subtract(Val::Num(x), Val::Name(y)) if y.clone() == m.name => {
                parent.op = Op::Value(Val::Num(x - val));
                t.remove(&m.name);
            }
            Op::Multiply(Val::Name(x), Val::Name(y)) if x.clone() == m.name => {
                parent.op = Op::Multiply(Val::Num(val), Val::Name(y.clone()));
                t.remove(&m.name);
            }
            Op::Multiply(Val::Name(x), Val::Name(y)) if y.clone() == m.name => {
                parent.op = Op::Multiply(Val::Name(x.clone()), Val::Num(val));
                t.remove(&m.name);
            }
            Op::Multiply(Val::Name(x), Val::Num(y)) if x.clone() == m.name => {
                parent.op = Op::Value(Val::Num(val * y));
                t.remove(&m.name);
            }
            Op::Multiply(Val::Num(x), Val::Name(y)) if y.clone() == m.name => {
                parent.op = Op::Value(Val::Num(x * val));
                t.remove(&m.name);
            }
            Op::Divide(Val::Name(x), Val::Name(y)) if x.clone() == m.name => {
                parent.op = Op::Divide(Val::Num(val), Val::Name(y.clone()));
                t.remove(&m.name);
            }
            Op::Divide(Val::Name(x), Val::Name(y)) if y.clone() == m.name => {
                parent.op = Op::Divide(Val::Name(x.clone()), Val::Num(val));
                t.remove(&m.name);
            }
            Op::Divide(Val::Name(x), Val::Num(y)) if x.clone() == m.name => {
                parent.op = Op::Value(Val::Num(val / y));
                t.remove(&m.name);
            }
            Op::Divide(Val::Num(x), Val::Name(y)) if y.clone() == m.name => {
                parent.op = Op::Value(Val::Num(x / val));
                t.remove(&m.name);
            }
            _ => unreachable!(),
        }

        cloned = t.clone();
        iter = cloned.arena.values().into_iter().filter(not_humn_leaf);
    }

    let c = t.get("root").unwrap();
    let (target, new_root) = match &c.op {
        Op::Add(Val::Num(x), Val::Name(y)) => (x, y),
        Op::Add(Val::Name(x), Val::Num(y)) => (y, x),
        Op::Subtract(Val::Num(x), Val::Name(y)) => (x, y),
        Op::Subtract(Val::Name(x), Val::Num(y)) => (y, x),
        Op::Multiply(Val::Num(x), Val::Name(y)) => (x, y),
        Op::Multiply(Val::Name(x), Val::Num(y)) => (y, x),
        Op::Divide(Val::Num(x), Val::Name(y)) => (x, y),
        Op::Divide(Val::Name(x), Val::Num(y)) => (y, x),
        _ => unreachable!(),
    };

    let mut nr = new_root.to_owned();
    let mut acc = target.to_owned();

    'l: loop {
        match t.get(&nr).unwrap().op.clone() {
            // # X + y = Z
            // # y = Z - X
            Op::Add(Val::Num(x), Val::Name(y)) => {
                acc = acc - x;
                nr = y;
            }
            // # x + Y = Z
            // # x = Z - Y
            Op::Add(Val::Name(x), Val::Num(y)) => {
                acc = acc - y;
                nr = x;
            }
            // # X - y = Z
            // # y = X - Z
            Op::Subtract(Val::Num(x), Val::Name(y)) => {
                acc = x - acc;
                nr = y;
            }
            // # x - Y = Z
            // # x = Z + Y
            Op::Subtract(Val::Name(x), Val::Num(y)) => {
                acc = y + acc;
                nr = x;
            }
            // # X * y = Z
            // # y = Z / x
            Op::Multiply(Val::Num(x), Val::Name(y)) => {
                acc = acc / x;
                nr = y;
            }
            // # x * Y = Z
            // # x = Z / y
            Op::Multiply(Val::Name(x), Val::Num(y)) => {
                acc = acc / y;
                nr = x;
            }
            // # X / y = Z
            // # X = y * Z
            // # X / Z = y
            Op::Divide(Val::Num(x), Val::Name(y)) => {
                acc = x / acc;
                nr = y;
            }
            // # x / Y = Z
            // # x = Z * Y
            Op::Divide(Val::Name(x), Val::Num(y)) => {
                acc = y * acc;
                nr = x;
            }
            _ => unreachable!(),
        }

        if nr == "humn" {
            break;
        }
    }

    acc
}

pub fn parse<'a>(data: &'a str) -> Tree {
    let (rest, monkeys) = separated_list1(newline, monkey)(data).unwrap();

    assert_eq!("", rest.trim());

    let mut t = Tree::new();

    for monkey in monkeys {
        t.insert(monkey);
    }

    t
}

#[derive(Clone, Debug)]
pub struct Tree {
    arena: HashMap<String, Monkey>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            arena: HashMap::new(),
        }
    }

    fn insert(&mut self, m: Monkey) {
        self.arena.insert(m.name.clone(), m);
    }

    fn remove(&mut self, m: &str) {
        self.arena.remove(m);
    }

    fn get(&self, s: &str) -> Option<&Monkey> {
        self.arena.get(s)
    }

    fn children(&self, n: &str) -> Vec<&Monkey> {
        self.arena
            .get(n)
            .map(|parent| {
                parent
                    .op
                    .next()
                    .into_iter()
                    .map(|s| self.get(s))
                    .collect::<Option<Vec<_>>>()
                    .unwrap_or(vec![])
            })
            .unwrap_or(vec![])
    }

    fn parent_mut(&mut self, n: &str) -> Option<&mut Monkey> {
        let cloned = self.clone();
        self.arena
            .iter_mut()
            .find(|(name, _monkey)| cloned.children(name).iter().any(|m| m.name == n))
            .map(|(_name, monkey)| monkey)
    }

    fn leaves(&self) -> impl Iterator<Item = Monkey> + '_ {
        self.arena.iter().filter_map(|(_name, monkey)| {
            if let Op::Value(_) = monkey.op {
                Some(monkey.clone())
            } else {
                None
            }
        })
    }
}

impl Iterator for Tree {
    type Item = Monkey;

    fn next(&mut self) -> Option<Self::Item> {
        self.leaves().nth(0).clone()
    }
}

fn monkey(s: &str) -> IResult<&str, Monkey> {
    let (rest, (name, _, op)) = tuple((alpha1, tag(": "), op))(s)?;

    Ok((
        rest,
        Monkey {
            name: name.to_string(),
            op,
        },
    ))
}

fn op(s: &str) -> IResult<&str, Op> {
    alt((addition, subtraction, multiplication, division, value))(s)
}

fn addition(s: &str) -> IResult<&str, Op> {
    let (rest, (lhs, _, rhs)) = tuple((alpha1, tag(" + "), alpha1))(s)?;

    Ok((
        rest,
        Op::Add(Val::Name(lhs.to_string()), Val::Name(rhs.to_string())),
    ))
}

fn subtraction(s: &str) -> IResult<&str, Op> {
    let (rest, (lhs, _, rhs)) = tuple((alpha1, tag(" - "), alpha1))(s)?;

    Ok((
        rest,
        Op::Subtract(Val::Name(lhs.to_string()), Val::Name(rhs.to_string())),
    ))
}

fn multiplication(s: &str) -> IResult<&str, Op> {
    let (rest, (lhs, _, rhs)) = tuple((alpha1, tag(" * "), alpha1))(s)?;

    Ok((
        rest,
        Op::Multiply(Val::Name(lhs.to_string()), Val::Name(rhs.to_string())),
    ))
}

fn division(s: &str) -> IResult<&str, Op> {
    let (rest, (lhs, _, rhs)) = tuple((alpha1, tag(" / "), alpha1))(s)?;

    Ok((
        rest,
        Op::Divide(Val::Name(lhs.to_string()), Val::Name(rhs.to_string())),
    ))
}

fn value(s: &str) -> IResult<&str, Op> {
    map(map_res(digit1, |s: &str| i64::from_str_radix(s, 10)), |d| {
        Op::Value(Val::Num(d))
    })(s)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Monkey {
    name: String,
    op: Op,
}

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Add(Val, Val),
    Subtract(Val, Val),
    Multiply(Val, Val),
    Divide(Val, Val),
    Value(Val),
}

impl Op {
    fn next(&self) -> Vec<&str> {
        match self {
            Op::Add(Val::Name(lhs), Val::Name(rhs)) => vec![lhs, rhs],
            Op::Add(Val::Name(lhs), Val::Num(_)) => vec![lhs],
            Op::Add(Val::Num(_), Val::Name(rhs)) => vec![rhs],
            Op::Subtract(Val::Name(lhs), Val::Name(rhs)) => vec![lhs, rhs],
            Op::Subtract(Val::Name(lhs), Val::Num(_)) => vec![lhs],
            Op::Subtract(Val::Num(_), Val::Name(rhs)) => vec![rhs],
            Op::Multiply(Val::Name(lhs), Val::Name(rhs)) => vec![lhs, rhs],
            Op::Multiply(Val::Name(lhs), Val::Num(_)) => vec![lhs],
            Op::Multiply(Val::Num(_), Val::Name(rhs)) => vec![rhs],
            Op::Divide(Val::Name(lhs), Val::Name(rhs)) => vec![lhs, rhs],
            Op::Divide(Val::Name(lhs), Val::Num(_)) => vec![lhs],
            Op::Divide(Val::Num(_), Val::Name(rhs)) => vec![rhs],
            Op::Value(_) => vec![],
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Val {
    Num(i64),
    Name(String),
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

    #[test]
    fn test1() {
        let parsed = parse(INPUT);
        let res = part1(parsed);
        assert_eq!(152, res)
    }

    #[test]
    fn test2() {
        let parsed = parse(INPUT);
        let res = part2(parsed);
        assert_eq!(301, res)
    }
}
