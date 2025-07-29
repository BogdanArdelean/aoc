use std::f64::INFINITY;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use anyhow::{Result, anyhow};
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(Debug, Clone, Copy, Default)]
struct TDPoint {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TwoDPoint {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct TDVec {
    start: TDPoint,
    velocity: TDPoint
}

#[derive(Debug, Clone, Copy)]
struct LineEq {
    slope: f64,
    offset: f64
}

impl LineEq {
    fn find_y_intersect(&self, other: &LineEq) -> f64 {
        // a1x + b1 = a2x + b2
        // x = (b2 - b2) / (a1 - a2)
        if self.slope - other.slope == 0.0 {
            (other.offset - self.offset) / (self.slope - other.slope)
        } else {
            (other.offset - self.offset) / (self.slope - other.slope)
        }
    }

    fn get_y(&self, x: f64) -> f64 {
        self.slope * x + self.offset
    }
}

impl TDVec {
    fn get_xy_line_eq(&self) -> LineEq {
        let p1 = TwoDPoint {x: self.start.x, y: self.start.y };
        let p2 = TwoDPoint {x: self.start.x + self.velocity.x, y: self.start.y + self.velocity.y };
        
        let slope = if p2.x - p1.x == 0.0 {
            f64::INFINITY
        } else if p2.y == p1.y {
            0.0
        } else {
            (p2.y - p1.y) / (p2.x - p1.x)
        };
        // ax + b = y
        // slope * self.start.x - self.start.y = -b
        let offset = self.start.y - slope*self.start.x;
        if !f64::is_normal(slope) || !f64::is_normal(offset) {
            // println!("XY line abnormal: {}, {}", slope, offset);
        }
        LineEq {
            slope,
            offset
        }
    }

    fn get_xz_line_eq(&self) -> LineEq {
        let p1 = TwoDPoint {x: self.start.x, y: self.start.z };
        let p2 = TwoDPoint {x: self.start.x + self.velocity.x, y: self.start.z + self.velocity.z };
        
        let slope = if p2.x - p1.x == 0.0 {
            f64::INFINITY
        } else if p2.y == p1.y {
            0.0
        } else {
            (p2.y - p1.y) / (p2.x - p1.x)
        };
        // ax + b = y
        // slope * self.start.x - self.start.y = -b
        let offset = self.start.z - slope*self.start.x;
        if !f64::is_normal(slope) || !f64::is_normal(offset) {
            // println!("XZ line abnormal: {}, {}", slope, offset);
        }
        LineEq {
            slope,
            offset
        }
    }

    fn get_t_at_y(&self, y: f64) -> f64 {
        (y - self.start.y) / self.velocity.y
    }

    fn get_t_at_z(&self, y: f64) -> f64 {
        (y - self.start.y) / self.velocity.y
    }

