use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{anyhow, Result};

fn parse(path: &Path) -> Result<Vec<i32>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;

    line
        .split(",")
        .map(|s|s.parse().map_err(|o|anyhow!("parse")))
        .collect::<Result<Vec<i32>>>()
}

fn part_1(crabs: &Vec<i32>) -> i32 {
    let mx = crabs.into_iter().max().copied().unwrap() as usize + 1;
    let mut scores = vec![0; mx];

    for crab in crabs {
        for i in 0..mx {
            scores[i] += (*crab - i as i32).abs();
        }
    }

    scores.iter().min().copied().unwrap()
}

fn part_2(crabs: &Vec<i32>) -> i32 {
    let mx = crabs.into_iter().max().copied().unwrap() as usize + 1;
    let mut scores = vec![0; mx];

    for crab in crabs {
        for i in 0..mx {
            let distance = (*crab - i as i32).abs();
            scores[i] += (distance * (distance + 1)) / 2;
        }
    }

    scores.iter().min().copied().unwrap()
}

fn main() -> Result<()> {
    let crabs = parse(&Path::new("input.txt"))?;
    let rp1 = part_1(&crabs);
    println!("Part 1 {}", rp1);
    let rp2 = part_2(&crabs);
    println!("Part 2 {}", rp2);
    Ok(())
}
