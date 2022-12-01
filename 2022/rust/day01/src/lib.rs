pub fn part1(data: String) -> i32 {
    group_by_elves(&data).max().unwrap()
}

pub fn part2(data: String) -> i32 {
    let mut calories: Vec<i32> = group_by_elves(&data).collect();

    calories.sort();

    calories.iter().rev().take(3).sum()
}

fn group_by_elves<'a>(data: &'a str) -> impl Iterator<Item = i32> + 'a {
    let mut t: i32 = 0;
    let mut v: Vec<i32> = vec![];
    for n in data.lines() {
        match n {
            "" => {
                v.push(t);
                t = 0;
            }
            _ => t += n.parse::<i32>().unwrap(),
        }
    }
    if t > 0 {
        v.push(t);
    }
    v.into_iter()
}
