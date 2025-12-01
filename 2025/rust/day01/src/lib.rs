type InputType = Vec<Turn>;
type OutType = usize;

#[derive(Debug)]
pub enum Turn {
    Left(i32),
    Right(i32),
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    data.to_lowercase()
        .lines()
        .map(|line| {
            let (l, r) = line.trim().split_at(1);
            match l {
                "l" => Turn::Left(r.parse().unwrap()),
                "r" => Turn::Right(r.parse().unwrap()),
                x => unimplemented!("Unimplemented: {x}"),
            }
        })
        .collect()
}

pub fn part1(input: InputType) -> OutType {
    let mut cur: i32 = 50;
    let mut z_count = 0;

    for t in input {
        match t {
            Turn::Left(c) => {
                cur -= c;
                while cur < 0 {
                    cur += 100;
                }
            }
            Turn::Right(c) => {
                cur += c;
                while cur > 99 {
                    cur -= 100;
                }
            }
        }

        if cur == 0 {
            z_count += 1;
        }
    }

    z_count
}

#[allow(unused_variables)]
pub fn part2(input: InputType) -> OutType {
    let mut cur: i32 = 50;
    let mut z_count = 0;

    for t in input {
        match (cur, t) {
            (0, Turn::Left(c)) => {
                cur -= c;
                cur += 100;
                while cur < 0 {
                    z_count += 1;
                    cur += 100;
                }
                if cur == 0 {
                    z_count += 1;
                }
            }
            (_, Turn::Left(c)) => {
                cur -= c;
                while cur < 0 {
                    z_count += 1;
                    cur += 100;
                }
                if cur == 0 {
                    z_count += 1;
                }
            }
            (_, Turn::Right(c)) => {
                cur += c;
                while cur > 99 {
                    z_count += 1;
                    cur -= 100;
                }
            }
        }
    }

    z_count
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    generate_test!(
        r#"L68
           L30
           R48
           L5
           R60
           L55
           L1
           L99
           R14
           L82"#,
        1,
        3
    );

    generate_test!(
        r#"L68
           L30
           R48
           L5
           R60
           L55
           L1
           L99
           R14
           L82"#,
        2,
        6
    );

    generate_test! { 2025, 1, 1, 1071}
    generate_test! { 2025, 1, 2, 6700}
}
