use anyhow::Result;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Rev;
use std::path::Path;

type Memory = Vec<i64>;
fn parse(path: &Path) -> Result<(Memory, Vec<i64>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut memory = vec![];
    let mut v = vec![];
    let mut is_file_block = true;
    let mut file_idx = -1;
    let line = reader.lines().next().unwrap()?;
    for chr in line.chars() {
        let size = (chr as i64) - '0' as i64;

        v.push(size);
        let (content, next_file_idx) = match is_file_block {
            true => (file_idx, file_idx - 1),
            false => (0, file_idx),
        };
        file_idx = next_file_idx;

        for _ in 0..size {
            memory.push(content);
        }

        is_file_block = !is_file_block;
    }

    Ok((memory, v))
}

fn get_next_free_block(m: &Memory, mut curr: usize) -> usize {
    while curr < m.len() && m[curr] != 0 {
        curr += 1;
    }

    curr
}

fn get_next_occupied_block(m: &Memory, mut curr: usize) -> usize {
    while curr > 0 && m[curr] == 0 {
        curr -= 1;
    }

    curr
}

fn compact_memory(m: &Memory) -> Memory {
    let mut c_m = m.clone();
    let mut nx_free_idx = get_next_free_block(&c_m, 0);
    let mut nx_occupied_idx = get_next_occupied_block(&c_m, c_m.len() - 1);

    while nx_free_idx < nx_occupied_idx {
        c_m[nx_free_idx] = c_m[nx_occupied_idx];
        c_m[nx_occupied_idx] = 0;
        nx_free_idx = get_next_free_block(&c_m, nx_free_idx);
        nx_occupied_idx = get_next_occupied_block(&c_m, nx_occupied_idx);
    }

    c_m
}

fn find_free_space_before(m: &Memory, sz: usize, before: usize) -> Option<usize> {
    let mut last_non_zero = -1;
    let before = before.min(m.len());
    for idx in 0..before {
        let block_id = m[idx];
        if block_id == 0 {
            let s = idx as i64 - last_non_zero;
            if s as usize >= sz {
                return Some((last_non_zero + 1) as usize);
            }
        } else {
            last_non_zero = idx as i64;
        }
    }

    return None;
}

fn get_file(m: &Memory, idx: usize) -> Option<(usize, usize)> {
    let mut idx = idx as i64 - 1;

    if idx < 0 {
        return None;
    }

    while idx >= 0 && m[idx as usize] == 0 {
        idx -= 1;
    }

    if idx < 0 {
        return None;
    }

    let last_idx = idx;
    let f_id = m[idx as usize];

    while idx >= 0 && m[idx as usize] == f_id {
        idx -= 1;
    }

    Some(((idx + 1) as usize, (last_idx - idx) as usize))
}

fn mem_move(m: &mut Memory, from: usize, to: usize, sz: usize) {
    for idx in 0..sz {
        m[to + idx] = m[from + idx];
        m[from + idx] = 0;
    }
}

fn compact_memory_files(m: &Memory) -> Memory {
    let mut c_m = m.clone();

    let mut idx = c_m.len();
    let mut max_file = i64::MIN;
    while let Some((file_start_idx, sz)) = get_file(&c_m, idx) {
        let block_id = c_m[file_start_idx];
        if block_id < max_file {
            idx = file_start_idx;
            continue;
        }

        max_file = max_file.max(block_id);
        if let Some(free_space_idx) = find_free_space_before(&c_m, sz, file_start_idx) {
            mem_move(&mut c_m, file_start_idx, free_space_idx, sz);
        }

        idx = file_start_idx;
    }
    c_m
}

fn sum_range(from: usize, size: usize) -> usize {
    let to = from + size - 1;
    (from..=to).sum()
}

fn part2_fast(m: &[i64]) -> usize {
    let mut blanks: Vec<BinaryHeap<Reverse<usize>>> = vec![BinaryHeap::new(); 10];
    let mut files = HashMap::<i64, (usize, usize)>::new();
    {
        let mut idx = 0;
        for i in 0..m.len() {
            let sz = m[i] as usize;
            if i % 2 == 0 {
                files.insert(i as i64 / 2, (idx, sz));
            } else {
                blanks[sz].push(Reverse(idx));
            }

            idx += sz;
        }
    }

    let mut checksum = 0;
    let mut file = (m.len() as i64 - 1) / 2;
    while let Some((file_idx, file_size)) = files.get(&file) {
        let mut selected_blank = None;
        for blank_size in *file_size..10 {
            if let Some(&Reverse(blank_idx)) = blanks[blank_size].peek() {
                if blank_idx < *file_idx {
                    selected_blank = match selected_blank {
                        Some((a, b)) if b <= blank_idx => Some((a, b)),
                        _ => Some((blank_size, blank_idx)),
                    };
                }
            }
        }

        let new_file_idx = if let Some((blank_size, blank_idx)) = selected_blank {
            blanks[blank_size].pop();
            let remaining_size = blank_size - file_size;
            let remaining_idx = blank_idx + file_size;
            if remaining_size > 0 {
                blanks[remaining_size].push(Reverse(remaining_idx));
            }
            blank_idx
        } else {
            *file_idx
        };

        checksum += (sum_range(new_file_idx, *file_size) * file as usize);
        file -= 1;
    }

    checksum
}

fn part1(m: &Memory) -> i64 {
    compact_memory(m)
        .iter()
        .enumerate()
        .map(|(idx, bl_id)| idx as i64 * ((*bl_id != 0) as i64) * (-bl_id - 1))
        .sum()
}

fn part2(m: &Memory) -> i64 {
    compact_memory_files(m)
        .iter()
        .enumerate()
        .map(|(idx, bl_id)| idx as i64 * ((*bl_id != 0) as i64) * (-bl_id - 1))
        .sum()
}

fn main() -> Result<()> {
    let (memory, v) = parse(&Path::new("input.txt"))?;
    let p1 = part1(&memory);
    println!("Part 1 {}", p1);

    println!("Part 2 fast {}", part2_fast(&v));

    let p2 = part2(&memory);
    println!("Part 2 {}", p2);

    Ok(())
}
