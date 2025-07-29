use std::collections::{VecDeque, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{anyhow, Result};
use itertools::Itertools;

type Garden = Vec<Vec<char>>;

fn add(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn is_withing_bounds((x, y): &(i32, i32), garden: &Garden) -> bool {
    if *x < 0 || *x >= garden.len() as i32 || *y < 0 || *y >= garden[0].len() as i32 {
        false
    } else {
        true
    }
}

fn is_empty((x, y): &(i32, i32), garden: &Garden) -> bool {
    garden[*x as usize][*y as usize] == '.' || garden[*x as usize][*y as usize] == 'S'
}

fn calculate_tiles_ending(garden: &Garden, start: (i32, i32), steps: i64) -> i32 {
    let dirs = [(1,0), (-1, 0), (0, 1), (0, -1)];

    let mut tiles = 0;
    
    let mut viz = Vec::<Vec<i64>>::new();
    viz.resize_with(garden.len(), || {
        let mut cols = Vec::<i64>::new();
        cols.resize(garden[0].len(), 0);
        cols
    });


    let mut q = VecDeque::<(i32, i32)>::new();
    q.push_back(start);
    
    for i in 1..=steps {
        let mut q2 = VecDeque::<(i32, i32)>::new();
        for p in q {
            for dir in dirs {
                let new = add(&dir, &p);
                if is_withing_bounds(&new, garden) && is_empty(&new, garden) && viz[new.0 as usize][new.1 as usize] < i {
                    viz[new.0 as usize][new.1 as usize] = i;
                    q2.push_back(new);
                }
            }
        }
        q = q2;
    }
    if steps > 200 {
        pretty_print(steps, &garden, &q);
    }

    q.len() as i32
}

fn shortest_path(garden: &Garden, start: (i32, i32)) -> Vec<Vec<i64>> {
    let dirs = [(1,0), (-1, 0), (0, 1), (0, -1)];

    let mut tiles = 0;
    
    let mut viz = Vec::<Vec<i64>>::new();
    viz.resize_with(garden.len(), || {
        let mut cols = Vec::<i64>::new();
        cols.resize(garden[0].len(), 0);
        cols
    });


    let mut q = VecDeque::<(i32, i32)>::new();
    q.push_back(start);
    let mut steps = 1;
    while !q.is_empty() {
        let mut q2 = VecDeque::<(i32, i32)>::new();
        for p in q {
            for dir in dirs {
                let new = add(&dir, &p);
                if is_withing_bounds(&new, garden) && is_empty(&new, garden) && viz[new.0 as usize][new.1 as usize] == 0 {
                    viz[new.0 as usize][new.1 as usize] = steps;
                    q2.push_back(new);
                }
            }
        }

        q = q2;
        steps += 1;
    }

    viz
}

fn wrap_around((x, y): &(i32, i32), garden: &Garden) -> (i32, i32) {
    let x_new = if *x < 0 {
        let modulus = (-*x % garden.len() as i32);
        let diff =  garden.len() as i32 - modulus - (*x % garden.len() as i32 == 0) as i32;
        diff
    } else if *x >= garden.len() as i32 {
        *x % garden.len() as i32
    } else {
        *x
    };

    let y_new = if *y < 0 {
        garden[0].len() as i32 - (-*y % garden[0].len() as i32) - (*y % garden[0].len() as i32 == 0) as i32
    } else if *y >= garden[0].len() as i32 {
        *y % garden[0].len() as i32
    } else {
        *y
    };

    (x_new, y_new)
}

fn oddness(i: i64) -> i32 {
    if i % 2 == 0 {
        0b1
    } else {
        0b10
    }
}

fn count_shortest_path_wrapping(garden: &Garden, start: (i32, i32), steps: i64) -> HashMap<i64, i64> {
    let mut hm = HashMap::<_,_>::new();
    let dirs = [(1,0), (-1, 0), (0, 1), (0, -1)];
    
    let mut viz = HashMap::<(i32, i32), i32>::new();


    let mut q = VecDeque::<(i32, i32)>::new();
    q.push_back(start);
    for i in 0..=steps {
        hm.insert(i, 0);
        let mut q2 = VecDeque::<(i32, i32)>::new();

        for p in q {
            *hm.get_mut(&i).unwrap() += 1;
            for dir in dirs {
                let new = add(&dir, &p);
                let wrapped = wrap_around(&new, garden);

                if is_withing_bounds(&wrapped, garden) && is_empty(&wrapped, garden) {
                
                    if let Some(odns) = viz.get(&new) {
                        if (odns & oddness(i)) == 0 {
                            viz.insert(new, oddness(i) | odns);
                            q2.push_back(new);    
                        }
                
                    } else {
                        viz.insert(new, oddness(i));
                        q2.push_back(new);
                    }
                }
            }
        }

        q = q2;
    }
    hm
}

fn pretty_print(steps: i64, garden: &Garden, q: &VecDeque<(i32, i32)>) {
    println!("Steps: {}", steps);

    let mut garden_cloned = garden.clone();
    for p in q {
        garden_cloned[p.0 as usize][p.1 as usize] = 'X';
    }

    for row in garden_cloned {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn print(garden: &Vec<Vec<i64>>) {
    let max_width = garden.iter()
        .flat_map(|row| row.iter())
        .map(|&num| num.to_string().len())
        .max()
        .unwrap_or(0);
    for row in garden {
        for col in row {
            print!("{:width$} ", col, width = max_width);
        }
        println!();
    }
}

fn parse(path: &Path) -> Result<(Garden, (i32, i32))> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let garden = reader.lines().map(|l| l.and_then(|l| {
        Ok(l.chars().collect_vec())
    })).collect::<std::result::Result<Garden, _>>()?;

    let mut x = 0;
    let mut y = 0;
    for (i, row) in garden.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'S' {
                x = i as i32;
                y = j as i32;
                break;
            }
        }
    }

    anyhow::Ok((garden, (x, y)))
}

fn print_shortest_paths(hm: &HashMap<i64, i64>, steps: i64) {
    let mut hm_diff = HashMap::<i64, i64>::new();

    for i in 0..=steps {
        let (k, v) = hm.get_key_value(&i).unwrap();
        if *v != k*4 {
            println!("At {}: is {} -> should be {} diff: {}", k, v, k*4, k*4 - *v);
        }
        if i >= 63 && i % 2 == 1 {
            if let Some(x) = hm_diff.get_mut(&((i - 63) / 131)) {
                *x += k*4 - *v;
            } else {
                hm_diff.insert(((i - 63) / 131), k*4 - *v);
            }
        }
    }

    for i in 2..hm_diff.len() as i64 {
        println!("diff: {}", hm_diff[&i] - hm_diff[&(i-2)])
    }
}

fn naive_part2(hm: &HashMap<i64, i64>, steps: i64) -> i64 {
    let mut sum = 0;
    
    for i in (1..63).step_by(2) {
        sum += (i*4 - hm[&i]);
    }

    let mut hm_diff = HashMap::<i64, i64>::new();

    for i in 0..=steps {
        if !hm.contains_key(&i) {
            break;
        }
        let (k, v) = hm.get_key_value(&i).unwrap();
        if *v != k*4 {
            println!("At {}: is {} -> should be {} diff: {}", k, v, k*4, k*4 - *v);
        }
        if i >= 63 && i % 2 == 1 {
            if let Some(x) = hm_diff.get_mut(&((i - 63) / 131)) {
                *x += k*4 - *v;
            } else {
                hm_diff.insert(((i - 63) / 131), k*4 - *v);
            }
        }
    }

    let mut diff_odd = HashMap::<i64, i64>::new();
    for i in (0..131) {
        let bigger = 63 + (131) + i;
        let smaller = 63 + i;
        let d = (bigger*4 - hm[&bigger]) - (smaller*4 - hm[&smaller]);
        diff_odd.insert(i, d);
    }


    let mut diff_even = HashMap::<i64, i64>::new();
    for i in (0..131) {
        let bigger = 63 + (131 * 3) + i;
        let smaller = 63 + 131 + i;
        let d = (bigger*4 - hm[&bigger]) - (smaller*4 - hm[&smaller]);
        diff_even.insert(i, d);
    }

    let mut all: i128 = (steps as i128 + 1) / 2;

    let new_steps = steps as i128 - 62;
    let cycles = new_steps / 131;
    let remainder = new_steps % 131;

    let odd_diff = hm_diff[&2] as i128 - hm_diff[&0] as i128;
    let odd_k = (cycles + 1) / 2;
    let even_diff = hm_diff[&3] as i128 - hm_diff[&1] as i128;
    let even_k = cycles / 2;

    let odd_sum = odd_k * hm_diff[&0] as i128 + odd_diff * (odd_k * (odd_k - 1)) / 2;
    let even_sum = even_k * hm_diff[&1] as i128 + even_diff * (even_k * (even_k - 1)) / 2;

    all *= 4*all;
    all -= sum as i128; 
    all -= odd_sum;
    all -= even_sum;

    if cycles % 2 == 1 {
        for i in (1..remainder).step_by(2) {
            all -= (4*(63 + i) - hm[&(63 + i as i64)] as i128) as i128 + (cycles) * diff_odd[&(i as i64)] as i128;
        }
    } else {
        for i in (0..remainder).step_by(2) {
            all -= (4*(63 + i) - hm[&(63 + i as i64)] as i128) as i128 + (cycles) * diff_odd[&(i as i64)] as i128;
        }
    }

    println!("{}", all);

    let mut sum = 0;
    
    for i in (1..=steps as i64).step_by(2) {
        sum += hm[&i];
    }

    sum
}

fn sum_to_n(x: i128) -> i128 {
    x*(x+1)/2
}
fn main() -> Result<()> {
    let (garden, start) = parse(&Path::new("input.txt"))?;
    // let start_2 = (0,0);
    // for i in (1..50000).step_by(1) {
    //     let part_1 = calculate_tiles_ending(&garden, start, i);
    //     println!("Part 1: {} {}", part_1, i);
    // }
    let steps = 701;
    let hm = count_shortest_path_wrapping(&garden, start, steps);
    println!("Naive: {}", naive_part2(&hm, 26501365));
    // print_shortest_paths(&hm, steps);
    anyhow::Ok(())
}
