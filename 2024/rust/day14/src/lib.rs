use std::fmt::Write;

use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
    sequence::tuple,
};
use parser::{signed_dig_pair, signed_point};
use util::Pos;

type InputType = Robots;
type OutType = usize;

#[derive(Debug)]
pub struct Robot {
    p: Pos<i32>,
    v: (i32, i32),
}

pub struct Robots {
    width: i32,
    height: i32,
    bots: Vec<Robot>,
}

impl std::fmt::Debug for Robots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self
                    .bots
                    .iter()
                    .filter(|robot| robot.p.x == x && robot.p.y == y)
                    .count();
                if c > 0 {
                    write!(f, "{}", c)?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Robots {
    pub fn safety_score(&self) -> usize {
        let left = 0..=(self.width / 2) - 1;
        let right = (self.width / 2) + 1..=self.width - 1;
        let top = 0..=(self.height / 2) - 1;
        let bottom = (self.height / 2) + 1..=self.height - 1;

        let q1 = (left.clone(), top.clone());
        let q2 = (right.clone(), top);
        let q3 = (left, bottom.clone());
        let q4 = (right, bottom);

        [q1, q2, q3, q4]
            .iter()
            .map(|(xrange, yrange)| {
                self.bots
                    .iter()
                    .filter(|robot| xrange.contains(&robot.p.x) && yrange.contains(&robot.p.y))
                    .count()
            })
            .product()
    }

    pub fn move_n(&mut self, n: i32) {
        for robot in self.bots.iter_mut() {
            robot.p.x = ((robot.p.x + self.width) + ((robot.v.0 * n) % self.width)) % self.width;
            robot.p.y = ((robot.p.y + self.height) + ((robot.v.1 * n) % self.height)) % self.height;
        }
    }
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    let (rest, bots) = separated_list1(
        newline,
        map(
            tuple((
                tag("p="),
                signed_point(","),
                tag(" v="),
                signed_dig_pair(","),
            )),
            |(_, p, _, v)| Robot { p, v },
        ),
    )(data)
    .unwrap();

    assert_eq!("", rest.trim());

    Robots {
        width: 101,
        height: 103,
        bots,
    }
}

#[allow(unused_variables)]
pub fn part1(mut robots: InputType) -> OutType {
    robots.move_n(100);
    robots.safety_score()
}

#[allow(unused_variables)]
pub fn part2(mut robots: InputType) -> OutType {
    let mut iter = 0;
    let mut lowest_safety_score = usize::MAX;
    let mut lowest_safety_score_iter = 0;
    let mut since_last_lowest = 0;

    loop {
        iter += 1;
        since_last_lowest += 1;

        robots.move_n(1);

        let safety_score = robots.safety_score();
        if safety_score < lowest_safety_score {
            lowest_safety_score = safety_score;
            lowest_safety_score_iter = iter;

            // println!("\nIter: {iter}\n");
            // println!("{:?}", robots);
            // println!();
        }

        if since_last_lowest >= 10000 {
            break;
        }
    }

    lowest_safety_score_iter
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    //     generate_test!(
    //         r#"p=0,4 v=3,-3
    // p=6,3 v=-1,-3
    // p=10,3 v=-1,2
    // p=2,0 v=2,-1
    // p=0,0 v=1,3
    // p=3,0 v=-2,-2
    // p=7,6 v=-1,-3
    // p=3,0 v=-1,-2
    // p=9,3 v=2,3
    // p=7,3 v=-1,2
    // p=2,4 v=2,-3
    // p=9,5 v=-3,-3"#,
    //         // r#"p=2,4 v=2,-3"#,
    //         1,
    //         12
    //     );

    //     generate_test!(
    //         r#"p=0,4 v=3,-3
    // p=6,3 v=-1,-3
    // p=10,3 v=-1,2
    // p=2,0 v=2,-1
    // p=0,0 v=1,3
    // p=3,0 v=-2,-2
    // p=7,6 v=-1,-3
    // p=3,0 v=-1,-2
    // p=9,3 v=2,3
    // p=7,3 v=-1,2
    // p=2,4 v=2,-3
    // p=9,5 v=-3,-3"#,
    //         2,
    //         0
    //     );

    generate_test! { 2024, 14, 1, 231852216}
    generate_test! { 2024, 14, 2, 8159}
}
