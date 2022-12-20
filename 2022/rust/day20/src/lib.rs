pub fn part1(state: Vec<i64>) -> i64 {
    decode(state, 1, 1)
}

pub fn part2(state: Vec<i64>) -> i64 {
    decode(state, 811589153, 10)
}

fn decode(state: Vec<i64>, key: i64, times: usize) -> i64 {
    let mut q = state
        .iter()
        .map(|x| x * key)
        .enumerate()
        .collect::<Vec<(usize, i64)>>();

    for _ in 0..times {
        for (oidx, val) in state.iter().enumerate() {
            if *val == 0 {
                continue;
            }

            let pos = q
                .iter()
                .position(|x| x.0 == oidx.try_into().unwrap())
                .unwrap();

            let val = q.remove(pos);

            let len: i64 = q.len().try_into().unwrap();
            let pos: i64 = pos.try_into().unwrap();
            let new_idx: usize = (val.1 + pos).rem_euclid(len).try_into().unwrap();

            q.insert(new_idx, val);
        }
    }

    let zero_idx = q.iter().position(|(_, x)| *x == 0).unwrap();
    let a = q[(zero_idx + 1000) % q.len()].1;
    let b = q[(zero_idx + 2000) % q.len()].1;
    let c = q[(zero_idx + 3000) % q.len()].1;

    a + b + c
}

pub fn parse<'a>(data: &'a str) -> Vec<i64> {
    data.lines().map(|f| f.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"1
2
-3
3
-2
0
4"#;

    #[test]
    fn test1() {
        let parsed = parse(INPUT);
        let res = part1(parsed);
        assert_eq!(3, res)
    }
}
