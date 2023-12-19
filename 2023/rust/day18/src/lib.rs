use itertools::Itertools;
use util::{Direction, Pos};

pub struct Trench {
    direction: Direction,
    num: usize,
    color: String,
}

pub struct DigPlan {
    trenches: Vec<Trench>,
}

impl DigPlan {
    fn perimeter(&self) -> i64 {
        self.trenches.iter().map(|trench| trench.num).sum::<usize>() as i64
    }

    fn area(&self) -> i64 {
        let (_, segments) = self.trenches.iter().fold(
            (Pos::new(0, 0), Vec::new()),
            |(start_pos, mut acc): (Pos<i64>, Vec<(Pos<i64>, Pos<i64>)>), trench| {
                let end_pos = start_pos
                    .translate_n(&trench.direction, trench.num)
                    .unwrap();

                acc.push((start_pos, end_pos.clone()));

                (end_pos, acc)
            },
        );

        let sum: i64 = segments
            .iter()
            .map(|(a, b)| (a.x * b.y) - (a.y * b.x))
            .sum();

        sum / 2
    }
}

pub fn parse<'a>(data: &'a str) -> DigPlan {
    let mut trenches = Vec::new();

    for line in data.lines() {
        let mut pieces = line.split_whitespace();
        let direction = match pieces.next().unwrap() {
            "D" => Direction::S,
            "L" => Direction::W,
            "R" => Direction::E,
            "U" => Direction::N,
            d => unimplemented!("{:?}", d),
        };

        let num: usize = pieces.next().unwrap().parse().unwrap();

        let color = pieces
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(")")
            .to_string();

        trenches.push(Trench {
            direction,
            num,
            color,
        })
    }

    DigPlan { trenches }
}

pub fn part1<'a>(plan: DigPlan) -> i64 {
    let area = plan.area();
    let perimeter = plan.perimeter();
    let interior: i64 = area - (perimeter / 2) + 1;

    interior + perimeter
}

pub fn part2<'a>(plan: DigPlan) -> i64 {
    let fixed_trenches = plan
        .trenches
        .iter()
        .map(|trench| {
            let (num, dir) = trench.color.split_at(5);
            let num = usize::from_str_radix(num, 16).unwrap();
            let direction = match dir {
                "0" => Direction::E,
                "1" => Direction::S,
                "2" => Direction::W,
                "3" => Direction::N,
                x => unimplemented!("{}", x),
            };

            Trench {
                direction,
                num,
                color: "".to_string(),
            }
        })
        .collect_vec();

    let fixed_plan = DigPlan {
        trenches: fixed_trenches,
    };

    let area = fixed_plan.area();
    let perimeter = fixed_plan.perimeter();
    let interior: i64 = area - (perimeter / 2) + 1;

    interior + perimeter
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 62);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 952408144115);
    }

    generate_test! { 2023, 18, 1, 26857}
    generate_test! { 2023, 18, 2, 129373230496292}
}
