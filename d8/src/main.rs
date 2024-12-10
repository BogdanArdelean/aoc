use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Location {
    r: i32,
    c: i32,
}

impl Location {
    fn new(r: i32, c: i32) -> Self {
        Self { r, c }
    }

    fn get_antinode_locations(n1: &Location, n2: &Location, m: &AntennaMap) -> Vec<Location> {
        let dr = n1.r - n2.r;
        let dc = n1.c - n2.c;
        let mut res = vec![];

        let an1 = Location::new(n1.r + dr, n1.c + dc);
        let an2 = Location::new(n2.r - dr, n2.c - dc);

        if m.is_location_on_map(&an1) {
            res.push(an1);
        }
        if m.is_location_on_map(&an2) {
            res.push(an2);
        }

        res
    }

    fn get_antinode_locations_extended(n1: &Location, n2: &Location, m: &AntennaMap) -> Vec<Location> {
        let dr = n1.r - n2.r;
        let dc = n1.c - n2.c;
        let mut res = vec![];

        let mut loc = n1.clone();
        while m.is_location_on_map(&loc) {
            res.push(loc.clone());
            loc = Location::new(loc.r + dr, loc.c + dc);
        }

        loc = n2.clone();
        while m.is_location_on_map(&loc) {
            res.push(loc.clone());
            loc = Location::new(loc.r - dr, loc.c - dc);
        }

        res
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct AntiNode {
    location: Location,
    antenna_type: char,
}

#[derive(Debug, Clone)]
struct AntennaMap {
    rows: i32,
    cols: i32,
    antennas: HashMap<char, Vec<Location>>,
}

impl AntennaMap {
    fn is_location_on_map(&self, location: &Location) -> bool {
        location.r >= 0 && location.r < self.rows && location.c >= 0 && location.c < self.cols
    }

    fn compute_antinodes<F>(&self, antinode_calculator: F) -> Vec<AntiNode>
    where
        F: Fn(&Location, &Location, &AntennaMap) -> Vec<Location>  {
        let mut res = vec![];
        for (ant_type, locations) in &self.antennas {
            for i in 0..locations.len() - 1 {
                for j in i + 1..locations.len() {
                    let l1 = locations[i];
                    let l2 = locations[j];

                    let antinode_locs = antinode_calculator(&l1, &l2, &self);
                    for a_loc in antinode_locs {
                        res.push(AntiNode {
                            location: a_loc,
                            antenna_type: *ant_type,
                        });
                    }
                }
            }
        }

        res
    }
}

fn part1(m: &AntennaMap) -> usize {
    m.compute_antinodes(Location::get_antinode_locations)
        .iter()
        .map(|a| a.location.clone())
        .collect::<HashSet<Location>>()
        .len()
}

fn part2(m: &AntennaMap) -> usize {
    m.compute_antinodes(Location::get_antinode_locations_extended)
        .iter()
        .map(|a| a.location.clone())
        .collect::<HashSet<Location>>()
        .len()
}

fn parse(path: &Path) -> Result<AntennaMap> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut rows = 0;
    let mut cols = 0;
    let mut antennas = HashMap::<char, Vec<Location>>::new();
    for line in reader.lines() {
        let line = line?;

        for (idx, chr) in line.chars().enumerate() {
            cols = cols.max(idx as i32 + 1);
            if chr == '.' {
                continue;
            }

            let loc = Location::new(rows, idx as i32);
            antennas
                .entry(chr)
                .or_default()
                .push(loc)
        }

        rows += 1;
    }

    Ok(AntennaMap {
        rows,
        cols,
        antennas,
    })
}

fn main() -> Result<()> {
    let antenna_map = parse(&Path::new("input.txt"))?;
    let p1 = part1(&antenna_map);
    println!("Part 1 {}", p1);

    let p2 = part2(&antenna_map);
    println!("Part 2 {}", p2);
    Ok(())
}
