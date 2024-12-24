use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;
use scan_fmt::scan_fmt;
use itertools::Itertools;

type Connections = HashMap<String, Vec<String>>;
type Gates = HashMap<String, String>;
type Inputs = Vec<(String, i64)>;

fn apply(a: i64, b: i64, gate: &str) -> i64 {
    match gate {
        "XOR" => a ^ b,
        "AND" => a & b,
        "OR"  => a | b,
        _ => panic!("Unknown gate {}",gate)
    }
}

fn simulate(connections: &Connections, gates: &Gates, inputs: &Inputs) -> HashMap<String, i64> {
    let mut gate_results = HashMap::from_iter(inputs.iter().cloned());
    let mut outputs: VecDeque<(String, i64)> = inputs.iter().cloned().collect();
    let mut pending = HashMap::<String, i64>::new();

    while !outputs.is_empty() {
        let mut output_aux = VecDeque::new();
        while let Some((gate, a)) = outputs.pop_front() {
            for connection in connections.get(&gate).unwrap_or(&vec![]) {
                if let Some(b) = pending.get(connection).cloned() {
                    let o = apply(a, b, gates.get(connection).unwrap());
                    output_aux.push_back((connection.clone(), o));
                    gate_results.insert(connection.clone(), o);
                    pending.remove(connection);
                } else {
                    pending.insert(connection.clone(), a);
                }
            }
        }
        outputs = output_aux;
    }

    gate_results
}

fn parse(path: &Path) -> Result<(Connections, Gates, Inputs)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut inputs = vec![];
    while let Some(line) = lines.next() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        inputs.push(scan_fmt!(&line, "{}: {}", String, i64)?);
    }

    let mut connections = Connections::new();
    let mut gates = Gates::new();
    while let Some(line) = lines.next() {
        let line = line?;
        let (i1, op, i2, o) = scan_fmt!(&line, "{} {} {} -> {}", String, String, String, String)?;
        gates.insert(o.clone(), op);
        connections.entry(i1).or_default().push(o.clone());
        connections.entry(i2).or_default().push(o.clone());
    }

    Ok((connections, gates, inputs))
}

fn part1(connections: &Connections, gates: &Gates, inputs: &Inputs) -> i64 {
    let outputs = simulate(connections, gates, inputs);

    let res: Vec<_> = outputs
        .iter()
        .filter(|v| v.0.starts_with("z"))
        .sorted_by_key(|v| v.0)
        .map(|v| {
            println!("{} {}", v.0, v.1);
            v.1.clone()
        })
        .collect();

    let mut p1 = 0;
    for o in res.iter().rev() {
        p1 = (p1 << 1) | o;
    }

    p1
}

fn decode(outputs: &HashMap<String, i64>, sw: &str) -> i64 {
    let res: Vec<_> = outputs
        .iter()
        .filter(|v| v.0.starts_with(sw))
        .sorted_by_key(|v| v.0)
        .map(|v| {
            v.1.clone()
        })
        .collect();

    let mut p1 = 0;
    for o in res.iter().rev() {
        p1 = (p1 << 1) | o;
    }

    p1
}

/*
    Used graphviz to iteratively find problematic bits.
    Used special input cases to test carries. e.g. alternating bits in inputs
    Automatic solution after Christmas :D
 */
fn part2(connections: &Connections, gates: &Gates, inputs: &Inputs) -> i64 {
    let outputs = simulate(connections, gates, inputs);

    let x = decode(&outputs, "x");
    let y = decode(&outputs, "y");
    let z = decode(&outputs, "z");
    println!("X:{:b} Y:{:b} Z:{:b} R:{:b}", x, y, x+y, z);
    assert_eq!(x+y, z);
    0
}


fn main() -> Result<()> {
    let (connections, gates, inputs) = parse(&Path::new("input.txt"))?;
    let p1 = part1(&connections, &gates, &inputs);
    part2(&connections, &gates, &inputs);
    println!("Part 1: {}", p1);

    Ok(())
}
