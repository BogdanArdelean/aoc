use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Grid = Vec<Vec<char>>;
fn find_start(m: &Grid) -> (usize, usize) {
    for (x_i, row) in m.iter().enumerate() {
        for (y_i, col) in row.iter().enumerate() {
            if *col == '^' {
                return (x_i, y_i);
            }
        }
    }

    panic!("Can't find start position!");
}

fn step(
    m: &Grid,
    (x, y, mut d_idx): (usize, usize, usize),
) -> Option<(usize, usize, usize)> {
    const DD: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut x = x as i32;
    let mut y = y as i32;

    let (dx, dy) = DD[d_idx];
    x += dx;
    y += dy;

    if !((x as usize) < m.len() && (y as usize) < m[0].len()) {
        return None;
    }

    let chr = m[x as usize][y as usize];
    if chr == '#' {
        x -= dx;
        y -= dy;
        d_idx = (d_idx + 1) % 4;
    }

    Some((x as usize, y as usize, d_idx))
}

fn walk(m: &Grid) -> Vec<Vec<usize>> {
    let (mut x, mut y) = find_start(m);
    let mut d_idx = 0;
    let mut m_walked = vec![vec![0usize; m.len()]; m.len()];
    m_walked[x][y] = 1 << d_idx;
    while let Some((x_n, y_n, d_idx_n)) = step(m, (x, y, d_idx)) {
        x = x_n;
        y = y_n;
        d_idx = d_idx_n;
        m_walked[x][y] |= 1 << d_idx;
    }
    m_walked
}

fn has_cycle(m: &Grid) -> bool {
    let (mut x, mut y) = find_start(m);
    let mut d_idx = 0;
    let mut m_walked = vec![vec![0usize; m.len()]; m.len()];
    m_walked[x][y] = 1 << d_idx;
    while let Some((x_n, y_n, d_idx_n)) = step(m, (x, y, d_idx)) {
        x = x_n;
        y = y_n;
        d_idx = d_idx_n;

        let dir = 1 << d_idx;
        if m_walked[x][y] & dir > 0 {
            return true;
        }

        m_walked[x][y] |= dir;
    }

    false
}

fn parse(path: &Path) -> Result<Grid> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect())
}

fn part1(m: &Grid) -> usize {
    let m = walk(m);
    m.iter().flat_map(|v| v.iter()).filter(|c| **c > 0).count()
}

fn part2(mut m: Grid) -> usize {
    let (x_s, y_s) = find_start(&m);
    let walked = walk(&m);
    let mut count = 0;
    for (x_i, row) in walked.iter().enumerate() {
        for (y_i, c) in row.iter().enumerate() {
            if x_i == x_s && y_i == y_s {
                continue;
            }

            if *c > 0 {
                m[x_i][y_i] = '#';
                count += has_cycle(&m) as usize;
                m[x_i][y_i] = '.';
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let m = parse(Path::new("input.txt"))?;
    let p1 = part1(&m);
    println!("Part 1 {}", p1);

    let p2 = part2(m);
    println!("Part 2 {}", p2);
    Ok(())
}
