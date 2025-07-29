use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn chars_to_digit(s: &str) -> Vec<(usize, u32)> {
    let replacements = vec![("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
    ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

    let mut result = Vec::<(usize, u32)>::new();
    for (pat, digit) in replacements {
        for (idx, _) in s.match_indices(pat) {
            result.push((idx, digit));
        }
    }

    result
}

fn to_indices_and_digits(s: &str) -> Vec<(usize, u32)> {
    s.char_indices().flat_map(|(idx, c)| { 
        c.to_digit(10).map(|digit| (idx, digit))
    })
    .collect()
}


fn part1() -> std::io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let sum: u32 = reader.lines()
    .collect::<Result<Vec<String>,_>>()?
    .iter()
    .map(|s| to_indices_and_digits(s.as_str()))
    .map(|v| {
        let min = v.iter().min_by_key(|(idx, _)| idx).map(|(_, digit)| *digit);
        let max = v.iter().max_by_key(|(idx, _)| idx).map(|(_, digit)| *digit);
        match (min, max) {
            (Some(d1), Some(d2)) => d1 * 10 + d2,
            (_, _) => 0
        }
    })
    .sum();

    println!("Sum {}", sum);
    Ok(())
}

fn part2() -> std::io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let sum: u32 = reader.lines()
    .collect::<Result<Vec<String>,_>>()?
    .iter()
    .map(|s| (s, to_indices_and_digits(s.as_str()).into_iter()))
    .map(|(s, iter)| {
        let v2 = chars_to_digit(s.as_str());
        iter.chain(v2.into_iter())
    })
    .map(|v| {
        let min = v.clone().min_by_key(|(idx, _)| *idx).map(|(_, digit)| digit);
        let max = v.max_by_key(|(idx, _)| *idx).map(|(_, digit)| digit);
        match (min, max) {
            (Some(d1), Some(d2)) => d1 * 10 + d2,
            (_, _) => 0
        }
    })
    .sum();

    println!("Sum {}", sum);
    Ok(())
}

fn main() -> std::io::Result<()> {
    part1()?;
    part2()?;
    Ok(())
}
