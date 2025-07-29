use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};
use std::path::Path;
use anyhow::Result;
use scan_fmt::parse::scan;

#[derive(Debug, Clone, Copy)]
enum Orientation {
    XZ,  // (X, Y, Z)
    XY,  // (X, -Z, Y)
    XNZ, // (X, -Y, -Z)
    XNY, // (X, Z, -Y)

    NXZ, //  (-X, -Y, Z)
    NXY, //  (-X, Z, Y)
    NXNZ, // (-X, Y, -Z)
    NXNY, // (-X, -Z, -Y)

    YX, // (Y, Z, X)
    YZ, // (Y, -X, Z)
    YNX, // (Y, -Z, -X)
    YNZ, // (Y, X, -Z)

    NYX, //  (-Y, -Z, X)
    NYZ, //  (-Y, X, Z)
    NYNX, // (-Y, Z, -X)
    NYNZ, // (-Y, -X, -Z)

    ZX, //  (Z, -Y, X)
    ZY, //  (Z, X, Y)
    ZNX, // (Z, Y, -X)
    ZNY, // (Z, -X, -Y)

    NZX, //  (-Z, Y, X)
    NZY, //  (-Z, -X, Y)
    NZNX, // (-Z, -Y, -X)
    NZNY, // (-Z, X, -Y)
}

impl Orientation {
    fn all_variants() -> &'static [Self] {
        &[
            Self::XZ,  // (X, Y, Z)
            Self::XY,  // (X, -Z, Y)
            Self::XNZ, // (X, -Y, -Z)
            Self::XNY, // (X, Z, -Y)

            Self::NXZ, //  (-X, -Y, Z)
            Self::NXY, //  (-X, Z, Y)
            Self::NXNZ, // (-X, Y, -Z)
            Self::NXNY, // (-X, -Z, -Y)

            Self::YX, // (Y, Z, X)
            Self::YZ, // (Y, -X, Z)
            Self::YNX, // (Y, -Z, -X)
            Self::YNZ, // (Y, X, -Z)

            Self::NYX, //  (-Y, -Z, X)
            Self::NYZ, //  (-Y, X, Z)
            Self::NYNX, // (-Y, Z, -X)
            Self::NYNZ, // (-Y, -X, -Z)

            Self::ZX, //  (Z, -Y, X)
            Self::ZY, //  (Z, X, Y)
            Self::ZNX, // (Z, Y, -X)
            Self::ZNY, // (Z, -X, -Y)

            Self::NZX, //  (-Z, Y, X)
            Self::NZY, //  (-Z, -X, Y)
            Self::NZNX, // (-Z, -Y, -X)
            Self::NZNY, // (-Z, X, -Y)
        ]
    }

    fn reorient(&self, point: &Point) -> Point {
        let x = point.x;
        let y = point.y;
        let z = point.z;
        match self {
            Orientation::XZ => {Point::new(x, y, z)}
            Orientation::XY => {Point::new(x, -z, y)}
            Orientation::XNZ => {Point::new(x, -y, -z)}
            Orientation::XNY => {Point::new(x, z, -y)}

            Orientation::NXZ => {Point::new(-x, -y, z)}
            Orientation::NXY => {Point::new(-x, z, y)}
            Orientation::NXNZ => {Point::new(-x, y, -z)}
            Orientation::NXNY => {Point::new(-x, -z, -y)}

            Orientation::YX => {Point::new(y, z, x)}
            Orientation::YZ => {Point::new(y, -x, z)}
            Orientation::YNX => {Point::new(y, -z, -x)}
            Orientation::YNZ => {Point::new(y, x, -z)}

            Orientation::NYX => {Point::new(-y, -z, x)}
            Orientation::NYZ => {Point::new(-y, x, z)}
            Orientation::NYNX => {Point::new(-y, z, -x)}
            Orientation::NYNZ => {Point::new(-y, -x, -z)}

            Orientation::ZX => {Point::new(z, -y, x)}
            Orientation::ZY => {Point::new(z, x, y)}
            Orientation::ZNX => {Point::new(z, y, -x)}
            Orientation::ZNY => {Point::new(z, -x, -y)}

            Orientation::NZX => {Point::new(-z, y, x)}
            Orientation::NZY => {Point::new(-z, -x, y)}
            Orientation::NZNX => {Point::new(-z, -y, -x)}
            Orientation::NZNY => {Point::new(-z, x, -y)}
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x, y, z
        }
    }

    fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
        (p1.x - p2.x).abs() +
            (p1.y - p2.y).abs() +
            (p1.z - p2.z).abs()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

