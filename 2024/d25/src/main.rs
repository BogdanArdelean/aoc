use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

fn compact(kl: &Vec<Vec<char>>) -> Vec<i64> {
    let mut v = vec![-1; kl[0].len()];
    for i in 0..kl[0].len() {
        for j in 0..kl.len() {
            if kl[j][i] == '#' {
                v[i] += 1;
            }
        }
    }
    v
}

fn parse(path: &Path) -> Result<(Vec<Vec<i64>>, Vec<Vec<i64>>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut keys = vec![];
    let mut locks = vec![];

    let mut lines = reader.lines();
    loop {
        let kl: Vec<Vec<char>> = lines.by_ref().take(7).map(|s| s.unwrap().chars().collect()).collect();
        let c = compact(&kl);
        if kl[0][0] == '#' {
            locks.push(c);
        } else {
            keys.push(c);
        }

        if lines.next().is_none() { break; }
    }
    Ok((keys, locks))
}

fn is_fit(key: &Vec<i64>, lock: &Vec<i64>) -> bool {
    key.iter().zip(lock).all(|(a, b)| a+b <= 5)
}

fn part1(keys: &Vec<Vec<i64>>, locks: &Vec<Vec<i64>>) -> usize {
    let mut fits = 0;
    for key in keys {
        for lock in locks {
            fits += is_fit(key, lock) as usize;
        }
    }
    fits
}

fn main() -> Result<()> {
    let (keys, locks) = parse(&Path::new("input.txt"))?;
    let p1 = part1(&keys, &locks);
    println!("Part 1: {}", p1);
    Ok(())
}
