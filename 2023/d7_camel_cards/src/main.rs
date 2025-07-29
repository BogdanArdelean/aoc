use std::collections::HashMap;
use std::default;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;
use anyhow::{Result, anyhow};
use scan_fmt::scan_fmt;

#[derive(Debug, Clone)]
struct Hand {
    hand: String,
    numeric_repr: u32
}

impl Hand {
    fn new(hand: String) -> Hand {
        let mut freq_map = HashMap::<char, u32>::new();

        // Maintain a frequency map for all chars instead of jokers
        let mut jokers = 0;
        for c in hand.chars() {
            if c == 'J' {
                jokers += 1;
                continue;
            }
            freq_map.insert(c, freq_map.get(&c).and_then(|x| Some(*x + 1)).unwrap_or(1));
        }
        
        // Only joker hand, add fictious entry
        if freq_map.len() == 0 {
            freq_map.insert('K', 0);
        }

        // Add jokers to the most common type
        let (_, v) = freq_map.iter_mut().max_by_key(|(_, h)| **h).unwrap();
        *v += jokers;

        // Calculate the score => x^2 because (x + y)^2 > x^2 + y^2 in case of a Full House vs 4 of a kind
        let mut numeric_repr: u32 = freq_map
        .into_iter()
        .map(|(_, nr)| ((nr - 1) * (nr - 1)).min(0xF))
        .sum();

        // Alongisde the score, append each char as a nibble
        // This lets you sort fast and without a custom comparator
        for c in hand.chars() {
            let nr = match c {
                'T' => 0xA,
                'J' => 0x1,
                'Q' => 0xC,
                'K' => 0xD,
                'A' => 0xF,
                default => default.to_digit(10).unwrap()
            };

            numeric_repr = (numeric_repr << 4) | nr;
        }

        Hand {
            hand,
            numeric_repr
        }
    }
}

fn parse1(path: &Path) -> Result<Vec<(Hand, u32)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut result = Vec::<(Hand, u32)>::new();
    
    for line in reader.lines() {
        let line = line?;
        let (h, bid) = scan_fmt!(&line, "{} {}", String, u32)?;
        result.push((Hand::new(h), bid));
    }

    anyhow::Ok(result)
}

fn main() -> Result<()> {
    let mut res = parse1(Path::new("input.txt"))?;
    res.sort_by_key(|(hand, _)| hand.numeric_repr);
    
    let part1: usize = res
    .iter()
    .enumerate()
    .map(|(idx, (_, bid))| (idx + 1) * (*bid as usize))
    .sum();
    
    println!("Part 1: {:?}", part1);
    anyhow::Ok(())
}
