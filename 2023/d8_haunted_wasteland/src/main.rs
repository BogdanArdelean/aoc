use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use anyhow::{anyhow, Result};
use scan_fmt::scan_fmt;

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, (String, String)>
}

impl Graph {
    fn traverse(&self, steps: &String) -> i32 {
        let mut total_steps = 0;
        let mut steps_idx = 0;
        let steps = steps.chars().collect::<Vec<char>>();

        let mut current = "AAA".to_string();
        while current != "ZZZ" {
            let (left, right) = self.nodes.get(&current).unwrap();
            let instr = steps.get(steps_idx).unwrap();
            current = if *instr == 'R' {
                right.clone()
            } else {
                left.clone()
            };

            total_steps += 1;
            steps_idx += 1;
            steps_idx = steps_idx % steps.len();
        }

        total_steps
    }

    fn traverse_one(&self, source: &String, steps: &String) -> i32 {
        let mut total_steps = 0;
        let mut steps_idx = 0;
        let steps = steps.chars().collect::<Vec<char>>();

        let mut current = source.clone();
        while current.ends_with("Z") != true {
            let (left, right) = self.nodes.get(&current).unwrap();
            let instr = steps.get(steps_idx).unwrap();
            current = if *instr == 'R' {
                right.clone()
            } else {
                left.clone()
            };

            total_steps += 1;
            steps_idx += 1;
            steps_idx = steps_idx % steps.len();
        }

        total_steps
    }
}

fn parse(path: &Path) -> Result<(String, Graph)> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    let mut steps = "".to_string(); 
    reader.read_line(&mut steps)?;
    steps = scan_fmt!(&steps, "{}", String)?;

    let mut nodes = HashMap::<String, (String, String)>::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let (source, left, right) = scan_fmt!(&line, "{} = ({}, {})", String, String, String)?;
        nodes.insert(source, (left, right));
    }
    
    anyhow::Ok((steps, Graph {nodes}))
}

fn main() -> Result<()> {
    let (steps, graph) = parse(Path::new("input.txt"))?;
    let res = graph.traverse(&steps);
    println!("Part 1: {}", res);
    let start_nodes = graph.nodes.keys().filter(|k| k.ends_with("A")).cloned().collect::<Vec<String>>();
    
    let mut results = Vec::<u128>::new();
    for start in start_nodes {
        results.push(graph.traverse_one(&start, &steps) as u128);
    }
    let lcm: u128 = 1;
    let lcm = results.iter().fold(lcm, |l, r| {
        l*r / gcd::binary_u128(l, *r)
    });

    println!("Part2: {}", lcm);
    anyhow::Ok(())
}
