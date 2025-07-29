use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone)]
struct Entry {
    signal: Vec<String>,
    display: Vec<String>,
}

impl Entry {
    fn create(line: &str) -> Self {
        let mut spl = line.split(" | ");
        let signal = spl.next().unwrap().split(" ").map(|x| {
            let mut chars: Vec<char> = x.chars().collect(); // Collect characters into a Vec<char>
            chars.sort(); // Sort the characters alphabetically
            chars.into_iter().collect() // Collect them back into a String
        }).collect();

        let display = spl.next().unwrap().split(" ").map(|x| {
            let mut chars: Vec<char> = x.chars().collect(); // Collect characters into a Vec<char>
            chars.sort(); // Sort the characters alphabetically
            chars.into_iter().collect() // Collect them back into a String
        }).collect();

        Self {
            signal,
            display
        }
    }
}

fn parse(path: &Path) -> Result<Vec<Entry>> {
    let mut result = Vec::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        result.push(Entry::create(&line));
    }

    Ok(result)
}

fn part_1(entries: &Vec<Entry>) -> i32 {
    let mut count = 0;
    for entry in entries {
        count += entry.display
            .iter()
            .filter(|x| x.len() == 7 || x.len() == 4 || x.len() == 2 || x.len() == 3)
            .count() as i32;
    }
    count
}

fn all_chars_in_a2(a1: &str, a2: &str) -> bool {
    let set_a2: HashSet<char> = a2.chars().collect(); // Create a set of characters in A2
    a1.chars().all(|c| set_a2.contains(&c)) // Check if every character in A1 is in the set
}

fn find_mapping(entry: &Entry) -> HashMap<String, i32> {
    let mut mapping = HashMap::new();
    let mut imapping = HashMap::<i32, String>::new();

    // find one
    {
        let str = entry.signal.iter().filter(|x| x.len() == 2).next().unwrap().clone();
        mapping.insert(str.clone(), 1);
        imapping.insert(1, str);
    }
    // find four
    {
        let str = entry.signal.iter().filter(|x| x.len() == 4).next().unwrap().clone();
        mapping.insert(str.clone(), 4);
        imapping.insert(4, str);
    }
    // find seven
    {
        let str = entry.signal.iter().filter(|x| x.len() == 3).next().unwrap().clone();
        mapping.insert(str.clone(), 7);
        imapping.insert(7, str);
    }
    // find eight
    {
        let str = entry.signal.iter().filter(|x| x.len() == 7).next().unwrap().clone();
        mapping.insert(str.clone(), 8);
        imapping.insert(8, str);
    }

    // find nine
    {
        let four = imapping.get(&4).unwrap();
        let str = entry.signal.iter().filter(|x| x.len() == 6 && all_chars_in_a2(four, x)).next().unwrap().clone();
        mapping.insert(str.clone(), 9);
        imapping.insert(9, str);
    }
    // find six
    {
        let one = imapping.get(&1).unwrap();
        let str = entry.signal.iter().filter(|x| x.len() == 6 && !all_chars_in_a2(one, x)).next().unwrap().clone();
        mapping.insert(str.clone(), 6);
        imapping.insert(6, str);
    }
    // find zero
    {
        let nine = imapping.get(&9).unwrap();
        let six = imapping.get(&6).unwrap();
        let str = entry.signal.iter().filter(|x| x.len() == 6 && *x != six && *x != nine).next().unwrap().clone();
        mapping.insert(str.clone(), 0);
        imapping.insert(0, str);
    }
    // find five
    {
        let six = imapping.get(&6).unwrap();
        let str = entry.signal.iter().filter(|x| x.len() == 5 &&  all_chars_in_a2(x, six)).next().unwrap().clone();
        mapping.insert(str.clone(), 5);
        imapping.insert(5, str);
    }
    // find three
    {
        let one = imapping.get(&1).unwrap();
        let str = entry.signal.iter().filter(|x| x.len() == 5 && all_chars_in_a2(one, x)).next().unwrap().clone();
        mapping.insert(str.clone(), 3);
        imapping.insert(3, str);
    }
    // find two
    {
        let five = imapping.get(&5).unwrap();
        let three = imapping.get(&3).unwrap();
        let str = entry.signal.iter().filter(|x| x.len() == 5 && *x != five && *x != three).next().unwrap().clone();
        mapping.insert(str.clone(), 2);
        imapping.insert(2, str);
    }

    assert_eq!(mapping.len(), 10);

    mapping
}

fn translate_number(mapping: &HashMap<String, i32>, number: &Vec<String>) -> i32 {
    let mut result = 0;

    for digit in number {
        let digit = mapping.get(digit).unwrap();
        result *= 10;
        result += digit;
    }

    result
}

fn part_2(entries: &Vec<Entry>) -> i32 {
    let mut sum = 0;
    for entry in entries {
        let mapping = find_mapping(entry);
        let number = translate_number(&mapping, &entry.display);
        sum += number;
    }
    sum
}

fn main() -> Result<()> {
    let entries = parse(&Path::new("input.txt"))?;
    let p1 = part_1(&entries);
    println!("Part 1: {}", p1);
    let p2 = part_2(&entries);
    println!("Part 2: {}", p2);
    Ok(())
}
