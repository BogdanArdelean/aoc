use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct GridElement {
    value: i32,
    i: usize,
    j: usize,
}

#[derive(Debug, Clone)]
struct Grid {
    g: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(g: Vec<Vec<i32>>) -> Self {
        let rows = g.len();
        let cols = g.get(0).unwrap().len();

        Self {
            g,
            rows,
            cols
        }
    }

    fn in_bounds(&self, i: usize, j: usize) -> bool {
        i < self.rows &&
        j < self.cols
    }

    fn get(&self, i: usize, j: usize) -> Option<GridElement> {
        if !self.in_bounds(i, j) {
            return None;
        }

        Some(GridElement {
            value: self.g[i][j],
            i,
            j
        })
    }

    fn neighbours(&self, i: usize, j: usize) -> Vec<GridElement> {
        let mut neighbours = Vec::new();
        let dpos: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

        for (di, dj) in dpos {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if let Some(ge) = self.get(ni as usize, nj as usize) {
                neighbours.push(ge);
            }
        }

        neighbours
    }
}

fn calculate_risk(basins_low: &Vec<GridElement>) -> i32 {
    let mut risk = 0;
    for ge in basins_low {
        risk += (1 + ge.value);
    }
    risk
}

fn get_basins_low(grid: &Grid) -> Vec<GridElement> {
    let mut basins_low = Vec::new();
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            let ge = grid.get(i, j).unwrap();

            if grid.neighbours(i, j).iter().all(|n| n.value > ge.value) {
                basins_low.push(ge);
            }
        }
    }

    basins_low
}

fn parse(path: &Path) -> Result<Grid> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::new();

        for chr in line.chars() {
            row.push((chr as i32) - ('0' as i32));
        }

        grid.push(row);
    }

    Ok(Grid::new(grid))
}

fn dfs(grid: &Grid, ge: &GridElement, seen: &mut HashSet<GridElement>) -> i32 {
    seen.insert(ge.clone());

    let mut count = 1;
    for n in grid.neighbours(ge.i, ge.j) {
        if n.value == 9 {
            continue;
        }

        if seen.contains(&n) {
            continue;
        }

        count += dfs(grid, &n, seen);
    }

    count
}

fn get_basins_sizes(grid: &Grid, basins_low: &Vec<GridElement>) -> Vec<i32> {
    let mut seen_set = HashSet::new();
    let mut res = Vec::new();
    for bl in basins_low {
       res.push(dfs(grid, bl, &mut seen_set));
    }

    res.sort();
    res
}

fn main() -> Result<()> {
    let grid = parse(&Path::new("input.txt"))?;
    let basins_low = get_basins_low(&grid);
    println!("Part 1 {}", calculate_risk(&basins_low));

    let basins_sizes = get_basins_sizes(&grid, &basins_low);
    let p2 = basins_sizes.iter().rev().take(3).fold(1, |acc, e| acc * e);
    println!("Part 2 {}", p2);

    Ok(())
}
