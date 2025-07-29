use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{Result, anyhow};

type SlabId = i32;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct TwoDimPoint {
    x: i32,
    y: i32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct ThreeDimPoint {
    x: i32,
    y: i32,
    z: i32
}

#[derive(Debug, Clone, Copy)]
struct Slab {
    id: SlabId,
    p1: ThreeDimPoint,
    p2: ThreeDimPoint,
}

impl Slab {
    fn new(id: SlabId, p1: ThreeDimPoint, p2: ThreeDimPoint) -> Slab {
        Slab {
            id,
            p1,
            p2
        }
    }

    fn get_xy_view(&self) -> (i32, Vec<TwoDimPoint>) {
        let mut points = Vec::<_>::new();

        if self.p1.x != self.p2.x {
            let x_min = self.p1.x.min(self.p2.x);
            let x_max = self.p1.x.max(self.p2.x);

            for i in x_min..=x_max {
                points.push(TwoDimPoint { x: i, y: self.p1.y });
            }
        } else if self.p1.y != self.p2.y {
            let y_min = self.p1.y.min(self.p2.y);
            let y_max = self.p1.y.max(self.p2.y);

            for i in y_min..=y_max {
                points.push(TwoDimPoint { x: self.p1.x, y: i });
            }
        } else {
            points.push(TwoDimPoint { x: self.p1.x, y: self.p2.y });
        }

        (self.height(), points)
    }

    fn height(&self) -> i32 {
        self.p1.z.max(self.p2.z) - self.p1.z.min(self.p2.z) + 1
    }
}

fn fall(slabs: &Vec<Slab>) -> (HashMap<i32, HashSet<SlabId>>, HashMap<i32, HashSet<SlabId>>) {
    let mut supporting = HashMap::<_, _>::new();
    let mut supported = HashMap::<_, _>::new();
    let mut depth_map = HashMap::<TwoDimPoint, (i32, Slab)>::new();

    // sort by z
    let mut slabs = slabs.clone();
    slabs.sort_by(|a, b| {
        a.p1.z.min(a.p2.z).cmp(&b.p1.z.min(b.p2.z))
    });

    for falling_slab in &slabs {
        let (height, xy) = falling_slab.get_xy_view();
        
        let max_height = xy.iter().flat_map(|p| 
            depth_map.get(&p).and_then(|(h, _)| Some(*h)))
            .max().unwrap_or(0);
        
        for p in &xy {
            if let Some((h, slab)) = depth_map.get(&p) {
                if *h == max_height {
                    let x = supporting.entry(slab.id).or_insert(HashSet::<_>::new());
                    x.insert(falling_slab.id);

                    let x = supported.entry(falling_slab.id).or_insert(HashSet::<_>::new());
                    x.insert(slab.id);
                }
            }
        }

        for p in &xy {
            depth_map.insert(*p, (max_height + height, *falling_slab));
        }

        supporting.insert(falling_slab.id, HashSet::<_>::new());
    }

    (supporting, supported)
}

fn count_for_destruction(slab_support_map: &HashMap<i32, HashSet<i32>>, slab_supporting_map: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut count = 0;
    
    for (slab_id, supported_slabs) in slab_support_map {
        // println!("Slab Id: {}", slab_id);
        
        let mut all_supported = true;
        for supported_slab in supported_slabs {
            if let Some(set) = slab_supporting_map.get(supported_slab) {
                if set.len() <= 1 {
                    all_supported = false;
                    break;
                }
            }
        }

        if supported_slabs.len() < 1 || all_supported {
            count += 1;
        }
    }

    count
}

fn count_chain_reaction(slab_support_map: &HashMap<i32, HashSet<i32>>, slab_supporting_map: &HashMap<i32, HashSet<i32>>, id: i32) -> i32 {
    let mut result = 0;

    let mut q = VecDeque::<i32>::new();
    let mut will_fall = HashSet::<i32>::new();

    q.push_back(id);
    will_fall.insert(id);

    while !q.is_empty() {
        let next = q.pop_front().unwrap();

        for supports in slab_support_map.get(&next) {
            for supported in supports {
                if let Some(set) = slab_supporting_map.get(supported) {
                    if set.is_subset(&will_fall) {
                        will_fall.insert(*supported);
                        q.push_back(*supported);
                    }
                } else {
                    will_fall.insert(*supported);
                    q.push_back(*supported);
                }
            }
        }
    }

    will_fall.len() as i32 - 1
}

fn parse(path: &Path) -> Result<Vec<Slab>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut slabs = Vec::<_>::new();
    for (id, line) in reader.lines().enumerate() {
        let line = line?;
        let (x1, y1, z1, x2, y2, z2) = scan_fmt::scan_fmt!(&line, "{},{},{}~{},{},{}", i32, i32, i32, i32, i32, i32)?;

        let p1 = ThreeDimPoint {
            x: x1,
            y: y1,
            z: z1,
        };

        let p2 = ThreeDimPoint {
            x: x2,
            y: y2,
            z: z2,
        };

        slabs.push(Slab::new(id as i32, p1, p2));
    }

    anyhow::Ok(slabs)
}

fn main() -> Result<()> {
    let slabs = parse(&Path::new("input.txt"))?;
    let (supporting, supported) = fall(&slabs);
    let part_1 = count_for_destruction(&supporting, &supported);
    println!("Part 1: {}", part_1);

    let mut part_2 = 0;
    for id in supporting.keys() {
        part_2 += count_chain_reaction(&supporting, &supported, *id);
    }
    println!("Part 2: {}", part_2);
    anyhow::Ok(())
}
