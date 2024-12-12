use crate::Side::{Horizontal, Vertical};
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Grid = Vec<Vec<char>>;
const DD: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn parse(path: &Path) -> Result<Grid> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut grid = Grid::new();

    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect());
    }

    Ok(grid)
}

fn is_within_bounds(r: usize, c: usize, g: &Grid) -> bool {
    !(r >= g.len() || c >= g[0].len())
}

fn compute_perimeter(
    (r, c): (usize, usize),
    plant: char,
    g: &Grid,
    viz: &mut HashSet<(usize, usize)>,
) -> usize {
    if !is_within_bounds(r, c, g) || viz.contains(&(r, c)) {
        return 0;
    }
    viz.insert((r, c));

    let mut perimeter = 0;
    for (dr, dc) in DD {
        let rr = (r as i32 + dr) as usize;
        let cc = (c as i32 + dc) as usize;
        if !is_within_bounds(rr, cc, g) || g[rr][cc] != plant {
            perimeter += 1;
            continue;
        }

        perimeter += compute_perimeter((rr, cc), plant, g, viz);
    }

    perimeter
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Side {
    Horizontal(i64),
    Vertical(i64),
}

fn insert_side(
    (r, c): (usize, usize),
    d_idx: usize,
    sides: &mut HashMap<Side, Vec<i64>>
) {
    let r = r as i64 + 1;
    let c = c as i64 + 1;
    match d_idx {
        0 => sides.entry(Horizontal(r)).or_default().push(c),
        3 => sides.entry(Horizontal(-r)).or_default().push(c),
        1 => sides.entry(Vertical(c)).or_default().push(r),
        2 => sides.entry(Vertical(-c)).or_default().push(r),
        _ => panic!("Direction index invalid"),
    }
}

fn compute_sides(
    (r, c): (usize, usize),
    plant: char,
    g: &Grid,
    viz: &mut HashSet<(usize, usize)>,
    sides: &mut HashMap<Side, Vec<i64>>,
) {
    if !is_within_bounds(r, c, g) || viz.contains(&(r, c)) {
        return;
    }
    viz.insert((r, c));

    for (d_idx, (dr, dc)) in DD.iter().enumerate() {
        let rr = (r as i32 + dr) as usize;
        let cc = (c as i32 + dc) as usize;
        if !is_within_bounds(rr, cc, g) || g[rr][cc] != plant {
            insert_side((rr, cc), d_idx, sides);
            continue;
        }

        compute_sides((rr, cc), plant, g, viz, sides);
    }
}

fn count_continuous_sides(sides: HashMap<Side, Vec<i64>>) -> usize {
    sides
        .into_iter()
        .map(|(_, mut v)| {
            v.sort();
            v.windows(2).map(|w| w[1] - w[0]).filter(|c| *c > 1).count() + 1
        })
        .sum::<usize>()
}

fn part1(g: &Grid) -> usize {
    let mut sum = 0;
    let mut viz = HashSet::new();
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            let area_before = viz.len();
            let perimeter = compute_perimeter((r, c), g[r][c], g, &mut viz);
            let area = viz.len() - area_before;
            sum += (area * perimeter);
        }
    }
    sum
}

fn part2(g: &Grid) -> usize {
    let mut sum = 0;
    let mut viz = HashSet::new();

    for r in 0..g.len() {
        for c in 0..g[0].len() {
            let area_before = viz.len();

            let mut sides = HashMap::new();
            compute_sides((r, c), g[r][c], g, &mut viz, &mut sides);
            let continuous_sides = count_continuous_sides(sides);

            let area = viz.len() - area_before;
            sum += (area * continuous_sides);
        }
    }
    sum
}

fn main() -> Result<()> {
    let grid = parse(Path::new("input.txt"))?;
    let p1 = part1(&grid);
    println!("Part 1 {}", p1);

    let p2 = part2(&grid);
    println!("Part 2 {}", p2);
    Ok(())
}
