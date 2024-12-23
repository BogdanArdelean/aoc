use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;
use itertools::Itertools;
use scan_fmt::scan_fmt;

type Graph = HashMap<String, HashSet<String>>;
fn part1(g: &Graph) -> usize {
    let mut viz = HashSet::new();
    for (a, a_adj) in g {
        if !a.starts_with("t") { continue; }

        for (b, c) in a_adj.iter().tuple_combinations() {
            let b_adj = g.get(b).unwrap();
            if b_adj.contains(c) {
                let mut v = vec![a.clone(), b.clone(), c.clone()];
                v.sort();
                viz.insert(v);
            }
        }
    }
    viz.len()
}


fn part2(g: &Graph) -> Vec<String> {
    let upper_bound = g.values().map(|adj| adj.len()).max().unwrap();
    for max_sz in (3..=upper_bound).rev() {
        for (a, a_adj) in g {
            for neighbours in a_adj.iter().combinations(max_sz) {
                let mut found = true;
                for i in 0..neighbours.len()-1 {
                    for j in i+1..neighbours.len() {
                        let b = neighbours[i];
                        let c = neighbours[j];
                        found &= g.get(b).unwrap().contains(c);
                        if !found { break; }
                    }
                    if !found { break; }
                }

                if found {
                    let mut v = vec![a];
                    v.extend(neighbours.clone());
                    v.sort();
                    return v.iter().map(|s|s.clone().clone()).collect();
                }
            }
        }
    }

    panic!("Not found!")
}

fn parse(path: &Path) -> Result<Graph> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut graph = Graph::new();
    for line in reader.lines() {
        let line = line?;
        let (a, b) = scan_fmt!(&line, "{}-{}", String, String)?;
        graph.entry(a.clone()).or_default().insert(b.clone());
        graph.entry(b).or_default().insert(a);
    }

    Ok(graph)
}

fn main() -> Result<()> {
    let graph = parse(&Path::new("input.txt"))?;
    let p1 = part1(&graph);
    println!("Part 1: {}", p1);

    let p2 = part2(&graph);
    {
        print!("Part 2: ");
        for n in p2 {
            print!("{},", n);
        }
    }
    Ok(())
}
