use itertools::Itertools;
use parser::dig_pair;
use rayon::prelude::*;
use util::{Pos, Rect};
use winnow::{
    Parser,
    ascii::line_ending,
    combinator::{repeat, separated},
};

type InputType = Vec<Pos<usize>>;
type OutType = i64;

pub fn parse(data: &str) -> InputType {
    let (data, _): (InputType, Vec<_>) = (
        separated(
            1..,
            dig_pair::<usize>(",").map(|(x, y)| Pos::new(x, y)),
            line_ending,
        ),
        repeat(0.., line_ending),
    )
        .parse(data)
        .unwrap();

    data
}

pub fn part1(input: InputType) -> OutType {
    input
        .iter()
        .tuple_combinations()
        .par_bridge()
        .map(|(p1, p2)| {
            ((p1.x as i64).abs_diff(p2.x as i64) + 1) * ((p1.y as i64).abs_diff(p2.y as i64) + 1)
        })
        .max()
        .unwrap()
        .try_into()
        .unwrap()
}

/// Ray casting algorithm to check if a point is inside a polygon.
/// Casts a horizontal ray from the point to the right and counts edge crossings.
/// Odd number of crossings means the point is inside.
fn point_in_polygon(point: &Pos<usize>, polygon: &[Pos<usize>]) -> bool {
    let mut inside = false;
    let n = polygon.len();

    for i in 0..n {
        let p1 = &polygon[i];
        let p2 = &polygon[(i + 1) % n];

        // Check if the edge crosses the horizontal line at point.y
        let edge_crosses_ray = (p1.y > point.y) != (p2.y > point.y);

        if edge_crosses_ray {
            // Calculate the x-coordinate where the edge intersects the ray
            // Using the line equation: x = x1 + (y - y1) * (x2 - x1) / (y2 - y1)
            let x1 = p1.x as i64;
            let x2 = p2.x as i64;
            let y1 = p1.y as i64;
            let y2 = p2.y as i64;
            let y = point.y as i64;

            let intersection_x = x1 + (y - y1) * (x2 - x1) / (y2 - y1);

            // If intersection is to the right of the point, count it
            if (point.x as i64) < intersection_x {
                inside = !inside;
            }
        }
    }

    inside
}

pub fn part2(input: InputType) -> OutType {
    input
        .iter()
        .tuple_combinations()
        .filter_map(|(p1, p2)| {
            let rect = Rect::from_corners(p1.clone(), p2.clone());

            // Early exit 1: if any polygon vertex is strictly inside this rectangle (not on boundary)
            if input.iter().any(|point| rect.contains_strict(point)) {
                return None;
            }

            // Early exit 2: if any polygon edge passes through rectangle interior
            for (edge_p1, edge_p2) in input.iter().cycle().tuple_windows().take(input.len()) {
                if rect.segment_intersects_interior(edge_p1, edge_p2) {
                    return None;
                }
            }

            // Check if all four corners of rectangle are inside or on polygon boundary
            for corner in &rect.corners() {
                // Corner is valid if it's a polygon vertex (on boundary) OR inside polygon
                let is_vertex = input.iter().any(|v| v == corner);
                let is_inside = point_in_polygon(corner, &input);
                if !is_vertex && !is_inside {
                    return None;
                }
            }

            Some(rect.area() as i64)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    const TEST_INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    generate_test!(TEST_INPUT, 1, 50);

    generate_test!(TEST_INPUT, 2, 24);

    generate_test! { 2025, 9, 1, 4748985168}
    generate_test! { 2025, 9, 2, 1550760868}
}
