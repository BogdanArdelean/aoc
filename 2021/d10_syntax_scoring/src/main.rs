use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

enum Error {
    Expected(char),
    Incomplete(String)
}

fn get_mistake(s: &str) -> Error {
    let com = HashMap::from([
        (')', '('),
        ('>', '<'),
        ('}', '{'),
        (']', '['),
    ]);

    let mut stack = VecDeque::<char>::new();
    for chr in s.chars() {
        if let Some(v) = com.get(&chr) {
            if v != stack.back().unwrap() {
                return Error::Expected(chr);
            }

            stack.pop_back();
        } else {
            stack.push_back(chr);
        }
    }

    Error::Incomplete(stack.iter().collect())
}
fn part_1(lines: &Vec<String>) -> i32 {
    let scores = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    lines
        .iter()
        .filter_map(|x|match get_mistake(x) {
            Error::Expected(c) => Some(c),
            Error::Incomplete(_) => None
        })
        .filter_map(|x|scores.get(&x))
        .sum()
}

fn part_2(lines: &Vec<String>) -> i64 {
    let scores = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let mut scores: Vec<i64> =
    lines
        .iter()
        .filter_map(|x|match get_mistake(x) {
            Error::Expected(_) => None,
            Error::Incomplete(s) => Some(s)
        })
        .map(|s|
            s.chars()
                .rev()
                .fold(0, |acc, chr| acc * 5 + scores.get(&chr).unwrap()))
        .collect();

    scores.sort();
    scores.get(scores.len() / 2).unwrap().clone()
}

fn main() -> Result<()> {

    let lines = parse(&Path::new("input.txt"))?;
    let p1 = part_1(&lines);
    println!("Part 1 {}", p1);

    let p2 = part_2(&lines);
    println!("Part 2 {}", p2);
    Ok(())
}
