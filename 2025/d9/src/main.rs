use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64
}

fn area(p1: Point, p2: Point) -> i64 {
    ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1)
}

fn parse(path: &Path) -> Vec<Point> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .flatten()
        .map(|x| {
            let v: Vec<i64> = x
                .split(",")
                .map(|e| e.parse().unwrap())
                .collect();
            Point {
                x: v[0],
                y: v[1]
            }
        })
        .collect()
}

fn part1(points: &Vec<Point>) -> i64 {
    let mut max_area = 0;

    for i in 0..points.len() - 1 {
        for j in i+1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            max_area = max_area.max(area(p1, p2));
        }
    }

    max_area
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    p1: Point,
    p2: Point
}

impl Segment {
    fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn direction(&self) -> Direction {
        match (self.is_horizontal(), self.p1.x > self.p2.x, self.p1.y > self.p2.y) {
            (true, true, _)   => Direction::Left,
            (true, false, _)  => Direction::Right,
            (false, _, true)  => Direction::Down,
            (false, _, false) => Direction::Up
        }
    }

    fn contains_x(&self, x: i64) -> bool {
        self.p1.x.min(self.p2.x) <= x && self.p1.x.max(self.p2.x) >= x
    }

    fn contains_y(&self, y: i64) -> bool {
        self.p1.y.min(self.p2.y) <= y && self.p1.y.max(self.p2.y) >= y
    }
}

#[derive(Debug, Clone)]
struct Shape {
    segments: Vec<Segment>,
    by_p1: HashMap<Point, Segment>,
    by_p2: HashMap<Point, Segment>,
    hss: BTreeMap<i64, Vec<Segment>>, // by y
    vss: BTreeMap<i64, Vec<Segment>>, // by x
    outside_on: Direction,
}

impl Shape {
    fn new(points: &Vec<Point>) -> Self {
        let mut segments = vec![];
        let mut p_slow = points[0];

        let mut hss: BTreeMap<i64, Vec<Segment>> = BTreeMap::new();
        let mut vss: BTreeMap<i64, Vec<Segment>> = BTreeMap::new();
        let mut by_p1 = HashMap::new();
        let mut by_p2 = HashMap::new();
        for i in 1..points.len() {
            let p_fast = points[i];
            let segment = Segment::new(p_slow, p_fast);
            segments.push(segment);
            if segment.is_horizontal() {
                hss.entry(p_fast.y).or_default().push(segment);
            } else {
                vss.entry(p_fast.x).or_default().push(segment);
            }
            assert!(by_p1.insert(p_slow, segment).is_none());
            assert!(by_p2.insert(p_fast, segment).is_none());

            p_slow = p_fast;
        }

        let p_fast = points[0];
        let segment = Segment::new(p_slow, p_fast);
        segments.push(segment);

        if segment.is_horizontal() {
            hss.entry(p_fast.y).or_default().push(segment);
        } else {
            vss.entry(p_fast.x).or_default().push(segment);
        }

        assert!(by_p1.insert(p_slow, segment).is_none());
        assert!(by_p2.insert(p_fast, segment).is_none());

        let outside_on = vss.first_key_value().unwrap().1[0].direction();
        Self {
            segments,
            by_p1,
            by_p2,
            hss,
            vss,
            outside_on,
        }
    }

    fn get_next(map: &BTreeMap<i64, Vec<Segment>>, k1: i64, k2: i64, other: i64, compare_x: bool) -> Option<Segment> {
        for (_, segs) in map.range(k1..k2) {
            for s in segs {
                if compare_x && s.contains_x(other) {
                    return Some(*s);
                } else if !compare_x && s.contains_y(other) {
                    return Some(*s);
                }
            }
        }
        None
    }

