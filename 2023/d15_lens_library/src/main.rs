use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use anyhow::{anyhow, Ok, Result};
use linked_hash_map::LinkedHashMap;

fn parse(path: &Path) -> Result<Vec<String>> {
    let mut result = Vec::<_>::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let input = reader.lines().fold("".to_string(), |acc, x| {
        acc + &x.unwrap()
    });

    for instruction in input.split(",") {
        result.push(instruction.to_string())
    }

    Ok(result)
}

fn hash(s: &str) -> i32 {
    let mut result = 0;
    for chr in s.chars() {
        result += chr as i32;
        result *= 17;
        result %= 256;
    }
    result
}

fn find_focal_strength(instructions: &Vec<String>) -> i64 {
    let mut hm = HashMap::<i32, LinkedHashMap<String, i32>>::new();
    for instr in instructions {
        let mut split_by_eq = instr.split("=");
        let mut split_by_dash = instr.split("-");

        if split_by_eq.clone().count() > 1 {
            let label = split_by_eq.nth(0).unwrap();
            let hash_label = hash(label);
            let focal_length: i32 = split_by_eq.last().unwrap().parse().unwrap();

            if let Some(im) = hm.get_mut(&hash_label) {
                if let Some(update) = im.get_mut(label) {
                    *update = focal_length;
                } else {
                    im.insert(label.to_string(), focal_length);
                }
            } else {
                hm.insert(hash_label, LinkedHashMap::<_,_>::default());
                hm.get_mut(&hash_label).unwrap().insert(label.to_string(), focal_length);
            }
        } else {
            let label = split_by_dash.nth(0).unwrap();
            let hash_label = hash(label);

            if let Some(im) = hm.get_mut(&hash_label) {
                im.remove(label);
            }
        }
    }

    let mut sum = 0;
    
    for (bx_nr, bx) in &hm {
        for (entry_nr, (_, focal_length)) in bx.iter().enumerate() {
            sum += (*bx_nr as i64 + 1) * (entry_nr as i64 + 1) * *focal_length as i64
        }
    }

    sum
}

fn main() -> Result<()> {
    let instructions = parse(Path::new("input.txt"))?;
    
    let part_1: i32 = instructions.iter().map(|s| hash(s)).sum();
    let part_2 = find_focal_strength(&instructions);
    
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    Ok(())
}
