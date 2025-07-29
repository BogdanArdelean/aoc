use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

fn parse(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut str = String::new();
    reader.read_to_string(&mut str)?;
    Ok(str)
}

fn extract_mul(input: &str) -> Vec<(i32, i32)> {
    let mut v = vec![];
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    for c in re.captures_iter(input) {
        let x = c.get(1).unwrap().as_str();
        let y = c.get(2).unwrap().as_str();
        v.push((x.parse().unwrap(), y.parse().unwrap()));
    }

    v
}

fn extract_mul_with_enable(input: &str) -> Vec<(i32, i32)> {
    let mut v = vec![];
    let re = Regex::new(
        r"(?P<mul>mul\((?P<a>[0-9]+),(?P<b>[0-9]+)\))|(?P<dont>don't\(\))|(?P<do>do\(\))",
    )
    .unwrap();
    let mut is_enabled = true;
    for c in re.captures_iter(input) {
        if let Some(_) = c.name("dont") {
            is_enabled = false;
            continue;
        }
        if let Some(_) = c.name("do") {
            is_enabled = true;
            continue;
        }

        if !is_enabled {
            continue;
        }

        let x = c.name("a").unwrap().as_str();
        let y = c.name("b").unwrap().as_str();
        v.push((x.parse().unwrap(), y.parse().unwrap()));
    }

    v
}

fn part1(muls: &[(i32, i32)]) -> i32 {
    muls.iter().map(|(a, b)| a * b).sum()
}

fn main() -> Result<()> {
    let s = parse(&Path::new("input.txt"))?;

    let muls = extract_mul(&s);
    println!("Part 1 {}", part1(&muls));

    let muls2 = extract_mul_with_enable(&s);
    println!("Part 2 {}", part1(&muls2));
    Ok(())
}
