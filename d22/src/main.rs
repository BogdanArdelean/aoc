use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn generate_secret_number(mut number: i64, i: i64) -> Vec<i64> {
    let mut v = vec![number];

    for _ in 0..i {
        number = calculate_secret(number);
        v.push(number);
    }
    v
}

fn calculate_secret(mut number: i64) -> i64 {
    const MOD: i64 = 16777216;
    number = (number ^ number * 64) % MOD;
    number = (number ^ (number / 32)) % MOD;
    number = (number ^ (number * 2048)) % MOD;
    number
}

fn part1(codes: &Vec<i64>) -> i64 {
    let sum = codes
        .iter()
        .map(|s| generate_secret_number(*s, 2000).last().unwrap().clone())
        .sum();
    sum
}

fn part2(codes: &Vec<i64>) -> i64 {
    let mut global_map = HashMap::new();

    for &code in codes {
        let price: Vec<i64> = generate_secret_number(code, 2000)
            .iter()
            .map(|&x| x % 10)
            .collect();

        let mut local_map = HashMap::new();

        for window in price.windows(5) {
            let seq: Vec<i64> = window.windows(2).map(|w| w[1] - w[0]).collect();

            let cost = window[4];
            local_map.entry(seq).or_insert(cost);
        }

        for (seq, cost) in local_map {
            *global_map.entry(seq).or_insert(0) += cost;
        }
    }

    global_map.values().copied().max().unwrap_or(0)
}

fn parse(path: &Path) -> Result<Vec<i64>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect())
}
fn main() -> Result<()> {
    let numbers = parse(&Path::new("input.txt"))?;
    let p1 = part1(&numbers);
    println!("Part 1: {}", p1);

    let p2 = part2(&numbers);
    println!("Part 2: {}", p2);
    Ok(())
}
