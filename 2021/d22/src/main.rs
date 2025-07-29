use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone)]
enum InstructionType {
    On,
    Off
}

#[derive(Debug, Clone)]
struct Cuboid {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
    i: InstructionType
}

impl Cuboid {
    fn cubes(&self) -> i64 {
        (self.x.1 + 1 - self.x.0).abs() *
        (self.y.1 + 1 - self.y.0).abs() *
        (self.z.1 + 1 - self.z.0).abs()
    }

    fn on_cubes(&self) -> i64 {
        match self.i {
            InstructionType::On => self.cubes(),
            InstructionType::Off => 0
        }
    }

    fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        // Determine the overlapping range for each axis
        let x0 = self.x.0.max(other.x.0);
        let x1 = self.x.1.min(other.x.1);
        if x0 > x1 {
            return None; // No overlap on the x-axis
        }

        let y0 = self.y.0.max(other.y.0);
        let y1 = self.y.1.min(other.y.1);
        if y0 > y1 {
            return None; // No overlap on the y-axis
        }

        let z0 = self.z.0.max(other.z.0);
        let z1 = self.z.1.min(other.z.1);
        if z0 > z1 {
            return None; // No overlap on the z-axis
        }

        // Determine the InstructionType for the intersection.
        // This logic may vary based on your specific use case.
        // Here, we'll toggle the InstructionType.
        let intersect_type = match (&self.i, &other.i) {
            (InstructionType::On, InstructionType::Off) => InstructionType::Off,
            (InstructionType::On, InstructionType::On) => InstructionType::On,
            (InstructionType::Off, InstructionType::On) => InstructionType::On,
            (InstructionType::Off, InstructionType::Off) => InstructionType::Off,
        };

        Some(Cuboid {
            x: (x0, x1),
            y: (y0, y1),
            z: (z0, z1),
            i: intersect_type,
        })
    }

    fn difference(&self, other: &Cuboid) -> Vec<Cuboid> {
        // First, compute the intersection.
        let intersection = self.intersect(other);

        // If there's no intersection, the difference is just self.
        if intersection.is_none() {
            return vec![self.clone()];
        }

        let intersection = intersection.unwrap();

        let mut diffs = Vec::new();

        // Split along the X-axis
        if self.x.0 < intersection.x.0 {
            diffs.push(Cuboid {
                x: (self.x.0, intersection.x.0 - 1),
                y: self.y.clone(),
                z: self.z.clone(),
                i: InstructionType::On
            });
        }
        if intersection.x.1 < self.x.1 {
            diffs.push(Cuboid {
                x: (intersection.x.1 + 1, self.x.1),
                y: self.y.clone(),
                z: self.z.clone(),
                i: InstructionType::On
            });
        }

        // Split along the Y-axis
        if self.y.0 < intersection.y.0 {
            diffs.push(Cuboid {
                x: (intersection.x.0, intersection.x.1),
                y: (self.y.0, intersection.y.0 - 1),
                z: self.z.clone(),
                i: InstructionType::On
            });
        }
        if intersection.y.1 < self.y.1 {
            diffs.push(Cuboid {
                x: (intersection.x.0, intersection.x.1),
                y: (intersection.y.1 + 1, self.y.1),
                z: self.z.clone(),
                i: InstructionType::On
            });
        }

        // Split along the Z-axis
        if self.z.0 < intersection.z.0 {
            diffs.push(Cuboid {
                x: (intersection.x.0, intersection.x.1),
                y: (intersection.y.0, intersection.y.1),
                z: (self.z.0, intersection.z.0 - 1),
                i: InstructionType::On
            });
        }
        if intersection.z.1 < self.z.1 {
            diffs.push(Cuboid {
                x: (intersection.x.0, intersection.x.1),
                y: (intersection.y.0, intersection.y.1),
                z: (intersection.z.1 + 1, self.z.1),
                i: InstructionType::On
            });
        }

        diffs
    }
}

fn parse(path: &Path) -> Result<Vec<Cuboid>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut res = vec![];
    for line in reader.lines() {
        let line = line?;
        let (i, x1, x2, y1, y2, z1, z2) = scan_fmt::scan_fmt!(&line, "{} x={}..{},y={}..{},z={}..{}", String, i64, i64, i64, i64, i64, i64)?;

        let i = if i == "on" {
            InstructionType::On
        } else {
            InstructionType::Off
        };
        res.push(Cuboid {
            x: (x1, x2),
            y: (y1, y2),
            z: (z1, z2),
            i,
        });
    }

    Ok(res)
}

fn part1_ok(instructions: &Vec<Cuboid>) -> i64 {
    let mut v = vec![instructions[0].clone()];

    for i in 1..instructions.len() {
        let top = instructions[i].clone();
        match &top.i {
            InstructionType::On => {
                let mut v_aux = vec![top];
                for bottom in &v {
                    let mut v_aux_2 = vec![];
                    for top in &v_aux {
                        v_aux_2.extend(top.difference(bottom));
                    }
                    v_aux = v_aux_2;
                }
                v.extend(v_aux);
            }
            InstructionType::Off => {
                let mut v_aux = vec![];
                for bottom in &v {
                    if top.intersect(bottom).is_none() {
                        v_aux.push(bottom.clone());
                    } else {
                        v_aux.extend(bottom.difference(&top));
                    }
                }
                v = v_aux;
            }
        }
    }
    v.iter().map(|v| v.cubes()).sum()
}

fn main() -> Result<()> {
    let instructions = parse(&Path::new("input.txt"))?;
    println!("{}", part1_ok(&instructions));
    Ok(())
}
