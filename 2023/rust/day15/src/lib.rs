use indexmap::IndexMap;

pub fn parse<'a>(data: &'a str) -> impl Iterator<Item = &'a str> {
    data.trim_end_matches('\n').split(',')
}

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, c| (((acc + (c as usize)) * 17) % 256))
}

pub fn part1<'a>(input: impl Iterator<Item = &'a str>) -> usize {
    input.map(|s| hash(s)).sum()
}

pub fn part2<'a>(input: impl Iterator<Item = &'a str>) -> usize {
    let boxes: [IndexMap<String, usize>; 256] = core::array::from_fn(|_| IndexMap::new());

    input
        .fold(boxes, |mut boxes, item| {
            if item.ends_with("-") {
                let label = item.trim_end_matches("-");
                let label_hash = hash(label);
                boxes[label_hash].shift_remove(label);
                return boxes;
            }

            let (label, focal_length) = item.split_once("=").unwrap();
            let label_hash = hash(label);
            let focal_length: usize = focal_length.parse().unwrap();
            boxes[label_hash].insert(label.to_string(), focal_length);

            boxes
        })
        .iter()
        .enumerate()
        .map(|(idx, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(b_idx, (_, focal_length))| (idx + 1) * (b_idx + 1) * focal_length)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 1320);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 145);
    }

    generate_test! { 2023, 15, 1, 514025}
    generate_test! { 2023, 15, 2, 244461}
}
