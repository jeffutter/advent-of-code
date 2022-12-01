use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

struct OctopusGarden {
    octopi: HashMap<Point, u32>,
    width: usize,
    height: usize,
    recently_flashed: Vec<Point>,
    flashes: u32,
}

impl OctopusGarden {
    pub fn new() -> OctopusGarden {
        OctopusGarden {
            octopi: HashMap::new(),
            width: 0usize,
            height: 0usize,
            flashes: 0u32,
            recently_flashed: Vec::new(),
        }
    }

    fn add(&mut self, point: Point, val: u32) -> &Self {
        self.width = point.x.max(self.width);
        self.height = point.y.max(self.height);

        self.octopi.insert(point, val);

        self
    }

    fn flash(&mut self, point: &Point) -> &Self {
        let octopi = &mut self.octopi;

        match (self.recently_flashed.contains(point), octopi.get_mut(point)) {
            (true, _) => (),
            (false, Some(n)) if *n >= 9 => {
                self.recently_flashed.push(*point);
                self.flashes += 1;
                *n = 0;
                self.surrounding_points(*point).iter().for_each(|point| {
                    self.flash(point);
                });
            }
            (false, Some(n)) => *n += 1,
            (false, None) => (),
        }

        self
    }

    fn flash_all(&mut self) -> &Self {
        self.recently_flashed = Vec::new();

        for x in 0..=self.width {
            for y in 0..=self.height {
                self.flash(&Point::new(x, y));
            }
        }

        self
    }

    fn surrounding_points(&mut self, point: Point) -> Vec<Point> {
        let mut surrounding: Vec<Point> = Vec::new();
        let x = point.x as i32;
        let y = point.y as i32;

        [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .iter()
        .for_each(|point| {
            usize::try_from(point.0)
                .and_then(|x| usize::try_from(point.1).and_then(|y| Ok(Point::new(x, y))))
                .ok()
                .and_then(|point| {
                    if self.octopi.contains_key(&point) {
                        surrounding.push(point);
                    };
                    Some(())
                });
        });

        surrounding
    }
}

fn parse_garden(data: String) -> OctopusGarden {
    data.lines()
        .enumerate()
        .fold(OctopusGarden::new(), |garden, (y, row)| {
            row.char_indices().fold(garden, |mut garden, (x, c)| {
                garden.add(Point::new(x, y), c.to_digit(10).unwrap());
                garden
            })
        })
}

pub fn part1(data: String) -> u32 {
    let mut garden = parse_garden(data);

    for _ in 0..100 {
        garden.flash_all();
    }

    garden.flashes
}

pub fn part2(data: String) -> i32 {
    let mut garden = parse_garden(data);

    let mut counter = 0;

    while garden.recently_flashed.len() < garden.octopi.len() {
        garden.flash_all();
        counter += 1;
    }

    counter
}
