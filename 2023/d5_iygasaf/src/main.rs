use std::any;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::ops::Bound;
use anyhow::{anyhow, bail, Result};
use scan_fmt::scan_fmt;

#[derive(Debug, Default)]
struct EdgeMap {
    map: BTreeMap<i64, (i64, i64)>
}

impl EdgeMap {
    fn get_next(&self, k: i64) -> i64 {
        let range = self.map.range((Bound::Included(&0), Bound::Included(&k)));
        let range = range.last();
        if let Some((source, &(destination, incr))) = range {
            if source + incr > k {
                destination + (k - source)
            } else {
                k
            }
        } else {
            k
        }
    }

    fn get_next_neighbours(&self, (lower, upper): (i64, i64)) -> Vec<(i64, i64)> {
        let mut neighbours = Vec::<(i64, i64)>::new();
        
        let mut upper_gap = upper;

        let range = self.map.range((Bound::Included(&0), Bound::Included(&upper)));
        for (source, &(destination, incr)) in range.rev() {
            if source + incr <= lower {
                break;
            }
            if source + incr <= upper_gap {
                neighbours.push((source + incr, upper_gap));
            }
            let destination_lower = destination + (0.max(lower - source));
            let destination_upper = destination + incr.min(upper - source);
            neighbours.push((destination_lower, destination_upper));
            upper_gap = *source;
        }

        if upper_gap > lower {
            neighbours.push((lower, upper_gap));
        }

        neighbours
    }
}

struct Graph {
    edges: Vec<EdgeMap>,
}

impl Graph {
    fn traverse(&self, mut starting_point: i64) -> Vec<i64> {
        let mut traverse_path = Vec::<i64>::new();

        traverse_path.push(starting_point);
        for edge in &self.edges {
            starting_point = edge.get_next(starting_point);
            traverse_path.push(starting_point);
        }

        traverse_path
    }

    fn traverse_range(&self, (lower, upper): (i64, i64)) -> Vec<(i64, i64)> {
        let mut all_neigh = 0;
        let mut neighbours = vec![(lower, upper)];
        for edge in &self.edges {
            let mut next_neighbours = Vec::<(i64, i64)>::new();
            for r in neighbours {
                next_neighbours.append(&mut edge.get_next_neighbours(r));
            }
            all_neigh += next_neighbours.len();
            neighbours = next_neighbours;
        }

        println!("All neigh: {}", all_neigh);
        neighbours
    }
}

fn parse(path: &Path) -> anyhow::Result<(Vec<(i64, i64)>, Graph)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // parse seeds
    let mut seeds = Vec::<(i64, i64)>::new();
    let seeds_str = lines.next().ok_or(anyhow!("Should have seeds line"))??.split(": ").last().ok_or(anyhow!("Should have : and seeds"))?.to_string();
    for seed_str in seeds_str.split(" ").collect::<Vec<&str>>().chunks(2) {
        let seed_lower: i64 = seed_str[0].parse()?;
        let seed_upper: i64 = seed_lower + seed_str[1].parse::<i64>()?;
        seeds.push((seed_lower, seed_upper));
    }

    // parse graph
    lines.next();
    let mut tree_map = BTreeMap::<i64, (i64, i64)>::new();
    let mut edges = Vec::<EdgeMap>::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            edges.push(EdgeMap { map: tree_map });
            tree_map = BTreeMap::<i64, (i64, i64)>::new();
            continue;
        }

        if line.chars().nth(0).ok_or(anyhow!("Line does not have characters"))?.is_alphabetic() {
            continue;
        }

        let (destination, source, range) = scan_fmt!(&line, "{} {} {}", i64, i64, i64)?;
        tree_map.insert(source, (destination, range));
    }
    edges.push(EdgeMap { map: tree_map });
    anyhow::Ok((seeds, Graph {edges}))
}

fn main() -> anyhow::Result<()> {
    let (seeds, graph) = parse(Path::new("input.txt"))?;

    // let part1 = seeds.iter()
    // .map(|s| *graph.traverse(*s).last().unwrap())
    // .min()
    // .unwrap();
    
    // println!("Part 1: {}", part1);
    let part2 = seeds.iter()
    .flat_map(|s| graph.traverse_range(*s))
    .map(|v| v.0)
    .min().unwrap();
    
    println!("Part2: {:?}", part2);
    anyhow::Ok(())
}
