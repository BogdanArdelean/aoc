use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type DependencyGraph = HashMap<i32, HashSet<i32>>;
type Update = Vec<i32>;
type UpdateList = Vec<Vec<i32>>;

fn parse(path: &Path) -> Result<(DependencyGraph, UpdateList)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut dg = DependencyGraph::new();
    let mut updates = UpdateList::new();

    let mut lines = reader.lines();
    loop {
        let line = lines.next().unwrap()?;
        if line.is_empty() {
            break;
        }

        let (a, b) = scan_fmt::scan_fmt!(&line, "{}|{}", i32, i32)?;
        dg
            .entry(b)
            .or_default()
            .insert(a);
    }

    for line in lines {
        let line = line?;
        updates.push(line.split(",").map(|s| s.parse().unwrap()).collect());
    }

    Ok((dg, updates))
}

fn satisfies_dependency(g: &DependencyGraph, before: i32, after: i32) -> bool {
    if let Some(h) = g.get(&before) {
        !h.contains(&after)
    } else {
        true
    }
}

fn is_update_valid(g: &DependencyGraph, update: &Update) -> bool {
    for i in 0..update.len() - 1 {
        for j in i + 1..update.len() {
            let a = update[i];
            let b = update[j];
            if !satisfies_dependency(g, a, b) {
                return false;
            }
        }
    }

    true
}

fn sort(g: &DependencyGraph, update: &Update) -> Update {
    let mut update = update.clone();
    for i in 0..update.len() - 1 {
        for j in i + 1..update.len() {
            let a = update[i];
            let b = update[j];
            if !satisfies_dependency(g, a, b) {
                update[i] = b;
                update[j] = a;
            }
        }
    }
    update
}

fn part1(g: &DependencyGraph, update_list: &UpdateList) -> i32 {
    update_list
        .iter()
        .filter(|u| is_update_valid(g, u))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn part2(g: &DependencyGraph, update_list: &UpdateList) -> i32 {
    update_list
        .iter()
        .filter(|u| !is_update_valid(g, u))
        .map(|u| sort(g, u))
        .map(|u| u[u.len() / 2])
        .sum()
}

fn main() -> Result<()> {
    let (g, u) = parse(&Path::new("input.txt"))?;

    let p1 = part1(&g, &u);
    println!("Part 1 {}", p1);

    let p2 = part2(&g, &u);
    println!("Part 2 {}", p2);
    Ok(())
}
