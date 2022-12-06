pub fn part1<'a>(data: &'a str) -> usize {
    find_consecutive_unique(data, 4)
}

pub fn part2<'a>(data: &'a str) -> usize {
    find_consecutive_unique(data, 14)
}

pub fn find_consecutive_unique<'a>(data: &'a str, num: usize) -> usize {
    if let Some((idx, _chars)) = data.chars().collect::<Vec<char>>()[..]
        .windows(num)
        .enumerate()
        .find(|(_idx, chars)| {
            let mut deduped = chars.clone().to_vec();
            deduped.sort();
            deduped.dedup();

            let mut sorted = chars.clone().to_vec();
            sorted.sort();

            deduped == sorted
        })
    {
        return num + idx;
    }
    unreachable!()
}

pub fn parse<'a>(data: &'a str) -> &'a str {
    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7)
    }
}