fn find_translation_and_orientation(s1: &Vec<Point>, s2: &Vec<Point>) -> Option<(Point, Orientation)> {

    for s1_reference in s1 {
        let mut s1_set = HashSet::new();
        for s1_point in s1 {
            if s1_reference == s1_point { continue; }
            s1_set.insert(*s1_reference - *s1_point);
        }

        for orientation in Orientation::all_variants() {
            for s2_reference in s2 {
                let mut count = 1;

                for s2_point in s2 {
                    if s2_reference == s2_point { continue; }
                    let point = orientation.reorient(s2_reference) - orientation.reorient(s2_point);
                    if s1_set.contains(&point) {
                        count += 1;
                    }
                }

                if count >= 12 {
                    return Some((
                        *s1_reference - orientation.reorient(s2_reference),
                        orientation.clone()
                    ));
                }
            }
        }
    }
    None
}

fn find_translation_and_orientation_efficient(s1: &Vec<Point>, s2: &Vec<Point>) -> Option<(Point, Orientation)> {
    for orientation in Orientation::all_variants() {
        let s2_oriented = s2.iter().map(|x| orientation.reorient(x)).collect::<Vec<Point>>();
        let mut translation_counts = HashMap::new();

        for s1_point in s1 {
            for s2_point in &s2_oriented {
                let translation = s1_point.sub(*s2_point);
                *translation_counts.entry(translation).or_insert(1) += 1;
            }
        }

        for (translation, count) in translation_counts {
            if count >= 12 {
                // let transform = Box::new(move |p: &Point3D| rotation(p).add(&translation));
                return Some((translation, orientation.clone()));
            }
        }
    }

    None
}

fn translate_relative_to(node: i32, graph: &HashMap<i32, Vec<(i32, Point, Orientation)>>, viz: &mut HashSet<i32>, scanners: &Vec<Vec<Point>>) -> Vec<Point> {
    viz.insert(node);

    let mut result = vec![];
    for (neigh, n_p, n_o) in graph.get(&node).cloned().unwrap_or_default() {
        if viz.contains(&neigh) { continue }
        result.append(
            &mut translate_relative_to(neigh, graph, viz, scanners)
                .iter()
                .map(|p| {
                    n_p + n_o.reorient(p)
                })
                .collect()
        );
    }

    result.append(&mut scanners[node as usize].clone());
    result
}

fn translate_origins_relative_to(node: i32, graph: &HashMap<i32, Vec<(i32, Point, Orientation)>>, viz: &mut HashSet<i32>) -> Vec<Point> {
    viz.insert(node);

    let mut result = vec![];
    for (neigh, n_p, n_o) in graph.get(&node).cloned().unwrap_or_default() {
        if viz.contains(&neigh) { continue }
        result.append(
            &mut translate_origins_relative_to(neigh, graph, viz)
                .iter()
                .map(|p| {
                    n_p + n_o.reorient(p)
                })
                .collect()
        );
    }

    result.push(Point::new(0, 0, 0));
    result
}

fn get_graph(scanners: &Vec<Vec<Point>>) -> HashMap<i32, Vec<(i32, Point, Orientation)>> {
    let mut graph = HashMap::<i32, Vec<(i32, Point, Orientation)>>::new();

    for (i, scanner1) in scanners.iter().enumerate() {
        for (j, scanner2) in scanners.iter().enumerate() {
            if i == j { continue; }
            if let Some((p, t)) = find_translation_and_orientation_efficient(scanner1, scanner2) {
                graph
                    .entry(i as i32)
                    .and_modify(|v| v.push((j as i32, p, t)))
                    .or_insert(vec![(j as i32, p, t)]);
            }
        }
    }

    graph
}

fn part1(graph: &HashMap<i32, Vec<(i32, Point, Orientation)>>, scanners: &Vec<Vec<Point>>) -> usize {
    let points = translate_relative_to(0, graph, &mut HashSet::new(), &scanners);
    let mut hset = HashSet::new();
    for p in points {
        hset.insert(p);
    }

    hset.len()
}

fn part2(graph: &HashMap<i32, Vec<(i32, Point, Orientation)>>) -> i32 {
    let origins = translate_origins_relative_to(0, graph, &mut HashSet::new());

    let mut mx = 0;
    for p1 in &origins {
        for p2 in &origins {
            let distance = Point::manhattan_distance(p1, p2);
            if distance > mx {
                mx = distance;
            }
        }
    }

    mx
}

fn parse(path: &Path) -> Result<Vec<Vec<Point>>> {
    let mut v = vec![];
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(_) = lines.next() {
        let mut scanner = vec![];
        loop {
            let line = lines.next().transpose()?;

            if line.is_none() {
                break;
            }

            let line = line.unwrap();
            if line.is_empty() {
                break;
            }

            let (x, y, z) = scan_fmt::scan_fmt!(&line, "{},{},{}", i32, i32, i32)?;
            scanner.push(Point::new(x, y, z));
        }
        v.push(scanner)
    }

    Ok(v)
}

fn main() -> Result<()> {
    let scanners = parse(&Path::new("input.txt"))?;
    let graph = get_graph(&scanners);
    let p1 = part1(&graph, &scanners);
    let p2 = part2(&graph);
    println!("Part 1 {}", p1);
    println!("Part 2 {}", p2);
    Ok(())
}
