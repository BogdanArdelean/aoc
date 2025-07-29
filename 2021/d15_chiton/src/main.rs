use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{ensure, Result};

#[derive(Debug, Clone)]
struct Grid {
    inner: Vec<Vec<i64>>
}

impl Grid {
    fn create(inner: Vec<Vec<i64>>) -> Self {
        Self {
            inner
        }
    }

    fn rows(&self) -> usize {
        self.inner.len()
    }

    fn cols(&self) -> usize {
        self.inner[0].len()
    }

    fn get_risk(&self, r: usize, c: usize) -> i64 {
        if r < self.rows() && c < self.cols() {
            self.inner[r][c]
        } else {
            i32::MAX as i64
        }
    }
}

fn get(risk: &Vec<Vec<i64>>, r: i64, c: i64) -> i64 {
    if (r as usize) < risk.len() && (c as usize) < risk[0].len() {
        risk[r as usize][c as usize]
    } else {
        i32::MAX as i64
    }
}

fn part_1_stupid(grid: &Grid) -> i64 {
    let mut total_risk = vec![vec![i64::MAX; grid.cols()]; grid.rows()];
    total_risk[0][0] = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            total_risk[r][c] = min(min(
                get(&total_risk, r as i64 - 1, c as i64),
                get(&total_risk, r as i64, c as i64 - 1)
            ) + grid.get_risk(r, c), total_risk[r][c]);
        }
    }

    total_risk[grid.rows() - 1][grid.cols() - 1]
}

#[derive(Debug)]
struct Entry {
    cost: i64,
    r: usize,
    c: usize
}

impl Entry {
    fn new(cost: i64, r: usize, c: usize) -> Self {
        Self {
            cost,
            r,
            c
        }
    }
}

impl Eq for Entry {}

impl PartialEq<Self> for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd<Self> for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn get_neighbours(grid: &Grid, entry: &Entry) -> Vec<Entry> {
    let d_pos = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    // let d_pos = vec![(0, 1), (1, 0)];

    let mut results = Vec::new();
    for (dr, dc) in d_pos {
        let r_n = entry.r as i32 + dr;
        let c_n = entry.c as i32 + dc;

        if r_n  < 0 || c_n < 0 || r_n >= grid.rows() as i32 || c_n >= grid.cols() as i32 {
            continue;
        }

        results.push(Entry::new(entry.cost + grid.get_risk(r_n as usize, c_n as usize), r_n as usize, c_n as usize));
    }

    results
}

fn part_2_shortest_path(grid: &Grid) -> i64 {
    let mut b_heap = BinaryHeap::<Entry>::new();
    let mut viz = vec![vec![false; grid.cols()]; grid.rows()];

    b_heap.push(Entry::new(0, 0, 0));

    while !b_heap.is_empty() {
        let entry = b_heap.pop().unwrap();

        if viz[entry.r][entry.c] {
            continue;
        }

        if entry.r == grid.rows() - 1 && entry.c == grid.cols() - 1 {
            return entry.cost;
        }

        viz[entry.r][entry.c] = true;

        for n in get_neighbours(grid, &entry) {
            if viz[n.r][n.c] {
                continue;
            }
            b_heap.push(n);
        }
    }

    -1
}

fn modify_fn(val: i64, add: i64) -> i64 {
    let r = (val + add) % 10;

    if r < val {
        1 + r
    } else {
        r
    }
}
fn modify_matrix(matrix: &Vec<Vec<i64>>, add: i64) -> Vec<Vec<i64>> {
    let mut matrix = matrix.clone();
    for row in matrix.iter_mut() {
        for value in row.iter_mut() {
            *value = modify_fn(*value, add);
        }
    }
    matrix
}
fn make_big_grid(grid: &Grid) -> Grid {
    let mut inner = grid.inner.clone();

    for (i, row) in inner.iter_mut().enumerate() {
        row.append(&mut grid.inner[i].clone().iter().map(|x| modify_fn(*x, 1)).collect());
        row.append(&mut grid.inner[i].clone().iter().map(|x| modify_fn(*x, 2)).collect());
        row.append(&mut grid.inner[i].clone().iter().map(|x| modify_fn(*x, 3)).collect());
        row.append(&mut grid.inner[i].clone().iter().map(|x| modify_fn(*x, 4)).collect());
    }

    let inner_aux = inner.clone();
    inner.append(&mut modify_matrix(&inner_aux, 1));
    inner.append(&mut modify_matrix(&inner_aux, 2));
    inner.append(&mut modify_matrix(&inner_aux, 3));
    inner.append(&mut modify_matrix(&inner_aux, 4));

    Grid::create(inner)
}

fn parse(path: &Path) -> Result<Grid> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut inner = Vec::new();
    for line in reader.lines() {
        let line = line?;
        inner.push(
            line.chars().map(|c| c as i64 - '0' as i64).collect()
        )
    }

    Ok(Grid::create(inner))
}

fn main() -> Result<()> {
    let grid = parse(&Path::new("input.txt"))?;
    let p1 = part_1_stupid(&grid);
    println!("Part 1 {}", p1);

    let grid = make_big_grid(&grid);
    let p2 = part_1_stupid(&grid);
    println!("Part 2 {}", p2);

    let p2 = part_2_shortest_path(&grid);
    println!("Part 2 {}", p2);
    Ok(())
}
