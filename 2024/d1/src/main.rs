use anyhow::Result;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse(path: &Path) -> Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut l1 = vec![];
    let mut l2 = vec![];

    for line in reader.lines() {
        let line = line?;
        let (e1, e2) = scan_fmt!(&line, "{} {}", i32, i32)?;
        l1.push(e1);
        l2.push(e2);
    }

    Ok((l1, l2))
}

fn part1(mut l1: Vec<i32>, mut l2: Vec<i32>) -> i32 {
    l1.sort();
    l2.sort();

    l1.iter().zip(l2.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn part2(l1: Vec<i32>, l2: Vec<i32>) -> i64 {
    let mut map = HashMap::<i32, i64>::new();

    for e in &l2 {
        map.entry(*e).and_modify(|x| *x += 1).or_insert(1);
    }

    l1.iter()
        .map(|e| *e as i64 * map.get(e).copied().unwrap_or_default())
        .sum()
}

fn main() -> Result<()> {
    let (l1, l2) = parse(&Path::new("input.txt"))?;
    let p1 = part1(l1.clone(), l2.clone());
    println!("Part 1 {}", p1);

    let p2 = part2(l1, l2);
    println!("Part 2 {}", p2);
    Ok(())
}
