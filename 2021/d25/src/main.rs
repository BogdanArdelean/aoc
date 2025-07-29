use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;
use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

fn print_grid(g: &Grid) {
    for row in g {
        for chr in row {
            print!("{}", chr);
        }
        println!();
    }
    println!();
}

fn move_cucumber(i: usize, j: usize, g: &Grid) -> Option<(usize, usize)> {
    if !(i < g.len() && j < g[0].len()) { return None; }

    let chr = g[i][j];
    match chr {
        '.' => None,
        '>' => {
            let ii = i;
            let jj = (j+1) % g[0].len();
            if g[ii][jj] == '.' {
                Some((ii, jj))
            } else {
                None
            }
        }
        'v' => {
            let ii = (i + 1) % g.len();
            let jj = j;
            
            if g[ii][jj] == '.' {
                Some((ii, jj))
            } else {
                None
            }
        }
        _   => panic!("Unrecognized seabed tile!!") 
    }
}

fn find_landing_time(mut g: Grid) -> usize {
    let mut steps = 0;
    loop {
        let mut moved = false;
        let mut g_aux = vec![vec!['.'; g[0].len()]; g.len()];
        for i in 0..g.len() {
            for j in 0..g[0].len() {
                if g[i][j] == '.' {continue;}

                if g[i][j] == '>' {
                    if let Some((ii, jj)) = move_cucumber(i, j, &g) {
                        g_aux[ii][jj] = g[i][j];
                        moved |= true;
                    } else {
                        g_aux[i][j] = g[i][j];
                    }
                } else{
                    g_aux[i][j] = g[i][j];
                }
            }
        }

        g = g_aux;
        let mut g_aux = vec![vec!['.'; g[0].len()]; g.len()];
        for i in 0..g.len() {
            for j in 0..g[0].len() {
                if g[i][j] == '.' {continue;}

                if g[i][j] == 'v' {
                    if let Some((ii, jj)) = move_cucumber(i, j, &g) {
                        g_aux[ii][jj] = g[i][j];
                        moved |= true;
                    } else {
                        g_aux[i][j] = g[i][j];
                    }
                } else {
                    g_aux[i][j] = g[i][j];
                }
            }
        }

        g = g_aux;
        steps += 1;

        if !moved { break; }
    }

    steps
}

fn parse(path: &Path) -> Result<Grid> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut g = Grid::new();
    for line in reader.lines() {
        let line = line?;
        g.push(
            line
                .chars()
                .collect()
        );
    }

    Ok(g)
}

fn main() -> Result<()> {
    let grid = parse(&Path::new("input.txt"))?;
    let p1 = find_landing_time(grid);
    println!("Part 1 {}", p1);
    Ok(())
}
