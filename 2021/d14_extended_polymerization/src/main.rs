use anyhow::Result;
use std::collections::{BTreeMap, HashMap, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn simulate_clever(template: &LinkedList<char>, mapping: &HashMap<(char, char), char>, steps: usize) -> HashMap<char, u128> {
    let mut count = HashMap::new();
    for ch in template {
        count.entry(*ch).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut pairs = HashMap::<(char, char), u128>::new();
    let vec: Vec<_> = template.iter().collect();
    for w in vec.windows(2) {
        pairs.entry((*w[0], *w[1])).or_insert(1);
    }

    for _ in 0..steps {
        let mut new_pairs = HashMap::new();

        for ((a, b), cnt) in pairs {
            if let Some(c) = mapping.get(&(a, b)) {
                count.entry(*c).and_modify(|x| *x += cnt).or_insert(cnt);
                new_pairs.entry((a, *c)).and_modify(|x| *x += cnt).or_insert(cnt);
                new_pairs.entry((*c, b)).and_modify(|x| *x += cnt).or_insert(cnt);
            }
        }

        pairs = new_pairs;
    }

    count
}

fn part_1(template: &LinkedList<char>, mapping: &HashMap<(char, char), char>) -> u128 {
    let hmap = simulate_clever(template, mapping, 10);

    hmap.values().max().unwrap() - hmap.values().min().unwrap()
}

fn part_2(template: &LinkedList<char>, mapping: &HashMap<(char, char), char>) -> u128 {
    let hmap = simulate_clever(template, mapping, 40);

    hmap.values().max().unwrap() - hmap.values().min().unwrap()
}

fn parse(path: &Path) -> Result<(LinkedList<char>, HashMap<(char, char), char>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let template = lines.next().unwrap()?.chars().collect();
    lines.next();

    let mut hm = HashMap::new();
    for line in lines {
        let line = line?;
        let (a, b, c) = scan_fmt::scan_fmt!(&line, "{/./}{/./} -> {/./}", char, char, char)?;
        hm.insert((a, b), c);
    }

    Ok((template, hm))
}

fn main() -> Result<()> {
    let (template, mapping) = parse(&Path::new("input.txt"))?;

    let p1 = part_1(&template, &mapping);
    println!("Part 1 {}", p1);

    let p2 = part_2(&template, &mapping);
    println!("Part 2 {}", p2);

    Ok(())
}
