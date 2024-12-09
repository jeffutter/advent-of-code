use std::{
    iter,
    ops::{Index, IndexMut},
};

type InputType = Disk;
type OutType = usize;

#[derive(Clone)]
pub enum Block {
    Data(usize, usize),
    Free(usize),
}

impl Block {
    fn explode(&self) -> impl Iterator<Item = Self> {
        match self {
            Block::Data(i, n) => iter::repeat_n(Block::Data(*i, 1), *n),
            Block::Free(n) => iter::repeat_n(Block::Free(1), *n),
        }
    }
}

#[derive(Clone)]
pub struct Disk(Vec<Block>);

impl FromIterator<Block> for Disk {
    fn from_iter<T: IntoIterator<Item = Block>>(iter: T) -> Self {
        Disk(iter.into_iter().collect())
    }
}

#[allow(unused_variables)]
pub fn parse(data: &str) -> InputType {
    data.trim_end()
        .chars()
        .enumerate()
        .map(|(idx, c)| {
            if idx % 2 == 0 {
                let n = idx / 2;
                Block::Data(n, c.to_string().parse().unwrap())
            } else {
                Block::Free(c.to_string().parse().unwrap())
            }
        })
        .collect()
}

impl Index<usize> for Disk {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Disk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Disk {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b);
    }

    fn checksum(&self) -> usize {
        self.0
            .iter()
            .flat_map(|block| block.explode())
            .enumerate()
            .map(|(idx, b)| match b {
                Block::Data(n, _) => idx * n,
                Block::Free(_) => 0,
            })
            .sum()
    }
}

#[allow(unused_variables)]
pub fn part1(input: InputType) -> OutType {
    let mut disk: Disk = input.0.iter().flat_map(|block| block.explode()).collect();

    let mut s = 0;
    let mut e = disk.len() - 1;

    while s < e {
        match disk[s] {
            Block::Data(_, _) => (),
            Block::Free(_) => loop {
                match disk[e] {
                    Block::Free(_) => (),
                    Block::Data(_, _) => {
                        disk.swap(s, e);
                        break;
                    }
                }
                e -= 1;
            },
        }
        s += 1;
    }

    disk.checksum()
}

#[allow(unused_variables)]
pub fn part2(mut disk: InputType) -> OutType {
    let mut e = disk.len() - 1;

    while e > 0 {
        match disk[e] {
            Block::Free(_) => (),
            Block::Data(i, width) => {
                let empty_idx = (0..e).find_map(|idx| {
                    if let Block::Free(n) = disk[idx] {
                        if n >= width {
                            return Some((idx, n));
                        }
                    }
                    None
                });

                match empty_idx {
                    Some((idx, free_width)) if free_width == width => {
                        disk.swap(idx, e);
                    }
                    Some((idx, free_width)) if free_width > width => {
                        disk[idx] = disk[e].clone();
                        disk[e] = Block::Free(width);
                        disk.0.insert(idx + 1, Block::Free(free_width - width));
                        e += 1
                    }
                    _ => (),
                }
            }
        }
        e -= 1;
    }

    disk.checksum()
}

#[cfg(test)]
mod tests {
    use util::generate_test;

    #[test]
    fn example_0() {
        let data = parse(r#"12345"#);
        assert_eq!(part1(data), 62)
    }

    generate_test!(r#"2333133121414131402"#, 1, 1928);

    generate_test!(r#"2333133121414131402"#, 2, 2858);

    generate_test! { 2024, 9, 1, 6360094256423}
    generate_test! { 2024, 9, 2, 6379677752410}
}
