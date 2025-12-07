use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn parse(path: &Path) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut reader = BufReader::new(File::open(path).unwrap());
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let splits: Vec<&str> = input.split('\n').collect();

    let mut mat = vec![];
    for i in 0..splits.len() - 1 {
        let split = splits[i];
        let mut row = vec![];
        for n_str in split.trim().split_whitespace() {
            if let Ok(n) = n_str.parse() {
                row.push(n);
            }
        }
        mat.push(row)
    }

    let mut ops = vec![];
    for op in splits[splits.len() - 1].trim().split_whitespace() {
        ops.push(op.chars().nth(0).unwrap());
    }

    (mat, ops)
}

fn part1(mat: &Vec<Vec<u64>>, ops: &Vec<char>) -> u64 {
    let mut sum = 0;
    for col in 0..mat[0].len() {
        let op = ops[col];
        let mut op_res = (op == '*') as u64;
        for row in 0..mat.len() {
            match op {
                '+' => op_res += mat[row][col],
                '*' => op_res *= mat[row][col],
                _ => panic!("can't be here!"),
            }
        }
        sum += op_res;
    }
    sum
}

fn part2(numbers_per_column: &Vec<u32>, path: &Path) -> Vec<Vec<u64>> {
    let mut r = vec![vec![]; numbers_per_column.len()];
    for (idx, nrs) in numbers_per_column.iter().enumerate() {
        r[idx].resize(*nrs as usize, 0);
    }

    let mut reader = BufReader::new(File::open(path).unwrap());
    let mut input = String::new();
    reader.read_to_string(&mut input).unwrap();
    let splits: Vec<&str> = input.split('\n').collect();

    for i in 0..splits.len() - 1 {
        let split = splits[i];
        let mut idx = 0;
        for (pr, jump) in numbers_per_column.iter().enumerate() {
            let nr = &split[idx..idx+(*jump as usize)];
            for (n, chr) in nr.chars().rev().enumerate() {
                if chr == ' ' { continue; }
                r[pr][n] *= 10;
                r[pr][n] += chr as u64 - '0' as u64;
            }
            idx += (*jump as usize + 1)
        }
    }

    r
}

fn main() {
    let (mat, ops) = parse(Path::new("input.txt"));
    let p1 = part1(&mat, &ops);
    println!("part 1: {}", p1);

    let mut numbers_per_column = vec![];
    for col in 0..mat[0].len() {
        let mut max_digits = 0;
        for row in 0..mat.len() {
            max_digits = max_digits.max(mat[row][col].ilog10() + 1);
        }
        numbers_per_column.push(max_digits);
    }

    let mat = part2(&numbers_per_column, Path::new("input.txt"));
    let mut sum = 0;
    for pr in 0..mat.len() {
        let op = ops[pr];
        let mut op_res = (op == '*') as u64;
        for nr in &mat[pr] {
            match op {
                '+' => op_res += *nr,
                '*' => op_res *= *nr,
                _ => panic!("can't be here!"),
            }
        }
        sum += op_res;
    }

    println!("part 2: {}", sum);
}
