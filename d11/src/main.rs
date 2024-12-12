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

    let r = if stone == 0 {
        simulate_stone(1, limit - 1, cache)
    } else if let Some((s1, s2)) = split_stone(stone) {
        simulate_stone(s1, limit - 1, cache) + simulate_stone(s2, limit - 1, cache)
    } else {
        simulate_stone(stone * 2024, limit - 1, cache)
    };

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

fn count_stones_fast(stones: &Vec<i64>, limit: i64) -> usize {
    let mut hm = HashMap::<i64, usize>::new();
    for s in stones {
        *hm.entry(*s).or_default() += 1;
    }
    for _ in 0..limit {
        let mut hm_next = HashMap::new();
        for (stone, count) in &hm {
            let stone = *stone;
            let count = *count;

            if stone == 0 {
                *hm_next.entry(1).or_default() += count;
            } else if let Some((s1, s2)) = split_stone(stone) {
                *hm_next.entry(s1).or_default() += count;
                *hm_next.entry(s2).or_default() += count;
            } else {
                *hm_next.entry(stone * 2024).or_default() += count;
            };
        }
        hm = hm_next
    }

    hm.values().sum()
}

fn main() -> Result<()> {
    let stones = parse(Path::new("input.txt"))?;
    println!("Part 1 {}", count_stones(&stones, 25));

    println!("Part 2 {}", count_stones(&stones, 75));

    println!("Part 2 fast {}", count_stones_fast(&stones, 75));
    Ok(())
}
