use std::iter;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, not_line_ending},
    combinator::{eof, opt},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub fn part1<'a>(root: Directory) -> i32 {
    root.all_dirs()
        .map(|x| x.total_size())
        .filter(|x| x <= &100000)
        .sum()
}

pub fn part2<'a>(root: Directory) -> i32 {
    let total = root.total_size();
    let free = 70000000 - total;
    let to_delete = 30000000 - free;

    let mut big_enonugh: Vec<&Directory> = root
        .all_dirs()
        .filter(|x| x.total_size() > to_delete)
        .collect();

    big_enonugh.sort_by_key(|x| x.total_size());

    big_enonugh.first().unwrap().total_size()
}

pub fn parse<'a>(data: &'a str) -> Directory {
    let (_, root) = parse_directory(data).unwrap();

    root
}

pub fn parse_file(s: &str) -> IResult<&str, File> {
    let (rest, (size, _, _name)) = terminated(
        tuple((parser::from_dig, tag(" "), not_line_ending)),
        many0(line_ending),
    )(s)?;

    Ok((rest, File::new(size)))
}

pub fn parse_dir(s: &str) -> IResult<&str, &str> {
    let (rest, name) = terminated(preceded(tag("dir "), take_until("\n")), line_ending)(s)?;

    Ok((rest, name))
}

pub fn parse_directory_item(s: &str) -> IResult<&str, DirectoryItem> {
    let (rest_a, _) = many0(parse_dir)(s)?;

    let (rest, maybe_directory) = opt(parse_directory)(rest_a)?;

    if let Some(dir) = maybe_directory {
        let (rest_b, _) = many0(parse_dir)(rest)?;
        return Ok((rest_b, DirectoryItem::Directory(dir)));
    }

    let (rest, file) = parse_file(rest_a)?;
    let (rest_c, _) = many0(parse_dir)(rest)?;

    Ok((rest_c, DirectoryItem::File(file)))
}

pub fn parse_directory(s: &str) -> IResult<&str, Directory> {
    let (rest, (_, _dir_name, _, directory_items, _)) = terminated(
        tuple((
            tag("$ cd "),
            terminated(not_line_ending, line_ending),
            terminated(tag("$ ls"), line_ending),
            many0(parse_directory_item),
            alt((tag("$ cd .."), eof)),
        )),
        many0(line_ending),
    )(s)?;

    Ok((rest, Directory::new(directory_items)))
}

#[derive(Debug, Clone)]
pub enum DirectoryItem {
    File(File),
    Directory(Directory),
    Dir,
}

#[derive(Debug, Clone)]
pub struct File {
    size: i32,
}

impl File {
    pub fn new(size: i32) -> Self {
        Self { size }
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    items: Vec<DirectoryItem>,
}

impl Directory {
    pub fn new(items: Vec<DirectoryItem>) -> Self {
        Self { items }
    }

    pub fn file_size(&self) -> i32 {
        self.items
            .iter()
            .filter_map(|x| match x {
                DirectoryItem::File(file) => Some(file.size),
                _ => None,
            })
            .sum()
    }

    pub fn total_size(&self) -> i32 {
        self.file_size() + self.dirs().map(|x| x.total_size()).sum::<i32>()
    }

    pub fn dirs(&self) -> impl Iterator<Item = &Directory> {
        self.items.iter().filter_map(|x| match x {
            DirectoryItem::Directory(dir) => Some(dir),
            _ => None,
        })
    }

    pub fn all_dirs(&self) -> Box<dyn Iterator<Item = &Directory> + '_> {
        Box::new(iter::once(self).chain(self.dirs().map(|x| x.all_dirs()).flatten()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_size() {
        let input = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
        let parsed = parse(input);
        let res = part1(parsed);
        assert_eq!(res, 95437)
    }
}
