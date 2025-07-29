use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Cuboid {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    z1: i64,
    z2: i64,
    sign: i64, // 1 for addition, -1 for subtraction
}

impl Cuboid {
    // Calculate the intersection between two cuboids
    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let nx1 = max(self.x1, other.x1);
        let nx2 = min(self.x2, other.x2);
        if nx1 > nx2 {
            return None;
        }
        let ny1 = max(self.y1, other.y1);
        let ny2 = min(self.y2, other.y2);
        if ny1 > ny2 {
            return None;
        }
        let nz1 = max(self.z1, other.z1);
        let nz2 = min(self.z2, other.z2);
        if nz1 > nz2 {
            return None;
        }
        Some(Cuboid {
            x1: nx1,
            x2: nx2,
            y1: ny1,
            y2: ny2,
            z1: nz1,
            z2: nz2,
            sign: 0, // sign will be set appropriately by the caller
        })
    }

    // Calculate the volume of the cuboid
    fn volume(&self) -> i64 {
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1)
    }
}

#[derive(Debug)]
enum StepType {
    On,
    Off,
}

#[derive(Debug)]
struct Step {
    step_type: StepType,
    cuboid: Cuboid,
}

impl FromStr for Step {
    type Err = String;

    fn from_str(s: &str) -> Result<Step, Self::Err> {
        let parts: Vec<&str> = s.trim().split(' ').collect();
        if parts.len() != 2 {
            return Err("Invalid step format".to_string());
        }
        let step_type = match parts[0] {
            "on" => StepType::On,
            "off" => StepType::Off,
            _ => return Err("Invalid step type".to_string()),
        };
        let ranges: Vec<&str> = parts[1].split(',').collect();
        if ranges.len() != 3 {
            return Err("Invalid ranges".to_string());
        }
        let mut coords = Vec::new();
        for range in ranges {
            let range_parts: Vec<&str> = range.split('=').collect();
            if range_parts.len() != 2 {
                return Err("Invalid range format".to_string());
            }
            let nums: Vec<i64> = range_parts[1]
                .split("..")
                .map(|n| n.parse::<i64>().map_err(|_| "Invalid number".to_string()))
                .collect::<Result<_, _>>()?;
            if nums.len() != 2 {
                return Err("Invalid number of coordinates".to_string());
            }
            coords.push((nums[0], nums[1]));
        }
        Ok(Step {
            step_type,
            cuboid: Cuboid {
                x1: coords[0].0,
                x2: coords[0].1,
                y1: coords[1].0,
                y2: coords[1].1,
                z1: coords[2].0,
                z2: coords[2].1,
                sign: 1, // initial sign is positive; will adjust based on overlap
            },
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the input file
    let file = File::open("input.txt").map_err(|e| format!("Failed to open input.txt: {}", e))?;
    let reader = BufReader::new(file);

    // Parse steps
    let steps: Vec<Step> = reader
        .lines()
        .filter_map(|line| {
            match line {
                Ok(l) if !l.trim().is_empty() => Some(l.parse::<Step>()),
                _ => None,
            }
        })
        .collect::<Result<_, _>>()
        .map_err(|e| format!("Failed to parse steps: {}", e))?;

    // List to hold all existing cuboids with their signs
    let mut cuboids: Vec<Cuboid> = Vec::new();

    for step in steps {
        let mut new_cuboids: Vec<Cuboid> = Vec::new();
        for existing in &cuboids {
            if let Some(intersect) = step.cuboid.intersection(existing) {
                // The sign is opposite to the existing cuboid
                let sign = -existing.sign;
                let mut intersect_cuboid = intersect.clone();
                intersect_cuboid.sign = sign;
                new_cuboids.push(intersect_cuboid);
            }
        }
        if let StepType::On = step.step_type {
            // Add the new cuboid itself
            let mut on_cuboid = step.cuboid.clone();
            on_cuboid.sign = 1;
            new_cuboids.push(on_cuboid);
        }
        // Add all new cuboids to the list
        cuboids.extend(new_cuboids);
    }

    // Calculate the total volume
    let total: i64 = cuboids
        .iter()
        .map(|c| c.volume() * c.sign)
        .sum();

    println!("{}", total);

    Ok(())
}