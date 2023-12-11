use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pipe {
    Vertical(Pos),
    Horizontal(Pos),
    NE(Pos),
    NW(Pos),
    SE(Pos),
    SW(Pos),
}

impl From<(char, Pos)> for Pipe {
    fn from(value: (char, Pos)) -> Self {
        match value.0 {
            '|' => Pipe::Vertical(value.1),
            '-' => Pipe::Horizontal(value.1),
            'L' => Pipe::NE(value.1),
            'J' => Pipe::NW(value.1),
            '7' => Pipe::SW(value.1),
            'F' => Pipe::SE(value.1),
            _ => unimplemented!(),
        }
    }
}

impl Pipe {
    fn possible_moves(&self, min: &Pos, max: &Pos) -> Vec<Pos> {
        let moves = match self {
            Pipe::Vertical(Pos { x, y }) => {
                vec![Pos::new(*x, y - 1), Pos::new(*x, y + 1)]
            }
            Pipe::Horizontal(Pos { x, y }) => {
                vec![Pos::new(x - 1, *y), Pos::new(x + 1, *y)]
            }
            Pipe::NE(Pos { x, y }) => {
                vec![Pos::new(x + 1, *y), Pos::new(*x, y - 1)]
            }
            Pipe::NW(Pos { x, y }) => {
                vec![Pos::new(x - 1, *y), Pos::new(*x, y - 1)]
            }
            Pipe::SE(Pos { x, y }) => {
                vec![Pos::new(x + 1, *y), Pos::new(*x, y + 1)]
            }
            Pipe::SW(Pos { x, y }) => {
                vec![Pos::new(x - 1, *y), Pos::new(*x, y + 1)]
            }
        };

        moves
            .into_iter()
            .filter(|Pos { x, y }| x >= &min.x && x <= &max.x && y >= &min.y && y <= &max.y)
            .collect_vec()
    }

    fn pos(&self) -> &Pos {
        match self {
            Pipe::Vertical(pos) => pos,
            Pipe::Horizontal(pos) => pos,
            Pipe::NE(pos) => pos,
            Pipe::NW(pos) => pos,
            Pipe::SE(pos) => pos,
            Pipe::SW(pos) => pos,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PipeSketch {
    pipes: HashMap<Pos, Pipe>,
    pipe_loop: HashMap<Pos, Pipe>,
    start: Pos,
    min: Pos,
    max: Pos,
    last: Option<Pos>,
    cur: Pos,
}

#[derive(Debug)]
struct NoLoopError {}

impl PipeSketch {
    fn mv(&mut self) {
        let possible_moves = self
            .pipes
            .get(&self.cur)
            .unwrap()
            .possible_moves(&self.min, &self.max);

        let next = match &self.last {
            None => possible_moves.first().unwrap().clone(),
            Some(last) => possible_moves
                .iter()
                .filter(|pos| *pos != last)
                .cloned()
                .next()
                .unwrap(),
        };

        self.last = Some(self.cur.clone());
        self.cur = next;
    }

    fn find_loop(&mut self) -> Result<(), NoLoopError> {
        let possible_start_pipes = vec![
            Pipe::Vertical(self.start.clone()),
            Pipe::Horizontal(self.start.clone()),
            Pipe::NW(self.start.clone()),
            Pipe::NE(self.start.clone()),
            Pipe::SW(self.start.clone()),
            Pipe::SE(self.start.clone()),
        ];

        let mut loop_found: bool = false;

        for possible_start in possible_start_pipes {
            let mut visited = vec![possible_start.clone()];
            let mut sketch = self.clone();
            sketch
                .pipes
                .insert(self.start.clone(), possible_start.clone());

            while sketch.last == None || sketch.cur != sketch.start {
                sketch.mv();

                if let Some(pipe) = sketch.pipes.get(&sketch.cur) {
                    if sketch.last != None
                        && !pipe
                            .possible_moves(&sketch.min, &sketch.max)
                            .contains(&sketch.last.clone().unwrap())
                    {
                        break;
                    }

                    visited.push(pipe.clone());
                    if pipe.pos() == &sketch.start {
                        loop_found = true;
                        break;
                    }
                } else {
                    break;
                }
            }

            if loop_found {
                self.pipes
                    .insert(possible_start.pos().clone(), possible_start);
                self.pipe_loop = self
                    .pipes
                    .iter()
                    .filter(|(_pos, pipe)| visited.contains(pipe))
                    .map(|(pos, pipe)| (pos.clone(), pipe.clone()))
                    .collect();
                return Ok(());
            }
        }

        Err(NoLoopError {})
    }
}

pub fn parse<'a>(data: &'a str) -> PipeSketch {
    let mut pipes = HashMap::new();
    let mut start: Option<Pos> = None;
    let mut max = Pos::new(0, 0);

    for (y, line) in data.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pos = Pos::new(x.try_into().unwrap(), y.try_into().unwrap());
            max = max.max(pos.clone());

            if char == '.' {
                continue;
            }

            if char == 'S' {
                start = Some(pos);
                continue;
            }

            pipes.insert(pos.clone(), (char, pos).into());
        }
    }

