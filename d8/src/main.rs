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

    fn get_antinode_locations(&self, other: &Location) -> [Location; 2] {
        let dr = self.r - other.r;
        let dc = self.c - other.c;

        [
            Location::new(self.r + dr, self.c + dc),
            Location::new(other.r - dr, other.c - dc),
        ]
    }

    fn get_antinode_locations_clamped(&self, other: &Location, m: &AntennaMap) -> Vec<Location> {
        let mut res = vec![];
        let dr = self.r - other.r;
        let dc = self.c - other.c;

        let mut loc = self.clone();
        while m.is_location_on_map(&loc) {
            res.push(loc.clone());
            loc = Location::new(loc.r + dr, loc.c + dc);
        }

        loc = other.clone();
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

    fn compute_antinodes(&self) -> Vec<AntiNode> {
        let mut res = vec![];
        for (ant_type, locations) in &self.antennas {
            for i in 0..locations.len() - 1 {
                for j in i + 1..locations.len() {
                    let l1 = locations[i];
                    let l2 = locations[j];

                    let antinodes_locs = l1.get_antinode_locations(&l2);

                    if self.is_location_on_map(&antinodes_locs[0]) {
                        res.push(AntiNode {
                            location: antinodes_locs[0],
                            antenna_type: *ant_type,
                        });
                    }
                    if self.is_location_on_map(&antinodes_locs[1]) {
                        res.push(AntiNode {
                            location: antinodes_locs[1],
                            antenna_type: *ant_type,
                        });
                    }
                }
            }
        }

        res
    }

    fn compute_antinodes_updated(&self) -> Vec<AntiNode> {
        let mut res = vec![];
        for (ant_type, locations) in &self.antennas {
            for i in 0..locations.len() - 1 {
                for j in i + 1..locations.len() {
                    let l1 = locations[i];
                    let l2 = locations[j];

                    let antinodes_locs = l1.get_antinode_locations_clamped(&l2, &self);
                    for a_loc in antinodes_locs {
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
    m.compute_antinodes()
        .iter()
        .map(|a| a.location.clone())
        .collect::<HashSet<Location>>()
        .len()
}

fn part2(m: &AntennaMap) -> usize {
    m.compute_antinodes_updated()
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
                .and_modify(|v| v.push(loc))
                .or_insert(vec![loc]);
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
