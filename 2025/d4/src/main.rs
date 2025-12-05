use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse(path: &Path) -> Vec<Vec<char>> {
    let reader = BufReader::new(File::open(path).unwrap());
    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn count_rolls(grid: &Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let di = vec![-1, -1, -1, 0, 1, 1, 1, 0];
    let dj = vec![-1, 0, 1, 1, 1, 0, -1, -1];

    let mut sum = 0;

    for idx in 0..di.len() {
        let ii = i + di[idx];
        let jj = j + dj[idx];
        if ii < 0 || ii >= grid.len() as i32
            || jj < 0 || jj >= grid[0].len() as i32 {
            continue;
        }

        sum += (grid[ii as usize][jj as usize] == '@') as i32;
    }

    sum
}

fn part1(grid: &Vec<Vec<char>>, max_rolls: i32) -> i32 {
    let mut free = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, elm) in row.iter().enumerate() {
            free += (*elm == '@' && count_rolls(grid, i as i32, j as i32) < max_rolls) as i32;
        }
    }

    free
}

fn part2(mut grid: Vec<Vec<char>>, max_rolls: i32) -> i32 {
    let mut free = 0;
    let mut stop = false;

    let rows = grid.len();
    let cols = grid[0].len();

    while !stop {
        stop = true;
        for i in 0..rows {
            for j in 0..cols {
                let elm = grid[i][j];
                if elm == '@' && count_rolls(&grid, i as i32, j as i32) < max_rolls {
                    grid[i][j] = '.';
                    free += 1;
                    stop = false;
                }
            }
        }
    }

    free
}

fn main() {
    let grid = parse(Path::new("input.txt"));
    let p1 = part1(&grid, 4);
    println!("part 1: {}", p1);

    let p2 = part2(grid, 4);
    println!("part 2: {}", p2);
}