    fn contains_horizontal(&self, segment: Segment) -> bool {
        let mut x= if segment.direction() == Direction::Right {
            segment.p1.x
        } else {
            segment.p2.x + 1
        };

        let (x_target, x_target_t) = if segment.direction() == Direction::Right {
            (segment.p2.x, segment.p2.x)
        } else {
            (segment.p1.x + 1, segment.p1.x)
        };

        let y = segment.p1.y;
        while let Some(seg) = Shape::get_next(&self.vss, x, x_target, y, false) {
            let (other_seg, (d1, d2)) = if seg.p1.y == y {
                let x = (self.by_p2[&seg.p1], (self.by_p2[&seg.p1].direction(), seg.direction()));
                assert!(x.1.0 != x.1.1);
                x
            } else if seg.p2.y == y {
                let x = (self.by_p1[&seg.p2], (seg.direction(), self.by_p1[&seg.p2].direction()));
                assert!(x.1.0 != x.1.1);
                x
            } else {
                (seg, (seg.direction(), seg.direction()))
            };

            let ok = match segment.direction() {
                Direction::Left => {
                    match (d1, d2) {
                        (Direction::Right, Direction::Up)   => true,
                        (Direction::Up, Direction::Left)    => true,
                        (Direction::Up, Direction::Right)   => true,
                        (Direction::Left, Direction::Up)    => true,
                        (Direction::Right, Direction::Down) => true,
                        (Direction::Down, Direction::Left)  => true,
                        _ => false,
                    }
                }
                Direction::Right => {
                    match (d1, d2) {
                        (Direction::Left, Direction::Down)  => true,
                        (Direction::Down, Direction::Right) => true,
                        (Direction::Up, Direction::Right)   => true,
                        (Direction::Right, Direction::Down) => true,
                        (Direction::Down, Direction::Left)  => true,
                        _ => false,
                    }
                },
                _ => panic!("nope...")
            };

            if !ok && !(seg.p1.x > x_target_t) {
                if let Some(seg2) = Shape::get_next(&self.vss, seg.p1.x + 1, x_target, y, false) {
                    if seg2.p1.x == seg.p1.x + 1 {
                        x = seg.p1.x + 1;
                        continue;
                    }
                }
                return false;
            }

            x = seg.p1.x + 1;
            if x >= x_target {
                return true;
            }
        }

        true
    }

    fn contains_vertical(&self, segment: Segment) -> bool {
        let mut y= if segment.direction() == Direction::Up {
            segment.p1.y
        } else {
            segment.p2.y + 1
        };

        let (y_target, y_target_t) = if segment.direction() == Direction::Up {
            (segment.p2.y, segment.p2.y)
        } else {
            (segment.p1.y + 1, segment.p1.y)
        };

        let x = segment.p1.x;
        while let Some(seg) = Shape::get_next(&self.hss, y, y_target, x, true) {
            let (other_seg, (d1, d2)) = if seg.p1.x == x {
                let x = (self.by_p2[&seg.p1], (self.by_p2[&seg.p1].direction(), seg.direction()));
                assert!(x.1.0 != x.1.1);
                x
            } else if seg.p2.x == x {
                let x = (self.by_p1[&seg.p2], (seg.direction(), self.by_p1[&seg.p2].direction()));
                assert!(x.1.0 != x.1.1);
                x
            } else {
                (seg, (seg.direction(), seg.direction()))
            };


            let ok = match segment.direction() {
                Direction::Up => {
                    match (d1, d2) {
                        (Direction::Down, Direction::Right) => true,
                        (Direction::Right, Direction::Up)   => true,
                        (Direction::Up, Direction::Right)   => true,
                        (Direction::Left, Direction::Up)    => true,
                        (Direction::Down, Direction::Left)  => true,
                        (Direction::Right, Direction::Down) => true,
                        _ => false,
                    }
                }
                Direction::Down => {
                    match (d1, d2) {
                        (Direction::Left, Direction::Down)  => true,
                        (Direction::Up, Direction::Left)    => true,
                        (Direction::Up, Direction::Right)   => true,
                        (Direction::Left, Direction::Up)    => true,
                        (Direction::Right, Direction::Down) => true,
                        (Direction::Down, Direction::Left)  => true,
                        _ => false,
                    }
                },
                _ => panic!("nope...")
            };
            if !ok && !(seg.p1.y > y_target_t) {
                if let Some(seg2) = Shape::get_next(&self.hss, seg.p1.y + 1, y_target, x, true) {
                    if seg2.p1.y == seg.p1.y + 1 {
                        y = seg.p1.y + 1;
                        continue;
                    }
                }
                return false;
            }

            y = seg.p1.y + 1;
            if y >= y_target {
                return true;
            }
        }

        true
    }

    fn contains(&self, segment: Segment) -> bool {
        match segment.direction() {
            Direction::Up =>    { self.contains_vertical(segment)   }
            Direction::Down =>  { self.contains_vertical(segment)   }
            Direction::Left =>  { self.contains_horizontal(segment) }
            Direction::Right => { self.contains_horizontal(segment) }
        }
    }
}

fn part2(points: &Vec<Point>) -> i64 {
    let shape = Shape::new(&points);

    let mut max_area = 0;

    for i in 0..points.len() - 1 {
        for j in i+1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let v = vec![
                Segment::new(p1, Point {x: p1.x, y: p2.y}),
                Segment::new(p1, Point {x: p2.x, y: p1.y}),
                Segment::new(p2, Point {x: p2.x, y: p1.y}),
                Segment::new(p2, Point {x: p1.x, y: p2.y})
            ];
            let a = area(p1, p2);
            
            let mut ok = true;
            for s in v {
                ok = ok && shape.contains(s);
            }
            if ok && a > max_area {
                max_area = a;
            }
        }
    }

    max_area
}


fn main() {
    let points = parse(Path::new("input.txt"));
    let p1 = part1(&points);
    println!("part 1: {}", p1);

    let p2 = part2(&points);
    println!("part 2: {}", p2);
}
