use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{Result, Error, Ok, anyhow, bail};
use scan_fmt::scan_fmt;

struct ScratchCard {
    winning_numbers: HashSet<i32>,
    chosen_numbers: HashSet<i32>,
}

impl ScratchCard {
    fn parse(line: &str) -> Result<ScratchCard> {
        let winning_string = line.split(" | ").nth(0).ok_or(anyhow!("No winning numbers"))?.split(": ").last().ok_or(anyhow!("No winning numbers"))?;
        let mut winning_numbers = HashSet::<i32>::new();
        {
            for number_string in winning_string.split(" ") {
                let number: Result<i32> = number_string.trim().parse().map_err(|x| anyhow!("Could not parse number"));
                if let Result::Ok(nr) = number {
                    winning_numbers.insert(nr);
                }
            }
        }
        let mut chosen_numbers = HashSet::<i32>::new();
        let chosen_string = line.split(" | ").last().ok_or(anyhow!("No chosen numbers"))?;
        {
            for number_string in chosen_string.split(" ") {
                let number: Result<i32> = number_string.trim().parse().map_err(|x| anyhow!("Could not parse number"));
                if let Result::Ok(nr) = number {
                    chosen_numbers.insert(nr);
                }
            }
        }
        Ok(ScratchCard {
            winning_numbers,
            chosen_numbers
        })
    }

    fn get_matching_numbers(&self) -> Vec<i32> {
        self.winning_numbers.intersection(&self.chosen_numbers).copied().collect()
    }

}

fn main() -> Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut scratchcards = Vec::<ScratchCard>::new();
    for line in reader.lines() {
        let line = line?;
        scratchcards.push(ScratchCard::parse(line.as_str())?);
    }

    let part1: u32 = scratchcards.iter()
    .map(|card| card.get_matching_numbers())
    .filter(|matching| matching.len() > 0)
    .map(|matching| (1 << (matching.len() - 1)))
    .sum();
    
    println!("Part1 {}", part1);
    let mut total_cards: Vec<i32> = scratchcards.iter().map(|_| 1).collect();
    for (idx, card) in scratchcards.iter().enumerate() {
        for new_card_idx in 1..=card.get_matching_numbers().len() {
            total_cards[idx + new_card_idx] += total_cards[idx];
        }
    }
    let part2: i32 = total_cards.iter().sum();
    println!("Part2 {}", part2);
    Ok(())
}
