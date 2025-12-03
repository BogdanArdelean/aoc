use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn parse(path: &Path) -> Vec<Vec<u64>> {
    let reader = BufReader::new(File::open(path).unwrap());
    reader
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c as u64 - '0' as u64).collect())
        .collect()
}

fn maximum_joltage(banks: &Vec<Vec<u64>>, max_batteries: usize) -> u64 {
    let mut joltage = 0;

    for bank in banks {
        let mut max_left = vec![0; max_batteries];
        let mut bank_joltage = 0;

        for (idx, &joltage) in bank.iter().enumerate() {
            for idx_bat in (0..max_batteries).rev() {
                if idx < idx_bat {
                    max_left[idx_bat] = max_left[idx_bat] * 10 + joltage;
                } else {
                    let prev = if idx_bat > 0 { max_left[idx_bat - 1] } else { 0 };
                    max_left[idx_bat] = max_left[idx_bat].max(prev * 10 + joltage);
                }
            }
            bank_joltage = bank_joltage.max(max_left[max_batteries - 1]);
        }

        joltage += bank_joltage
    }

    joltage
}

fn main() {
    let input_file = "input.txt";
    let banks = parse(Path::new(&input_file));
    let p1 = maximum_joltage(&banks, 2);
    println!("part 1: {}", p1);

    let p2 = maximum_joltage(&banks, 12);
    println!("part 2: {}", p2);
}
