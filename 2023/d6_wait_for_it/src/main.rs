use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use regex::Regex;
use anyhow::{Result, Ok, anyhow};

fn get_roots(a: f64, b: f64, c:f64) -> (f64, f64) {
    let s = (b*b - 4.0*a*c).sqrt();
    ((-b + s) / (2.0 * a), (-b - s) / (2.0 * a))
}

fn parse(path: &Path) -> Result<Vec<(f64, f64)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut it = reader.lines();

    // get time
    let mut times = Vec::<f64>::new();
    let row = it.next().unwrap().unwrap();
    let time = row.split(":").last().unwrap();
    let re = Regex::new(r"\d+").unwrap();
    for m in re.find_iter(time) {
        times.push(m.as_str().parse()?);
    }

    // get distance
    let mut distances = Vec::<f64>::new();
    let row = it.next().unwrap().unwrap();
    let distance= row.split(":").last().unwrap();
    let re = Regex::new(r"\d+").unwrap();
    for m in re.find_iter(distance) {
        distances.push(m.as_str().parse()?);
    }
    
    Ok(times
    .into_iter()
    .zip(distances.into_iter())
    .collect())
}

fn parse2(path: &Path) -> Result<(f64, f64)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut it = reader.lines();

    // get time
    let row = it.next().unwrap().unwrap();
    let time = row.split(":").last().unwrap();
    let time = time.replace(" ", "");
    let time: f64 = time.parse()?;

    // get distance
    let row = it.next().unwrap().unwrap();
    let distance = row.split(":").last().unwrap();
    let distance = distance.replace(" ", "");
    let distance: f64 = distance.parse()?;
    
    Ok((time, distance))
}

fn main() -> Result<()> {
    let eqs = parse(Path::new("input.txt"))?;

    let mut result = 1.0; 
    for (time, distance) in eqs {
        let (mut lower, mut upper) = get_roots(-1.0, time, -distance);
        lower +=  0.00000001;
        upper -=  0.00000001;

        lower = lower.ceil().max(0.0);
        upper = upper.floor().min(time);
        result *= upper - lower + 1.0;
    }
    println!("Part 1: {}", result as i32);

    let (time, distance) = parse2(Path::new("input.txt"))?;
    let (mut lower, mut upper) = get_roots(-1.0, time, -distance);
    lower +=  0.00000001;
    upper -=  0.00000001;

    lower = lower.ceil().max(0.0);
    upper = upper.floor().min(time);
    println!("Part 2: {}", (upper - lower + 1.0) as i32);
    
    let r1 = (1..4);
    let r2 = (2..5);
    println!("Intersection: {}", r1.)
    Ok(())
}
