use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse(path: &Path) -> Vec<i32> {
    let mut r = vec![];
    let mut reader = BufReader::new(File::open(path).unwrap());
    for line in reader.lines() {
        let mut line = line.unwrap();
        let sign = if line.contains('R') {
            1
        } else {
            -1
        };
        r.push(line.as_str()[1..].parse::<i32>().unwrap() * sign);
    }
    r
}

fn count_zero(mut state: i32, lock_size: i32, turns: &Vec<i32>) -> u32 {
    let mut count = 0;
    for &turn in turns {
        state += turn;
        state = state % lock_size;

        state = if state < 0 {
            state + lock_size
        } else {
            state
        };

        count += (state == 0) as u32;
    }
    count
}

fn count_over_zero(mut state: i32, lock_size: i32, turns: &Vec<i32>) -> i32 {
    let mut count = 0i32;
    for &turn in turns {
        let last_state = state;
        count += (turn / lock_size).abs();
        let rem = turn % lock_size;

        state += rem;
        if state <= 0 {
            count += (last_state != 0) as i32;
            state = (state + lock_size) % lock_size;
        } else {
            count += (state / lock_size).abs();
            state %= lock_size;
        }
    }

    count
}

fn main() {
    let turns = parse(Path::new("input.txt"));
    println!("part 1: {}", count_zero(50, 100, &turns));
    println!("part 2: {}", count_over_zero(50, 100, &turns));
}
