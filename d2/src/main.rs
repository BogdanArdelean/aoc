use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse(path: &Path) -> Result<Vec<Vec<i32>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .filter_map(|x| {
            x.ok().map(|s| {
                s.split(" ")
                    .filter_map(|nr| nr.parse().ok())
                    .collect::<Vec<i32>>()
            })
        })
        .collect())
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let diffs: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();

    let all_increasing = diffs.iter().all(|&d| d > 0);
    let all_decreasing = diffs.iter().all(|&d| d < 0);

    let diffs_valid = diffs.iter().all(|&d| d.abs() >= 1 && d.abs() <= 3);

    (all_increasing || all_decreasing) && diffs_valid
}

fn part1(reports: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for report in reports {
        if is_safe(report) {
            count += 1;
        }
    }

    count
}

fn is_within_limits(a: i32, b: i32, increasing: bool) -> bool {
    let s = a - b;
    if s.abs() > 3 {
        return false;
    }

    if increasing && s >= 0 || !increasing && s <= 0 {
        return false;
    }

    true
}

fn is_safe_with_error_correction(report: &[i32]) -> bool {
    let mut slow = 0;
    let mut fast = 1;
    let mut corrected = usize::MAX;
    let increasing = report[0] - report[1] < 0;

    while fast < report.len() {
        if slow == corrected {
            slow += 1;
        }

        let a = report[slow];
        let b = report[fast];

        if !is_within_limits(a, b, increasing) {
            if corrected != usize::MAX {
                return false;
            }
            corrected = fast;
            fast += 1;
        } else {
            slow += 1;
            fast += 1;
        }
    }

    true
}

fn part2(reports: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for report in reports {
        let rev_report: Vec<i32> = report.clone().into_iter().rev().collect();
        if is_safe_with_error_correction(report) || is_safe_with_error_correction(&rev_report) {
            count += 1;
        }
    }

    count
}

fn main() -> Result<()> {
    let reports = parse(Path::new("input.txt"))?;

    let p1 = part1(&reports);
    println!("Part 1 {}", p1);

    let p2 = part2(&reports);
    println!("Part 2 {}", p2);
    Ok(())
}
