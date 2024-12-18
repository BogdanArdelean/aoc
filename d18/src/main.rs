use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};
use std::path::Path;
use anyhow::Result;
use scan_fmt::scan_fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Position {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}
fn can_go(p: Position,
          width: i64,
          height: i64,
          set: &HashSet<Position>) -> bool {
    p.x >= 0 && p.y >= 0 && p.x <= width && p.y <= height && !set.contains(&p)
}

fn min_path(from: Position, to: Position, corrupted: HashSet<Position>) -> i64 {
    let mut viz = HashSet::<Position>::new();
    let mut q = VecDeque::new();
    q.push_back((from, 0));
    viz.insert(from);
    while let Some((p, cost)) = q.pop_front() {
        if p == to {
            return cost;
        }

        for d in Direction::all() {
            let np = p + d;
            if !can_go(np, to.x, to.y, &corrupted) || viz.contains(&np) {
                continue;
            }
            q.push_back((np, cost + 1));
            viz.insert(np);
        }
    }
    -1
}

fn parse(path: &Path) -> Result<Vec<Position>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut res = vec![];

    for line in reader.lines() {
        let line = line?;
        let (x, y) = scan_fmt!(&line, "{},{}", i64, i64)?;
        res.push(
            Position {x, y}
        );
    }

    Ok(res)
}

fn part1(corrupted: &Vec<Position>) -> i64 {
    let from = Position {x:0, y:0};
    let to = Position {x:70, y:70};

    min_path(from, to, corrupted.iter().cloned().take(1024).collect())
}

fn part2(corrupted: &Vec<Position>) {
    let from = Position {x:0, y:0};
    let to = Position {x:70, y:70};

    let mut l = 1023;
    let mut r = corrupted.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        let found = min_path(from, to, corrupted.iter().cloned().take(m + 1).collect());
        if found == -1 {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }
    println!("Part 2: {}, {}", corrupted[l].x, corrupted[l].y);
}

fn main() -> Result<()> {
    let corrupted = parse(Path::new("input.txt"))?;
    println!("Part 1: {}", part1(&corrupted));
    part2(&corrupted);
    Ok(())
}
