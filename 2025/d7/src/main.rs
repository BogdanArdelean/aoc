use std::collections::{HashMap};
use std::path::Path;

fn parse(path: &Path) -> Vec<Vec<char>> {
    std::fs::read_to_string(path).unwrap()
        .split('\n')
        .take_while(|s| s.len() > 0)
        .map(|s| s.chars().collect())
        .collect()
}

fn solve(grid: &Vec<Vec<char>>) -> (u64, u64) {
    let mut hs = HashMap::new();
    let mut splits = 0;
    for (j, e) in grid[0].iter().enumerate() {
        if *e == 'S' {
            hs.insert(j, 1);
            break;
        }
    }

    for (i, row) in grid.iter().enumerate() {
        if i == 0 { continue; }
        for (j, elm) in row.iter().enumerate() {
            if *elm == '^' && hs.contains_key(&j) {
                splits += 1;
                let unique_paths = hs[&j];
                hs.remove(&j);
                *hs.entry(j + 1).or_insert(0) += unique_paths;
                *hs.entry(j - 1).or_insert(0) += unique_paths;
            }
        }
    }

    (splits, hs.values().sum())
}

fn main() {
    let grid = parse(Path::new("input.txt"));
    let (p1, p2) = solve(&grid);
    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}
