use std::collections::{HashSet, VecDeque};
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
        let dpos: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

        for (di, dj) in dpos {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if let Some(ge) = self.get(ni as usize, nj as usize) {
                neighbours.push(ge);
            }
        }

        neighbours
    }

    // returns the new state and the number of booms
    fn step(&self) -> (Grid, usize) {
        let mut grid = self.clone();

        let mut seen = HashSet::<(usize, usize)>::new();
        let mut q = VecDeque::new();

        // increase all
        for (row_nr, row) in &mut grid.g.iter_mut().enumerate() {
            for (col_nr, col) in row.iter_mut().enumerate() {
                *col = *col + 1;
                if *col > 9 {
                    q.push_back((row_nr, col_nr));
                    seen.insert((row_nr, col_nr));
                }
            }
        }

        while !q.is_empty() {
            let (i, j) = q.pop_back().unwrap();
            grid.g[i][j] = 0;
            for n in grid.neighbours(i, j) {
                let i_n = n.i;
                let j_n = n.j;

                if seen.contains(&(i_n, j_n)) {
                    continue;
                }

                grid.g[i_n][j_n] += 1;

                if grid.g[i_n][j_n] > 9 {
                    q.push_back((i_n, j_n));
                    seen.insert((i_n, j_n));
                }
            }
        }

        (grid, seen.len())
    }
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


fn part_1(grid: &Grid) -> usize {
    let mut g = grid.clone();
    let mut booms = 0;
    for i in 0..100 {
        let (g_next, b_next) = g.step();
        g = g_next;
        booms += b_next;
    }

    booms
}

fn part_2(grid: &Grid) -> usize {
    let mut g = grid.clone();
    let mut steps = 0;

    loop {
        let (g_next, b_next) = g.step();
        steps += 1;
        if b_next == g.rows * g.cols {
            break;
        }
        g = g_next;
    }

    steps
}


fn main() -> Result<()> {
    let grid = parse(&Path::new("input.txt"))?;
    let p1 = part_1(&grid);
    println!("Part 1 {}", p1);
    let p2 = part_2(&grid);
    println!("Part 2 {}", p2);
    Ok(())
}
