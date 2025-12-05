use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse(path: &Path) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = vec![];
    let mut ids = vec![];

    let reader = BufReader::new(File::open(path).unwrap());
    let mut lines = reader.lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.len() == 0 {
            break;
        }
        let split: Vec<&str> = line.split("-").collect();
        ranges.push(
            (split[0].parse().unwrap(),
            split[1].parse().unwrap())
        );
    }

    for line in lines {
        let line = line.unwrap();
        ids.push(line.parse().unwrap())
    }

    (ranges, ids)
}

fn part1(ids: &Vec<u64>, ranges: &Vec<(u64, u64)>) -> u64 {
    let mut sum = 0;
    for &id in ids {
        for &range in ranges {
            if id >= range.0 && id <= range.1 {
                sum += 1;
                break;
            }
        }
    }
    sum
}

fn part2(mut ranges: Vec<(u64, u64)>) -> u64 {
    ranges.sort_by_key(|&(left, _)| left);
    let mut last = ranges[0];
    let mut sum = last.1 - last.0 + 1;
    for i in 1..ranges.len() {
        let curr = ranges[i];

        if curr.0 > last.1 {
            last = curr;
            sum += last.1 - last.0 + 1;
            continue;
        }

        if curr.1 <= last.1 {
            continue;
        }

        sum += curr.1 - last.1;
        last = (last.0, curr.1);
    }

    sum
}

fn main() {
    let (ranges, ids) = parse(Path::new("input.txt"));
    let p1 = part1(&ids, &ranges);
    println!("part 1: {}", p1);

    let p2 = part2(ranges);
    println!("part 2: {}", p2);
}
