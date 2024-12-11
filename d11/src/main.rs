use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn split_stone(stone: i64) -> Option<(i64, i64)> {
    let mut count = 0;
    let mut stone_aux = stone;
    while stone_aux > 0 {
        stone_aux /= 10;
        count += 1;
    }

    if count % 2 == 0 {
        let a = stone % 10_i64.pow(count / 2);
        let b = stone / 10_i64.pow(count / 2);
        Some((a, b))
    } else {
        None
    }
}

fn simulate_stone(stone: i64, limit: i64, cache: &mut HashMap<(i64, i64), usize>) -> usize {
    if let Some(cnt) = cache.get(&(stone, limit)) {
        return *cnt;
    }

    if limit <= 0 {
        return 1;
    }

    if stone == 0 {
        let r = simulate_stone(1, limit - 1, cache);
        cache.insert((stone, limit), r);
        return r;
    }

    if let Some((s1, s2)) = split_stone(stone) {
        let r = simulate_stone(s1, limit - 1, cache) + simulate_stone(s2, limit - 1, cache);
        cache.insert((stone, limit), r);
        return r;
    }

    let r = simulate_stone(stone * 2024, limit - 1, cache);
    cache.insert((stone, limit), r);
    r
}

fn parse(path: &Path) -> Result<Vec<i64>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let r = reader
        .lines()
        .next()
        .unwrap()?
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    Ok(r)
}

fn count_stones(stones: &Vec<i64>, limit: i64) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|s| simulate_stone(*s, limit, &mut cache))
        .sum()
}

fn main() -> Result<()> {
    let stones = parse(Path::new("input.txt"))?;
    println!("Part 1 {}", count_stones(&stones, 25));

    println!("Part 2 {}", count_stones(&stones, 75));
    Ok(())
}
