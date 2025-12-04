use util::{BitMap, Pos};

type InputType = BitMap<usize>;
type OutType = usize;

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let height = data.lines().count();
    let width = data.lines().next().unwrap().len();

    BitMap::from_iter(
        data.lines().enumerate().flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(x, byte)| *byte == &b'@')
                .map(move |(x, _)| Pos::new(x, y))
        }),
        width,
        height,
    )
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    input.iter().filter(|pos| can_take(pos, &input)).count()
}

fn can_take(p: &Pos<usize>, bm: &BitMap<usize>) -> bool {
    p.successors_8_unsigned()
        .iter()
        .filter(|pos| bm.contains(pos))
        .count()
        < 4
}

#[allow(unused_variables)]
pub fn part2(mut input: InputType) -> OutType {
    let mut total_removed = 0;

    loop {
        let mut exit = true;

        for pos in input.clone().iter() {
            if can_take(&pos, &input) {
                input.remove(&pos);
                total_removed += 1;
                exit = false;
            }
        }

        if exit {
            break;
        }
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const TEST_INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    generate_test!(TEST_INPUT, 1, 13);

    generate_test!(TEST_INPUT, 2, 43);

    generate_test! { 2025, 4, 1, 1486}
    generate_test! { 2025, 4, 2, 9024}
}