    PipeSketch {
        pipes,
        pipe_loop: HashMap::new(),
        start: start.clone().unwrap(),
        cur: start.clone().unwrap(),
        min: Pos::new(0, 0),
        max,
        last: None,
    }
}

pub fn part1<'a>(mut sketch: PipeSketch) -> i32 {
    sketch.find_loop().unwrap();

    (sketch.pipe_loop.len() / 2).try_into().unwrap()
}

pub fn part2<'a>(mut sketch: PipeSketch) -> usize {
    sketch.find_loop().unwrap();

    let inside = (sketch.min.x..=sketch.max.x)
        .cartesian_product(sketch.min.y..=sketch.max.y)
        .map(|(x, y)| Pos::new(x, y))
        .filter(|pos| {
            let empty = !sketch.pipe_loop.contains_key(pos);

            let crosses = (sketch.min.x..pos.x)
                .fold((0, 0, 0, 0, 0), |(ne, sw, se, nw, vert), x| {
                    let pos = Pos::new(x, pos.y);
                    match sketch.pipe_loop.get(&pos) {
                        Some(Pipe::Vertical(_)) => (ne, sw, se, nw, vert + 1),
                        Some(Pipe::NE(_)) => {
                            if sw > 0 {
                                (ne, sw - 1, se, nw, vert + 1)
                            } else {
                                (ne + 1, sw, se, nw, vert)
                            }
                        }
                        Some(Pipe::SW(_)) => {
                            if ne > 0 {
                                (ne - 1, sw, se, nw, vert + 1)
                            } else {
                                (ne, sw + 1, se, nw, vert)
                            }
                        }
                        Some(Pipe::NW(_)) => {
                            if se > 0 {
                                (ne, sw, se - 1, nw, vert + 1)
                            } else {
                                (ne, sw, se, nw + 1, vert)
                            }
                        }
                        Some(Pipe::SE(_)) => {
                            if nw > 0 {
                                (ne, sw, se, nw - 1, vert + 1)
                            } else {
                                (ne, sw, se + 1, nw, vert)
                            }
                        }
                        Some(_) => (ne, sw, se, nw, vert),
                        None => (ne, sw, se, nw, vert),
                    }
                })
                .4;

            let even_crosses = crosses % 2 == 0;
            empty && !even_crosses
        })
        .collect_vec();

    inside.len()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const SAMPLE_INPUT1: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

    #[test]
    fn test_sample_1_1() {
        let data = parse(&SAMPLE_INPUT1);
        assert_eq!(part1(data), 4);
    }

    const SAMPLE_INPUT2: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

    #[test]
    fn test_sample_1_2() {
        let data = parse(&SAMPLE_INPUT2);
        assert_eq!(part1(data), 8);
    }

    const SAMPLE_INPUT3: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

    #[test]
    fn test_sample_2_1() {
        let data = parse(&SAMPLE_INPUT3);
        assert_eq!(part2(data), 4);
    }

    const SAMPLE_INPUT4: &str = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#;

    #[test]
    fn test_sample_2_2() {
        let data = parse(&SAMPLE_INPUT4);
        assert_eq!(part2(data), 4);
    }

    const SAMPLE_INPUT5: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

    #[test]
    fn test_sample_2_3() {
        let data = parse(&SAMPLE_INPUT5);
        assert_eq!(part2(data), 8);
    }

    const SAMPLE_INPUT6: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

    #[test]
    fn test_sample_2_4() {
        let data = parse(&SAMPLE_INPUT6);
        assert_eq!(part2(data), 10);
    }

    generate_test! { 2023, 10, 1, 7066}
    generate_test! { 2023, 10, 2, 401}
}
