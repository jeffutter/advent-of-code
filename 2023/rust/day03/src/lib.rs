use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance(&self, point: &Point) -> f64 {
        (((point.x as f64) - (self.x as f64)).powf(2.0)
            + ((point.y as f64) - (self.y as f64)).powf(2.0))
        .sqrt()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Item {
    point: Point,
    num: Option<usize>,
}

impl Item {
    fn new(x: usize, y: usize) -> Self {
        Self {
            point: Point { x, y },
            num: None,
        }
    }

    fn add_dig(&mut self, dig: usize) {
        match self.num {
            None => self.num = Some(dig),
            Some(old) => self.num = Some((old * 10) + dig),
        }
    }

    fn len(&self) -> usize {
        match self.num {
            None => 0,
            Some(dig) => dig.to_string().len(),
        }
    }

    fn self_points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        (self.point.x..(self.point.x + self.len())).map(|x| Point { x, y: self.point.y })
    }

    fn adjacent(&self, point: &Point) -> bool {
        self.self_points()
            .any(|self_point| self_point.distance(point) <= (2.0 as f64).sqrt())
    }
}

#[derive(Debug)]
pub struct Schematic {
    items: Vec<Item>,
    symbols: HashMap<Point, char>,
}

impl Schematic {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            symbols: HashMap::new(),
        }
    }

    fn add_symbol(&mut self, x: usize, y: usize, symbol: char) {
        let point = Point { x, y };
        self.symbols.insert(point, symbol);
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

pub fn parse<'a>(data: &'a str) -> Schematic {
    let mut schematic = Schematic::new();
    let mut cur_item: Option<Item> = None;

    let maybe_add_and_reset_cur =
        |schematic: &mut Schematic, cur_item: &mut Option<Item>| match &cur_item {
            None => (),
            Some(item) => {
                schematic.add_item(item.clone());
                *cur_item = None;
            }
        };

    for (y, line) in data.lines().enumerate() {
        maybe_add_and_reset_cur(&mut schematic, &mut cur_item);

        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => {
                    maybe_add_and_reset_cur(&mut schematic, &mut cur_item);
                }
                '0'..='9' => {
                    let dig = char.to_digit(10).unwrap();
                    match &mut cur_item {
                        None => {
                            let mut new_item = Item::new(x, y);
                            new_item.add_dig(dig as usize);
                            cur_item = Some(new_item);
                        }
                        Some(item) => {
                            item.add_dig(dig as usize);
                        }
                    }
                }
                symbol => {
                    maybe_add_and_reset_cur(&mut schematic, &mut cur_item);
                    schematic.add_symbol(x, y, symbol)
                }
            }
        }
    }

    schematic
}

pub fn part1<'a>(input: Schematic) -> usize {
    let mut total = 0;
    for item in input.items {
        for (symbol_point, _symbol) in input.symbols.iter() {
            if item.adjacent(&symbol_point) {
                match item.num {
                    None => (),
                    Some(num) => {
                        total += num;
                    }
                }
                break;
            }
        }
    }

    total
}

pub fn part2<'a>(input: Schematic) -> usize {
    let mut total = 0;
    for (symbol_point, symbol) in input.symbols.iter() {
        if *symbol != '*' {
            continue;
        }
        let geared: Vec<Item> = input
            .items
            .iter()
            .filter(|item| item.adjacent(&symbol_point))
            .cloned()
            .collect();

        if geared.len() != 2 {
            continue;
        }

        total += geared.first().unwrap().num.unwrap() * geared.last().unwrap().num.unwrap();
    }

    total
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    #[test]
    fn test_sample() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let data = parse(&input);
        assert_eq!(part1(data), 4361);
    }

    generate_test! { 2023, 3, 1, 519444}
    generate_test! { 2023, 3, 2, 74528807}
}
