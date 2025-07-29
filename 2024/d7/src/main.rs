use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Equation = (i64, Vec<i64>);

fn parse(path: &Path) -> Result<Vec<Equation>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut equations = vec![];
    for line in reader.lines() {
        let line = line?;
        let total: i64 = line.split(": ").nth(0).unwrap().parse()?;
        equations.push((
            total,
            line.split(": ")
                .nth(1)
                .unwrap()
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect(),
        ));
    }

    Ok(equations)
}

fn conc(mut a: i64, mut b: i64) -> i64 {
    let mut b_aux = b;
    while b_aux > 0 {
        b_aux = b_aux / 10;
        a = a * 10;
    }
    a + b
}

fn is_feasible<const PART2: bool>(total: i64, nums: &Vec<i64>, acc: i64, i: usize) -> bool
{
    if i == nums.len() {
        return total == acc;
    }

    is_feasible::<PART2>(total, nums, acc + nums[i], i + 1)
        || is_feasible::<PART2>(total, nums, acc * nums[i], i + 1)
        || (PART2 && is_feasible::<PART2>(total, nums, conc(acc, nums[i]), i + 1))
}

fn part1(eqs: &Vec<Equation>) -> i64 {
    eqs.iter()
        .filter(|(t, nums)| is_feasible::<false>(*t, nums, nums[0], 1))
        .map(|a| a.0)
        .sum()
}

fn part2(eqs: &Vec<Equation>) -> i64 {
    eqs.iter()
        .filter(|(t, nums)| is_feasible::<true>(*t, nums, nums[0], 1))
        .map(|a| a.0)
        .sum()
}

fn main() -> Result<()> {
    let eqs = parse(&Path::new("input.txt"))?;
    let p1 = part1(&eqs);
    println!("Part 1 {}", p1);

    let p2 = part2(&eqs);
    println!("Part 2 {}", p2);
    Ok(())
}
