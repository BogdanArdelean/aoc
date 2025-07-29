use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

fn parse(path: &Path) -> Result<Vec<i32>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut r = Vec::new();

    let line = reader.lines().next().unwrap()?;
    for nr in line.split(",") {
        r.push(nr.parse()?);
    }

    Ok(r)
}

fn solve(steps: i32, age: &Vec<i32>) -> i128 {
    let mut fish = 0;
    let mut buckets = [0; 9];
    for a in age {
        buckets[*a as usize] += 1;
    }

    for i in 0..steps {
        let new_fish = buckets[0];
        for j in 0..8 {
            buckets[j] = buckets[j + 1];
        }
        buckets[8] = new_fish;
        buckets[6] += new_fish;
    }

    for i in 0..9 {
        fish += buckets[i];
    }

    fish
}

fn main() -> Result<()> {
    let r = parse(&Path::new("input.txt"))?;
    let part_1 = solve(80, &r);
    let part_2 = solve(256, &r);
    println!("Part 1 {} \n Part 2 {}", part_1, part_2);
    Ok(())
}
