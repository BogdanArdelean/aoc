use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse(path: &Path) -> Vec<u64> {
    let reader = BufReader::new(File::open(path).unwrap());
    reader.lines().next().unwrap().unwrap()
        .split(",")
        .flat_map(|x| x.split("-"))
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_invalid(a: u64, b: u64) -> u64 {
    let mut r = 0;
    for i in a..=b {
        let digits = i.ilog10() as u64 + 1;

        if digits % 2 != 0 {
            continue;
        }

        let p = 10u64.pow((digits / 2) as u32);
        let first_half = i / p;
        let second_half = i % p;

        if first_half == second_half {
            r += i;
        }
    }
    r
}

fn get_invalid_twice(a: u64, b: u64) -> u64 {
    let mut r = 0;
    for i in a..=b {
        let digits = i.ilog10() as u64 + 1;
        let i_str: Vec<u8> = i.to_string().bytes().collect();
        for chunk in 1..=(digits / 2) {
            let ok = i_str
                .chunks(chunk as usize)
                .all(|f| f.eq(&i_str[0..chunk as usize]));

            if ok {
                r += i;
                break;
            }
        }
    }
    r
}

fn main() {
    let v = parse(Path::new("input.txt"));
    let p1: u64 = v
        .chunks(2)
        .map(|c| get_invalid(c[0], c[1]))
        .sum();

    let p2: u64 = v
        .chunks(2)
        .map(|c| get_invalid_twice(c[0], c[1]))
        .sum();

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);
}
