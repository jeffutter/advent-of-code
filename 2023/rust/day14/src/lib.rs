use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Rocks {
    Round,
    Cube,
    Empty,
}

#[derive(Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BitMap {
    cols: Vec<u128>,
    rows: Vec<u128>,
}

impl BitMap {
    fn new(width: usize, height: usize) -> Self {
        Self {
            cols: vec![0; width],
            rows: vec![0; height],
        }
    }

    fn present(&self, x: usize, y: usize) -> bool {
        let col = self.cols.get(x).unwrap();
        col & (1 << y) > 0
    }

    fn set(&mut self, x: usize, y: usize) {
        let col = self.cols.get_mut(x).unwrap();
        *col = *col | (1 << y);
        let row = self.rows.get_mut(y).unwrap();
        *row = *row | (1 << x);
    }

    fn unset(&mut self, x: usize, y: usize) {
        let col = self.cols.get_mut(x).unwrap();
        *col = *col & !(1 << y);
        let row = self.rows.get_mut(y).unwrap();
        *row = *row & (1 << x);
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Table {
    round_rocks: BitMap,
    cube_rocks: BitMap,
    max_x: usize,
    max_y: usize,
}

impl Table {
    fn new(width: usize, height: usize) -> Self {
        Self {
            round_rocks: BitMap::new(width, height),
            cube_rocks: BitMap::new(width, height),
            max_x: width - 1,
            max_y: height - 1,
        }
    }

    fn get(&self, x: usize, y: usize) -> Rocks {
        match (
            self.round_rocks.present(x, y),
            self.cube_rocks.present(x, y),
        ) {
            (true, true) => unreachable!(),
            (true, false) => Rocks::Round,
            (false, true) => Rocks::Cube,
            (_, _) => Rocks::Empty,
        }
    }

    fn set(&mut self, x: usize, y: usize, val: Rocks) {
        match val {
            Rocks::Round => {
                self.cube_rocks.unset(x, y);
                self.round_rocks.set(x, y);
            }
            Rocks::Cube => {
                self.round_rocks.unset(x, y);
                self.cube_rocks.set(x, y);
            }
            Rocks::Empty => {
                self.round_rocks.unset(x, y);
                self.cube_rocks.unset(x, y);
            }
        }
    }

    fn do_tilt(
        &mut self,
        outer_range: impl Iterator<Item = usize> + Clone,
        inner_range: impl Iterator<Item = usize> + Clone,
        cur: &dyn Fn(usize, usize) -> (usize, usize),
        next: &dyn Fn(usize, usize) -> (usize, usize),
    ) {
        let mut moves_made: usize;
        loop {
            moves_made = 0;

            for o in outer_range.clone() {
                for i in inner_range.clone() {
                    let (cur_x, cur_y) = cur(o, i);
                    let (next_x, next_y) = next(o, i);

                    if self.get(cur_x, cur_y) != Rocks::Round
                        || self.get(next_x, next_y) != Rocks::Empty
                    {
                        continue;
                    }

                    self.set(next_x, next_y, Rocks::Round);
                    self.set(cur_x, cur_y, Rocks::Empty);
                    moves_made += 1;
                }
            }

            if moves_made == 0 {
                break;
            }
        }
    }

    fn tilt(&mut self, direction: &Direction) {
        match direction {
            Direction::N => {
                self.do_tilt(0..=self.max_x, 1..=self.max_y, &|o, i| (o, i), &|o, i| {
                    (o, i - 1)
                })
            }
            Direction::E => self.do_tilt(
                0..=self.max_y,
                (0..self.max_x).rev(),
                &|o, i| (i, o),
                &|o, i| (i + 1, o),
            ),
            Direction::S => self.do_tilt(
                0..=self.max_x,
                (0..self.max_y).rev(),
                &|o, i| (o, i),
                &|o, i| (o, i + 1),
            ),
            Direction::W => {
                self.do_tilt(0..=self.max_y, 1..=self.max_x, &|o, i| (i, o), &|o, i| {
                    (i - 1, o)
                })
            }
        }
    }

    fn grade(&self) -> usize {
        let mut sum = 0;
        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                if self.round_rocks.present(x, y) {
                    sum += self.max_y - y + 1;
                }
            }
        }
        sum
    }
}

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                match self.get(x, y) {
                    Rocks::Round => f.write_str("O")?,
                    Rocks::Cube => f.write_str("#")?,
                    Rocks::Empty => f.write_str(".")?,
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

pub fn parse<'a>(data: &'a str) -> Table {
    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();
    let mut table = Table::new(width, height);

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    table.set(x, y, Rocks::Cube);
                }
                'O' => {
                    table.set(x, y, Rocks::Round);
                }
                '.' => (),
                x => unimplemented!("Unknown Char: {}", x),
            }
            table.max_x = table.max_x.max(x);
            table.max_y = table.max_y.max(y);
        }
    }

    table
}

pub fn part1<'a>(mut table: Table) -> usize {
    table.tilt(&Direction::N);
    table.grade()
}

pub fn part2<'a>(mut table: Table) -> usize {
    const NUM_TILTS: usize = 1000000000;
    let mut tilted: HashMap<Table, usize> = HashMap::new();

    for i in 1..NUM_TILTS {
        for direction in vec![Direction::N, Direction::W, Direction::S, Direction::E] {
            table.tilt(&direction);
        }
        if let Some(tilted_at) = tilted.insert(table.clone(), i) {
            if (NUM_TILTS - i) % (i - tilted_at) == 0 {
                break;
            }
        }
    }

    table.grade()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_sample_1() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part1(data), 136);
    }

    #[test]
    fn test_sample_2() {
        let data = parse(&SAMPLE_INPUT);
        assert_eq!(part2(data), 64);
    }

    #[test]
    fn parse0() {
        let data = parse(SAMPLE_INPUT);
        assert_eq!(format!("{:?}", data).trim(), SAMPLE_INPUT);
    }

    #[test]
    fn parse1() {
        let input = "O.O.O.#.#..O";
        let data = parse(input);
        assert_eq!(format!("{:?}", data).trim(), input);
    }

    #[test]
    fn parse2() {
        let input = "O\n.\nO\n.\nO\n.\n#\n.\n#\n.\n.\nO";
        let data = parse(input);
        assert_eq!(format!("{:?}", data).trim(), input);
    }

    #[test]
    fn bitmap1() {
        let mut bm = BitMap::new(2, 2);
        assert!(!bm.present(0, 0));
        bm.set(0, 0);
        assert!(bm.present(0, 0));
        bm.unset(0, 0);
        assert!(!bm.present(0, 0));

        assert!(!bm.present(1, 1));
        bm.set(1, 1);
        assert!(bm.present(1, 1));
        bm.unset(1, 1);
        assert!(!bm.present(1, 1));

        bm.set(0, 0);
        bm.set(0, 1);
        bm.set(1, 0);
        bm.set(1, 1);

        assert!(bm.present(0, 0));
        assert!(bm.present(0, 1));
        assert!(bm.present(1, 0));
        assert!(bm.present(1, 1));
    }

    generate_test! { 2023, 14, 1, 109596}
    generate_test! { 2023, 14, 2, 96105}
}
