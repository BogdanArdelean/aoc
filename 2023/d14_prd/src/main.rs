use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::path::Path;
use anyhow::{anyhow, Ok, Result};

type Platform = Vec<Vec<char>>;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Tilt {
    N,
    W,
    S,
    E
}

fn get_i_range(platform: &Platform, tilt: Tilt) -> Range<usize> {
    match tilt {
        Tilt::N => 0..platform.len(),
        Tilt::W => 0..platform[0].len(),
        Tilt::S => 0..platform.len(),
        Tilt::E => 0..platform[0].len(),
    }
}

fn get_j_range(platform: &Platform, tilt: Tilt) -> Range<usize> {
    match tilt {
        Tilt::N => 0..platform[0].len(),
        Tilt::W => 0..platform.len(),
        Tilt::S => 0..platform[0].len(),
        Tilt::E => 0..platform.len(),
    }
}

fn get_ij(platform: &Platform, tilt: Tilt, i: usize, j: usize) -> (usize, usize) {
    let platform_rows = platform.len();
    let platform_cols = platform[0].len();

    match tilt {
        Tilt::N => (i, j),
        Tilt::W => (j, i),
        Tilt::S => (platform_rows - i - 1, j),
        Tilt::E => (j, platform_cols - i - 1),
    }
}

fn get_elm(platform: &mut Platform, tilt: Tilt, i: usize, j: usize) -> &mut char {
    let platform_rows = platform.len();
    let platform_cols = platform[0].len();

    match tilt {
        Tilt::N => &mut platform[i][j],
        Tilt::W => &mut platform[j][i],
        Tilt::S => &mut platform[platform_rows - i - 1][j],
        Tilt::E => &mut platform[j][platform_cols - i - 1],
    }
}

fn tilt(platform: &mut Platform, tilt: Tilt) -> usize {
    let mut farthest_upper_free_space = Vec::<usize>::new();
    farthest_upper_free_space.resize(get_j_range(platform, tilt).end, 0);
    
    let mut load = 0;
    for i in get_i_range(platform, tilt) {
        for j in get_j_range(platform, tilt) {
            let chr = *get_elm(platform, tilt, i, j);
            match chr {
                'O' => {
                    let (ti, _) = get_ij(platform, tilt, farthest_upper_free_space[j], j);
                    load += platform.len() - ti;
                    
                    *get_elm(platform, tilt, i, j) = '.';
                    *get_elm(platform, tilt, farthest_upper_free_space[j], j) = 'O';
                    
                    farthest_upper_free_space[j] += 1;
                },
                '#' => {
                    farthest_upper_free_space[j] = i + 1;
                },
                _ => {}
            }
        }
    }

    load
}

fn part_two(mut platform: Platform) -> usize {
    let CYCLE = [Tilt::N, Tilt::W, Tilt::S, Tilt::E];
    let mut cycle_detector = HashMap::<Platform, usize>::new();
    let mut remaining_cycles = 0;
    let mut idx = 0;
    let mut load = 0;
    
    loop {
        for t in CYCLE {
            load = tilt(&mut platform, t);
        };
        idx += 1;

        if let Some(cycle_idx) = cycle_detector.get(&platform) {
            println!("{}", idx);
            let cycle_length = idx - cycle_idx;
            remaining_cycles = (1000000000 - idx) % cycle_length;
            break;
        } else {
            cycle_detector.insert(platform.clone(), idx);
        }
    }

    for _ in 0..remaining_cycles {
        for t in CYCLE {
            load = tilt(&mut platform, t);
        }
    }
    
    load
}

fn parse(path: &Path) -> Result<Platform> {
    let mut platfrom = Platform::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::<char>::new();

        for chr in line.chars() {
            row.push(chr);
        }

        platfrom.push(row);
    }
    Ok(platfrom)
}

fn main() -> Result<()> {
    let platform = parse(Path::new("input.txt"))?;
    
    let part_1 = tilt(&mut platform.clone(), Tilt::N);
    let part_2 = part_two(platform);
    
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
        
    Ok(())
}
