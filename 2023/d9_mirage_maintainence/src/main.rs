use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use scan_fmt::scan_fmt;
use anyhow::{anyhow, Result};
use itertools::{TupleWindows, Itertools};

fn compute_part1(history: &Vec<i64>) -> i64 {
    let mut sum = 0;
    let mut current = history.clone();

    while current.len() > 0 && !current.iter().all(|item| *item == 0) {
        sum += current.last().unwrap_or(&0);
        
        let mut aux = Vec::<i64>::new();
        for i in 1..current.len() {
            aux.push(current[i] - current[i - 1]);
        }

        
        current = aux;
    }

    sum
}

fn parse(path: &Path) -> Result<Vec<Vec<i64>>> {
    let mut result = Vec::<Vec<i64>>::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for l in reader.lines() {
        let l = l?;
        let mut history = Vec::<i64>::new();
        for nr in l.split(" ") {
            history.push(nr.parse()?);
        }
        result.push(history);
    }

    anyhow::Ok(result)
}

fn main() -> Result<()> {
    let res = parse(Path::new("input.txt"))?;
    
    let mut sum = 0;
    for history in &res {
        sum += compute_part1(&history);
    }

    println!("Part 1: {}", sum);

    let mut sum = 0;
    for mut history in res {
        history.reverse();
        sum += compute_part1(&history);
    }

    println!("Part 2: {}", sum);

    anyhow::Ok(())
}
