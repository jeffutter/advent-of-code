use std::collections::BTreeMap;

#[derive(Debug)]
enum ByteCount {
    MORE0,
    MORE1,
    EQUAL,
}

fn counts<T: AsRef<str>>(data: impl Iterator<Item = T>) -> BTreeMap<usize, ByteCount> {
    data.fold(BTreeMap::new(), |mut acc, row| {
        row.as_ref()
            .trim_start()
            .trim_end()
            .char_indices()
            .for_each(|(i, c)| {
                let val = match c {
                    '1' => 1,
                    '0' => -1,
                    _ => unimplemented!("Unknown binary character: {}", c),
                };
                acc.entry(i).and_modify(|v| *v += val).or_insert(val);
            });
        acc
    })
    .iter()
    .map(|(k, v)| {
        let c = match *v {
            v if v > 0 => ByteCount::MORE1,
            v if v < 0 => ByteCount::MORE0,
            0 => ByteCount::EQUAL,
            _ => unreachable!("Unreachable"),
        };
        (*k, c)
    })
    .collect()
}

pub fn part1(data: String) -> i32 {
    let counts = counts(data.lines());

    let gamma: String = counts
        .iter()
        .map(|(_, v)| {
            let v = match *v {
                ByteCount::MORE0 => 0,
                ByteCount::MORE1 => 1,
                _ => panic!("Unknown digit"),
            };
            char::from_digit(v, 10).unwrap()
        })
        .collect();

    let epsilon: String = counts
        .iter()
        .map(|(_, v)| {
            let v = match *v {
                ByteCount::MORE0 => 1,
                ByteCount::MORE1 => 0,
                _ => panic!("Unknown digit"),
            };
            char::from_digit(v, 10).unwrap()
        })
        .collect();

    let gamman: i32 = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilonn: i32 = i32::from_str_radix(&epsilon, 2).unwrap();
    gamman * epsilonn
}

pub fn part2(data: String) -> i32 {
    let len = data.lines().next().unwrap().chars().count();

    let mut oxygen: Vec<&str> = data.lines().map(|l| l.trim_start().trim_end()).collect();
    let mut co2: Vec<&str> = oxygen.clone();

    for i in 0..(len) {
        if oxygen.len() > 1 {
            let oxygen_counts = counts(oxygen.iter());

            let mut oxygen_keep: Vec<&str> = Vec::new();

            oxygen.iter().for_each(|row| {
                match (oxygen_counts.get(&i).unwrap(), row.chars().nth(i).unwrap()) {
                    (ByteCount::MORE1, '1') => {
                        oxygen_keep.push(row);
                    }
                    (ByteCount::MORE0, '0') => {
                        oxygen_keep.push(row);
                    }
                    (ByteCount::EQUAL, '1') => {
                        oxygen_keep.push(row);
                    }
                    _ => (),
                }
            });
            oxygen = oxygen_keep;
        }
        if co2.len() > 1 {
            let co2_counts = counts(co2.iter());

            let mut co2_keep: Vec<&str> = Vec::new();

            co2.iter().for_each(|row| {
                match (co2_counts.get(&i).unwrap(), row.chars().nth(i).unwrap()) {
                    (ByteCount::MORE1, '0') => {
                        co2_keep.push(row);
                    }
                    (ByteCount::MORE0, '1') => {
                        co2_keep.push(row);
                    }
                    (ByteCount::EQUAL, '0') => {
                        co2_keep.push(row);
                    }
                    _ => (),
                }
            });
            co2 = co2_keep;
        }
    }

    let oxygenn: i32 = i32::from_str_radix(&oxygen.get(0).unwrap(), 2).unwrap();
    let co2n: i32 = i32::from_str_radix(&co2.get(0).unwrap(), 2).unwrap();
    oxygenn * co2n
}
