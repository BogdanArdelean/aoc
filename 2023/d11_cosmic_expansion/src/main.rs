use std::collections::{HashMap, VecDeque};
use std::default;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::{Path, Prefix};
use anyhow::{Ok, Result, anyhow};
use itertools::Itertools;

type Universe = Vec<Vec<char>>;
type Galaxy = (usize, usize);
type PrefixRows = Vec<u128>;
type PrefixCols = Vec<u128>;

fn find_galaxies(universe: &Universe) -> Vec<Galaxy> {
    let mut galaxies = Vec::<Galaxy>::new();
    for (x, row) in universe.iter().enumerate() {
        for (y, chr) in row.iter().enumerate() {
            if *chr == '#' {
                galaxies.push((x, y));
            }
        }
    }

    galaxies
}

fn get_expanding_map(universe: &Universe) -> (PrefixRows, PrefixCols) {
    let mut prefix_rows = PrefixRows::new();
    {
        let mut sum = 0;
        for row in universe {
            sum += row.iter().all(|c| *c == '.') as u128 * (1000000 - 1);
            prefix_rows.push(sum);
        }
    }
    let mut prefix_cols = PrefixCols::new();
    {
        let mut sum = 0;
        for i in 0..universe[0].len() {
            let mut all_ok = true;
            for j in 0..universe.len() {
                if universe[j][i] != '.' {
                    all_ok = false;
                    break;
                }
            }
            if all_ok {
                sum += 1000000 - 1;
            }
            prefix_cols.push(sum);
        }
    }

    (prefix_rows, prefix_cols)
}

fn get_sum(mut start: usize, mut stop: usize, prefix: &Vec<u128>) -> u128 {
    (start, stop) = if start > stop {
        (stop, start)
    } else {
        (start, stop)
    };

    prefix[stop] - prefix[start]
}

fn find_minimum_sums(galaxies: &Vec<Galaxy>, p_rows: &PrefixRows, p_cols: &PrefixCols) -> u128 {
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i+1..galaxies.len() {
            let source = galaxies[i];
            let dest = galaxies[j];
            let x_diff = (source.0 as i32 - dest.0 as i32).abs() as u128 + get_sum(source.0, dest.0, p_rows);
            let y_diff = (source.1 as i32 - dest.1 as i32).abs() as u128 + get_sum(source.1, dest.1, p_cols);
            let distance = x_diff + y_diff;

            println!("Distance {} -> {}: {}", i, j, distance);
            println!("X diff {}, Y diff {}, p_x: {}, p_y: {}", 
                (source.0 as i32 - dest.0 as i32).abs(), 
                (source.1 as i32 - dest.1 as i32).abs(), 
                get_sum(source.0, dest.0, p_rows),
                get_sum(source.1, dest.1, p_cols));

            sum += distance;
        }
    }
    sum
}

fn parse(path: &Path) -> Result<Universe> {
    let mut universe = Universe::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for row in reader.lines() {
        let row = row?; 
        universe.push(row.chars().collect());
    }

    Ok(universe)
}

fn main() -> Result<()> {
    let universe = parse(Path::new("input.txt"))?;
    let galaxies = find_galaxies(&universe);
    let (p_rows, p_cols) = get_expanding_map(&universe);
    let sum = find_minimum_sums(&galaxies, &p_rows, &p_cols);
    println!("Part 1: {}", sum);
    Ok(())
}
