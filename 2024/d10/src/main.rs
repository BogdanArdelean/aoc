use anyhow::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type HeightMap = Vec<Vec<i32>>;
fn parse(path: &Path) -> Result<HeightMap> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c as i32 - '0' as i32)
                .collect()
        })
        .collect())
}

fn find_trail<F>(map: &HeightMap, i: usize, j: usize, score_func: &mut F) -> usize
where
    F: FnMut(usize, usize) -> usize,
{
    if map[i][j] == 9 {
        return score_func(i, j);
    }
    const DD: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    let current_height = map[i][j];
    let mut count = 0;

    for (dx, dy) in DD {
        let ii = (i as i32 + dx) as usize;
        let jj = (j as i32 + dy) as usize;

        if ii >= map.len() || jj >= map[0].len() {
            continue;
        }

        let next_height = map[ii][jj];
        if next_height - current_height == 1 {
            count += find_trail(map, ii, jj, score_func);
        }
    }

    count
}

fn solve<Gen, F>(m: &HeightMap, score_func_gen: Gen) -> usize
where
    F: FnMut(usize, usize) -> usize,
    Gen: Fn() -> F,
{
    let mut count = 0;

    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] == 0 {
                count += find_trail(m, i, j, &mut score_func_gen());
            }
        }
    }

    count
}

fn part1(m: &HeightMap) -> usize {
    let gen = || {
        let mut hset = HashSet::new();
        move |a, b| {
            if hset.contains(&(a, b)) {
                0
            } else {
                hset.insert((a, b));
                1
            }
        }
    };

    solve(m, gen)
}

fn part2(m: &HeightMap) -> usize {
    let gen = || |_, _| 1;
    solve(m, gen)
}

fn main() -> Result<()> {
    let map = parse(Path::new("input.txt"))?;
    let p1 = part1(&map);
    println!("Part 1 {}", p1);

    let p2 = part2(&map);
    println!("Part 2 {}", p2);

    Ok(())
}
