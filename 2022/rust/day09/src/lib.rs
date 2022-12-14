use std::{collections::HashSet, fmt};

pub fn part1<'a>(moves: Vec<Move>) -> usize {
    let mut state = State::new(2);

    for mv in moves {
        state.apply_move(&mv);
    }

    state.tail_locations.len()
}

pub fn part2<'a>(moves: Vec<Move>) -> usize {
    let mut state = State::new(10);

    for mv in moves {
        state.apply_move(&mv);
    }

    state.tail_locations.len()
}

pub fn parse<'a>(data: &'a str) -> Vec<Move> {
    data.lines().map(Move::from_str).collect()
}

#[derive(Debug)]
pub enum Move {
    Right(i32),
    Up(i32),
    Left(i32),
    Down(i32),
}

impl Move {
    pub fn from_str(s: &str) -> Self {
        match s.split(" ").collect::<Vec<&str>>()[..] {
            [c, n] => {
                let spaces: i32 = n.parse().unwrap();
                match c {
                    "R" => Move::Right(spaces),
                    "L" => Move::Left(spaces),
                    "U" => Move::Up(spaces),
                    "D" => Move::Down(spaces),
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }

    fn moves(&self) -> i32 {
        match self {
            Move::Right(x) => *x,
            Move::Up(x) => *x,
            Move::Left(x) => *x,
            Move::Down(x) => *x,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone)]
struct State {
    knots: Vec<Point>,
    tail_locations: HashSet<Point>,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..10).rev() {
            write!(f, "\n")?;
            for x in 0..10 {
                let v = self.knots.iter().enumerate().find_map(|(i, p)| {
                    if *p == Point::new(x, y) {
                        return Some(i);
                    } else {
                        None
                    }
                });

                match v {
                    Some(x) => write!(f, "{}", x)?,
                    None => write!(f, ".")?,
                }
            }
        }
        Ok(())
    }
}

impl State {
    pub fn new(size: usize) -> Self {
        let mut tail_locations = HashSet::new();
        tail_locations.insert(Point::new(0, 0));

        Self {
            knots: vec![Point::new(0, 0); size],
            tail_locations,
        }
    }

    fn apply_move(&mut self, mv: &Move) -> &mut Self {
        for _ in 0..mv.moves() {
            let mut head = self.knots.get_mut(0).unwrap();
            match mv {
                Move::Up(_) => head.y += 1,
                Move::Right(_) => head.x += 1,
                Move::Down(_) => head.y -= 1,
                Move::Left(_) => head.x -= 1,
            };

            let mut peekable = self.knots.iter_mut().peekable();

            while let Some(prev) = peekable.next() {
                if let Some(knot) = peekable.peek_mut() {
                    if prev.x.abs_diff(knot.x) <= 1 && prev.y.abs_diff(knot.y) <= 1 {
                        continue;
                    }

                    let x_diff = (prev.x - knot.x).signum();
                    let y_diff = (prev.y - knot.y).signum();

                    knot.x += x_diff;
                    knot.y += y_diff;
                }
            }

            self.tail_locations
                .insert(self.knots.last().unwrap().clone());
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tail_space_count() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        let parsed = parse(input);
        let res = part1(parsed);
        assert_eq!(13, res)
    }

    #[test]
    fn test_many_knot_tail_space_count1() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
        let parsed = parse(input);
        let res = part2(parsed);
        assert_eq!(1, res)
    }

    #[test]
    fn test_many_knot_tail_space_count2() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        let parsed = parse(input);
        let res = part2(parsed);
        assert_eq!(36, res)
    }
}
