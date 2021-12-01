pub fn part1(data: String) {
    let result = data
        .lines()
        .map(|row| row.parse::<i32>().unwrap())
        .fold((0, None), |(acc, last), n| {
            if n > last.unwrap_or(i32::MAX) {
                (acc + 1, Some(n))
            } else {
                (acc, Some(n))
            }
        })
        .0;

    println!("Day1-1: {}", result)
}

pub fn part2(data: String) {
    let ns = data.lines().map(|row| row.parse::<i32>().unwrap());
    let i1 = ns.clone();
    let i2 = ns.clone().skip(1);
    let i3 = ns.clone().skip(2);

    let result = i1
        .zip(i2)
        .zip(i3)
        .fold((0, None), |(acc, last), ((n1, n2), n3)| {
            let n = n1 + n2 + n3;

            if n > last.unwrap_or(i32::MAX) {
                (acc + 1, Some(n))
            } else {
                (acc, Some(n))
            }
        })
        .0;

    println!("Day1-2: {}", result)
}
