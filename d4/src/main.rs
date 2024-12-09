use anyhow::Result;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse(path: &Path) -> Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .filter_map(|m| m.ok().map(|x| x.chars().collect()))
        .collect())
}

fn count_xmas(str: &str) -> usize {
    let xmas = Regex::new(r"XMAS").unwrap();
    let samx = Regex::new(r"SAMX").unwrap();
    xmas.find_iter(str).count() + samx.find_iter(str).count()
}

fn columns(puzzle: &Vec<Vec<char>>) -> Vec<String> {
    let mut res = vec![String::new(); puzzle[0].len()];

    for i in 0..puzzle.len() {
        for j in 0..puzzle[0].len() {
            res[j].push(puzzle[i][j]);
        }
    }

    res
}

fn diagonals(puzzle: &Vec<Vec<char>>) -> Vec<String> {
    assert_eq!(puzzle.len(), puzzle[0].len());
    let mut res = vec![];
    let row_len = puzzle[0].len() - 1;
    {
        let mut pdiag = String::new();
        let mut sdiag = String::new();
        for i in 0..puzzle.len() {
            pdiag.push(puzzle[i][i]);
            sdiag.push(puzzle[i][row_len - i]);
        }

        res.push(pdiag);
        res.push(sdiag);
    }

    for i in 1..puzzle.len() {
        let mut pdiag1 = String::new();
        let mut pdiag2 = String::new();
        let mut sdiag1 = String::new();
        let mut sdiag2 = String::new();

        for j in 0..puzzle.len() - i {
            pdiag1.push(puzzle[j][j + i]);
            pdiag2.push(puzzle[j + i][j]);

            sdiag1.push(puzzle[j][row_len - (j + i)]);
            sdiag2.push(puzzle[j + i][row_len - j]);
        }

        res.push(pdiag1);
        res.push(pdiag2);
        res.push(sdiag1);
        res.push(sdiag2);
    }

    res
}

fn part2(puzzle: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for i in 1..puzzle.len() - 1 {
        for j in 1..puzzle[0].len() - 1 {
            let mut diag1 = String::new();
            let mut diag2 = String::new();
            diag1.push(puzzle[i - 1][j - 1]);
            diag1.push(puzzle[i][j]);
            diag1.push(puzzle[i + 1][j + 1]);
            diag2.push(puzzle[i - 1][j + 1]);
            diag2.push(puzzle[i][j]);
            diag2.push(puzzle[i + 1][j - 1]);

            if (diag1 == "SAM" || diag1 == "MAS") && (diag2 == "SAM" || diag2 == "MAS") {
                count += 1;
            }
        }
    }
    count
}

fn part1(puzzle: &Vec<Vec<char>>) -> usize {
    let mut count: usize = puzzle
        .iter()
        .map(|x| count_xmas(&x.iter().cloned().collect::<String>()))
        .sum();
    count += columns(puzzle).iter().map(|x| count_xmas(x)).sum::<usize>();
    count += diagonals(puzzle)
        .iter()
        .map(|x| count_xmas(x))
        .sum::<usize>();
    count
}

fn main() -> Result<()> {
    let puzzle = parse(Path::new("input.txt"))?;
    let p1 = part1(&puzzle);
    println!("Part 1 {}", p1);

    let p2 = part2(&puzzle);
    println!("Part 2 {}", p2);
    Ok(())
}
