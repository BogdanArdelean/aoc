use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

fn parse(path: &Path) -> Result<(HashSet<String>, Vec<String>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let towels = lines.next().unwrap()?.split(", ").map(|s| s.to_string()).collect();
    lines.next();
    let patterns = lines.map(|l| l.unwrap()).collect();

    Ok((towels, patterns))
}

fn matches<'a>(pattern: &'a str, towels: &HashSet<String>, max_sz: usize, memo: &mut HashMap<&'a str, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(v) = memo.get(pattern) {
        return *v;
    }

    let mut sum = 0;
    for i in (1..=max_sz).rev() {
        if pattern.len() >= i && towels.contains(&pattern[0..i]) {
            sum += matches(&pattern[i..], towels, max_sz, memo);
        }
    }

    memo.insert(pattern, sum);
    sum
}

fn count(towels: &HashSet<String>, patterns: &Vec<String>) -> Vec<usize> {
    let max_sz = towels.iter().map(|s| s.len()).max().unwrap();
    patterns
        .iter()
        .map(|p|matches(p, &towels, max_sz, &mut HashMap::new()))
        .collect()
}

fn main() -> Result<()> {
    let (towels, patterns) = parse(Path::new("input.txt"))?;
    let r = count(&towels, &patterns);
    let p1: usize = r.iter().filter(|c| **c > 0).count();
    let p2: usize = r.iter().sum();

    println!("Part 1: {}", p1);

    println!("Part 2: {}", p2);
    Ok(())
}
