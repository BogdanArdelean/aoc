use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance_sq(&self, other: Point) -> i64 {
        (self.x - other.x).pow(2) +
            (self.y - other.y).pow(2) +
            (self.z - other.z).pow(2)
    }
}

fn parse(path: &Path) -> Vec<Point> {
    let reader = BufReader::new(File::open(path).unwrap());
    reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let v: Vec<_> = l
                .split(',')
                .map(|x| x.parse().unwrap()).collect();
            Point {
                x: v[0],
                y: v[1],
                z: v[2]
            }
        })
        .collect()
}

fn compute_distances(points: &Vec<Point>) -> Vec<(i64, usize, usize)> {
    let mut distances = vec![];

    for i in 0..points.len() - 1 {
        for j in i+1..points.len() {
            distances.push((
                points[i].distance_sq(points[j]),
                i,
                j
            ))
        }
    }
    distances.sort_by_key(|&(d, _, _)| d);
    distances
}

fn find(cc: &mut Vec<usize>, a: usize) -> usize {
    if cc[a] == a {
        return a;
    }

    let p = find(cc, cc[a]);
    cc[a] = p;

    p
}

fn connect(cc: &mut Vec<usize>, a: usize, b: usize) {
    let a = find(cc, a);
    let b = find(cc, b);
    cc[b] = cc[a];
}

fn part1(points: &Vec<Point>, distances: &Vec<(i64, usize, usize)>, c: usize) -> usize {
    let mut connected_components = vec![0; points.len()];

    // everyone is their parent
    for i in 0..connected_components.len() {
        connected_components[i] = i;
    }

    for i in 0..c {
        let d = distances[i];
        connect(&mut connected_components, d.1, d.2);
    }

    let mut hm = HashMap::new();
    for i in 0..connected_components.len() {
        let k = find(&mut connected_components, i);
        *hm.entry(k).or_insert(0usize) += 1;
    }

    let mut v: Vec<usize> = hm.into_values().collect();
    v.sort();
    v.reverse();
    v[0] * v[1] * v[2]
}

fn part2(points: &Vec<Point>, distances: &Vec<(i64, usize, usize)>) -> i64 {
    let mut connected_components = vec![0; points.len()];

    // everyone is their parent
    for i in 0..connected_components.len() {
        connected_components[i] = i;
    }

    for &d in distances {
        connect(&mut connected_components, d.1, d.2);

        for i in 0..connected_components.len() {
            find(&mut connected_components, i);
        }

        if connected_components.iter().all(|x| *x == connected_components[0]) {
            let a = points[d.1];
            let b = points[d.2];
            return a.x * b.x;
        }
    }

    panic!("Can't be here!")
}

fn part2_opt(points: &Vec<Point>, distances: &Vec<(i64, usize, usize)>) -> i64 {
    fn connect(cc: &mut Vec<usize>, cs: &mut Vec<usize>, a: usize, b: usize, target: usize) -> bool {
        let a = find(cc, a);
        let b = find(cc, b);

        if cc[a] != cc[b] {
            cs[a] += cs[b];
        }

        cc[b] = cc[a];

        cs[a] == target
    }
    
    let p_len = points.len();
    let mut connected_components = vec![0; p_len];
    let mut connected_sizes = vec![1; p_len];

    // everyone is their parent
    for i in 0..connected_components.len() {
        connected_components[i] = i;
    }

    for &d in distances {
        if connect(&mut connected_components, &mut connected_sizes, d.1, d.2, p_len) {
            let a = points[d.1];
            let b = points[d.2];
            return a.x * b.x;
        }
    }

    panic!("Can't be here!")
}

fn main() {
    let points = parse(&Path::new("input.txt"));
    let distances = compute_distances(&points);

    let p1 = part1(&points, &distances, 1000);
    println!("part 1: {}", p1);

    let p2 = part2_opt(&points, &distances);
    println!("part 2: {}", p2);
}