    fn minus_velocity(&self, other: &TDVec) -> TDVec {
        TDVec { start: self.start, velocity: TDPoint { x: self.velocity.x - other.velocity.x, y: self.velocity.y - other.velocity.y, z: self.velocity.z - other.velocity.z } }
    }
}

fn parse(path: &Path) -> Result<Vec<TDVec>> {
    let mut result = Vec::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let (sx, sy, sz, vx, vy, vz) = scan_fmt!(&line, "{}, {}, {} @ {}, {}, {}", f64, f64, f64, f64, f64, f64)?;
        result.push(TDVec { start: TDPoint {x: sx, y: sy, z:sz}, velocity: TDPoint { x: vx, y: vy, z: vz } });
    }
    anyhow::Ok(result)
}

fn intersection(v1: &LineEq, v2: &LineEq) -> TwoDPoint {
    let x = v1.find_y_intersect(&v2);
    let y = v2.get_y(x);
    if !f64::is_normal(x) || !f64::is_normal(y) {
        print!("");
    }
    TwoDPoint { x, y }
}

fn part_1(vectors: &Vec<TDVec>, low: f64, high: f64) -> i32 {
    let mut result = 0;
    for (v1, v2) in vectors.iter().tuple_combinations() {
        let i = intersection(&v1.get_xy_line_eq(), &v2.get_xy_line_eq());
        if i.x < low || i.x > high { continue; };
        if i.y < low || i.y > high { continue; };
    
        let t1 = v1.get_t_at_y(i.y);
        if t1 < 0.0 { continue; }
        
        let t2 = v2.get_t_at_y(i.y);
        if t2 < 0.0 { continue; };

        result += 1;
    }

    result
}

fn get_normal_intersection_point_xy(vectors: &Vec<TDVec>) -> TwoDPoint {
    for (v1, v2) in vectors.iter().tuple_combinations() {
        let intersection_point = intersection(&v1.get_xy_line_eq(), &v2.get_xy_line_eq());
        if f64::is_normal(intersection_point.x) && f64::is_normal(intersection_point.y) {
            return intersection_point;
        } else {
            print!("");
        }
    }

    TwoDPoint { x: f64::NEG_INFINITY, y: f64::NEG_INFINITY }
}

fn get_normal_intersection_point_xz(vectors: &Vec<TDVec>) -> TwoDPoint {
    for (v1, v2) in vectors.iter().tuple_combinations() {
        let intersection_point = intersection(&v1.get_xz_line_eq(), &v2.get_xz_line_eq());
        if f64::is_normal(intersection_point.x) && f64::is_normal(intersection_point.y) {
            return intersection_point;
        } else {
            print!("");
        }
    }

    TwoDPoint { x: f64::NEG_INFINITY, y: f64::NEG_INFINITY }
}

fn part_2(vectors: &Vec<TDVec>) -> TDPoint {

    for x in -1000..=2000 {
        for y  in -1000..=2000 {
            let nvec = TDVec { start: Default::default(), velocity: TDPoint {x: x.into(), y: y.into(), z: 0.0}};
            let all = vectors.iter().map(|x| x.minus_velocity(&nvec)).collect_vec();
            
            let intersection_point = get_normal_intersection_point_xy(&all);
            if !f64::is_normal(intersection_point.x) || !f64::is_normal(intersection_point.y) {
                // println!("Abnormal XY{}, {}, {:?}", x, y, intersection_point);
                continue;
            }
            let mut viable = true;
            for tvec in all {
                let eq = tvec.get_xy_line_eq();
                if eq.slope == INFINITY {
                    print!("");
                }
                let tvecy = tvec.get_xy_line_eq().get_y(intersection_point.x);
                if !f64::is_normal(tvecy) {
                    // println!("Abnormal Y: {}, {}, {}, {:?}", tvecy, x, y, intersection_point);
                    continue;
                }

                if f64::abs(tvecy - intersection_point.y) > 1.0 {
                    viable = false;
                    break;
                }

                // if tvec.get_t_at_y(tvecy) < 0.0 {
                //     viable = false;
                //     break;
                // }
            }
            if !viable {
                continue;
            }
            println!("Here {}, {}", x, y);
            for z in -2000..=2000 {
                let nvec = TDVec { start: Default::default(), velocity: TDPoint {x: x.into(), y: y.into(), z: z.into()}};
                let all = vectors.iter().map(|x| x.minus_velocity(&nvec)).collect_vec();
                let intersection_point2 = get_normal_intersection_point_xz(&all);
                if !f64::is_normal(intersection_point2.x) || !f64::is_normal(intersection_point2.y) {
                    println!("Abnormal XZ {}, {}, {:?}", x, z, intersection_point2);
                    continue;
                }
                if f64::abs(intersection_point2.x - intersection_point.x) > 1.0 {
                    continue;
                }

                let mut viable = true;
                for tvec in all {
                    let tvecz = tvec.get_xz_line_eq().get_y(intersection_point2.x);
                    if !f64::is_normal(tvecz) {
                        println!("Abnormal Z: {}, {}, {}, {:?}", tvecz, x, z, intersection_point2);
                        continue;
                    }
                    if f64::abs(tvecz - intersection_point2.y) > 50.0 {
                        viable = false;
                    }
                }

                if !viable {
                    continue;
                }

                return TDPoint {x: intersection_point.x, y: intersection_point.y, z: intersection_point2.y};
            }
        }
    }
    TDPoint { x: f64::NEG_INFINITY, y: f64::NEG_INFINITY, z: f64::NEG_INFINITY }
}
 
fn main() -> Result<()> {
    let vecs = parse(&Path::new("input.txt"))?;
    // let p1 = part_1(&vecs, 200000000000000.0, 400000000000000.0);
    // let p1 = part_1(&vecs, 7.0, 27.0);
    let p2 = part_2(&vecs);

    // println!("Part 1: {}", p1);
    println!("Part 2: {:?}", f64::round(p2.x + p2.y + p2.z));

    anyhow::Ok(())
}
