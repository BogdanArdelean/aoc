use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use anyhow::{anyhow, Ok, Result};
use itertools::Itertools;

fn find_palindrome_left(vals: &[u128]) -> i64 {
    let mut q = Vec::<(usize, u128)>::new();
        let mut res = 0;
        for (idx, val) in vals.iter().copied().enumerate() {
            if let Some((_, qval)) = q.last().copied() {
                if qval == val {
                    q.pop();
                } else {
                    q.push((idx + 1, val));
                }
            } else {
                q.push((idx + 1, val));
            }

            if q.is_empty() {
                res = idx + 1;
                break;
            }
        }
        res as i64 / 2
}

fn find_palindrome_right(vals: &[u128]) -> i64 {
    let mut q = Vec::<(usize, u128)>::new();
        let mut res = 0;
        for (idx, val) in vals.iter().rev().copied().enumerate() {
            if let Some((_, qval)) = q.last().copied() {
                if qval == val {
                    q.pop();
                } else {
                    q.push((idx + 1, val));
                }
            } else {
                q.push((idx + 1, val));
            }

            if q.is_empty() {
                res = idx + 1;
                break;
            }
        }
        if res != 0 {
            vals.len() as i64 - res as i64 / 2
        } else {
            0
        }
}

fn find_palindrome_size(vals: &[u128]) -> i64 {
    find_palindrome_left(vals).max(find_palindrome_right(vals))
}

fn parse(path: &Path) -> Result<Vec<(Vec<u128>, Vec<u128>)>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut str = String::new();
    reader.read_to_string(&mut str)?;

    let mut result = Vec::<_>::new();
    let last = str.split("\n\n").count();
    for (sz, map) in str.split("\n\n").enumerate() {
        if sz == last - 1 {
            continue;;
        }

        let mut rows = Vec::<u128>::new();
        let mut cols = Vec::<u128>::new();

        let map_rows = map.split("\n");
        let row_len = map_rows.clone().count();
        let col_len = map_rows.clone().last().unwrap().chars().count();

        rows.resize(row_len, 0);
        cols.resize(col_len, 0);

        for (i, row) in map_rows.enumerate() {
            for (j, col) in row.chars().enumerate() {
                let val = if col == '.' {
                    0
                } else {
                    1
                };

                rows[i] = (rows[i] << 1) | val;
                cols[j] = (cols[j] << 1) | val;
            }
        }

        result.push((rows, cols));
    }   

    Ok(result)
}

fn debug(vals: &[u128]) -> bool {
    let sums: i32 = vals.iter().tuple_windows::<(_, _)>().map(|(x, y)| {
        if *x == *y {
            return 1;
        } else {
            return 0;
        }
    }).sum();

    sums > 1
}

fn debug_sumedge(vals: &[u128]) -> i32 {
    let mut count = 0;
    for v    in vals.iter().combinations(2) {
        let a = *v[0];
        let b = *v[1];
        let axb = (a ^ b) as i128;
        if axb != 0 && (axb & (axb - 1)) == 0 {
            println!("{}, {}", a, b);
            count += 1;
        }
    }

    count
}

fn part_2(last_result: i64, rows: &mut [u128], cols: &mut [u128]) -> i64 {
    for i in 0..rows.len()-1 {
        for j in i+1..rows.len() {
            let a = rows[i];
            let b = rows[j];
            let axb = (a ^ b) as i128;
            if axb != 0 && (axb & (axb - 1)) == 0 {
                println!("{}, {}, {}", axb, a, b);
                rows[j] = a;
                let (x, y) = (find_palindrome_left(rows) * 100, find_palindrome_right(rows) * 100);
                println!("X: {} Y: {}, {}, {}", x, y, i, j);
                if x != last_result && x != 0 {
                    return x;
                }
                if y != last_result && y != 0 {
                    return y;
                }

                rows[j] = b;
                rows[i] = b;

                let (x, y) = (find_palindrome_left(rows) * 100, find_palindrome_right(rows) * 100);
                println!("X: {} Y: {}, {}, {}", x, y, i, j);
                if x != last_result && x != 0 {
                    return x;
                }
                if y != last_result && y != 0 {
                    return y;
                }

                rows[i] = a;
            }
        }
    }

    for i in 0..cols.len()-1 {
        for j in i+1..cols.len() {
            let a = cols[i];
            let b = cols[j];
            let axb = (a ^ b) as i128;
            if axb != 0 && (axb & (axb - 1)) == 0 {
                println!("{}, {}, {}", axb, a, b);
                cols[j] = a;
                let (x, y) = (find_palindrome_left(cols), find_palindrome_right(cols));
                println!("X: {} Y: {}, {}, {}", x, y, i, j);
                if x != last_result && x != 0 {
                    return x;
                }
                if y != last_result && y != 0 {
                    return y;
                }
                cols[j] = b;
                cols[i] = b;
                let (x, y) = (find_palindrome_left(cols), find_palindrome_right(cols));
                println!("X: {} Y: {}, {}, {}", x, y, i, j);
                if x != last_result && x != 0 {
                    return x;
                }
                if y != last_result && y != 0 {
                    return y;
                }
                cols[i] = a;
            }
        }
    }
    println!("{}, {:?}, {:?}", last_result, rows, cols);
    panic!("What?");
    0
}

fn main() -> Result<()> {
    let mut v = parse(Path::new("input.txt"))?;
    let mut sum = 0;
    let mut sum_pt_2 = 0;
    println!("Pal: {}", find_palindrome_size(&[1, 2, 3, 3, 2, 5, 4, 4, 5, 2, 3, 3, 2, 1, 0, 100]));
    println!("Pal: {}", find_palindrome_size(&[1,2,3,3,4,5,5,4,2,1]));
    for (idx, (rows, cols)) in v.iter_mut().enumerate() {
        println!("Processing {}", idx);
        let (row_res, col_res) = (find_palindrome_size(&rows), find_palindrome_size(&cols));
        // println!("Smudge {}: {}, {}",idx, debug_sumedge(rows), debug_sumedge(cols));
        // if debug(rows) {
        //     println!("{}, Rows: {:?}", idx, rows);
        // }
        // if debug(cols) {
        //     println!("{}, Cols: {:?}", idx, cols);
        // }
        // if row_res == col_res {
        //     println!("ZERO: Idx: {}", idx);
        //     println!("Rows: {:?}", rows); 33782
        //     println!("Cols: {:?}", cols);
        //     println!("{}, {}", row_res, col_res);
        // }
        // if row_res != 0 && col_res != 0 {
        //     println!("Two Non Zero Idx: {}", idx);
        //     println!("Rows: {:?}", rows);
        //     println!("Cols: {:?}", cols);
        //     println!("{}, {}", row_res, col_res);
        // }
        let part_2_res = part_2((row_res * 100) + col_res, rows, cols);
        sum_pt_2 += part_2_res;
        sum += (row_res * 100) + col_res;
        println!();
    }
    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum_pt_2);
    Ok(())
}
