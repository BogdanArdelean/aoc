use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::Path;
fn parse(path: &Path) -> Result<(Position, Position, HashSet<Position>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let mut start = Position::default();
    let mut finish = Position::default();
    let mut walls = HashSet::new();

    for (x, row) in grid.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            let x = x as i64;
            let y = y as i64;
            match c {
                '#' => {
                    walls.insert(Position { x, y });
                }
                'S' => start = Position { x, y },
                'E' => finish = Position { x, y },
                _ => {}
            }
        }
    }

    Ok((start, finish, walls))
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
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
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
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

fn min_cost(source: Position, walls: &HashSet<Position>) -> HashMap<Position, i64> {
    let mut mc = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((source, 0));
    mc.insert(source, 0);
    while let Some((p, cost)) = q.pop_front() {
        for d in Direction::all() {
            let np = p + d;
            let nc = cost + 1;
            if walls.contains(&np) || mc.contains_key(&np) {
                continue;
            }
            q.push_back((np, nc));
            mc.insert(np, nc);
        }
    }
    mc
}

fn cheat(
    blinks_rem: i64,
    curr: Position,
    start_cost: i64,
    cost: i64,
    min_diff: i64,
    from_finish: &HashMap<Position, i64>,
) -> i64 {
    let mut viz = HashSet::new();
    let mut q = VecDeque::new();

    q.push_back(curr);
    viz.insert(curr);
    let mut count = 0;
    for i in 1..=blinks_rem {
        let mut q_aux = VecDeque::new();

        while let Some(p) = q.pop_front() {
            for d in Direction::all() {
                let np = p + d;
                if !viz.contains(&np) {
                    q_aux.push_back(np);
                    viz.insert(np);
                }
            }
        }

        for np in &q_aux {
            if let Some(cf) = from_finish.get(np) {
                let nc = start_cost + *cf + i;
                if cost - nc >= min_diff {
                    count += 1;
                }
            }
        }

        q = q_aux;
    }

    count
}

fn part1(start: Position, finish: Position, walls: &HashSet<Position>) -> i64 {
    let min_cost_start = min_cost(start, walls);
    let min_cost_finish = min_cost(finish, walls);
    let min_cost_start_finish = *min_cost_start.get(&finish).unwrap();
    min_cost_start
        .iter()
        .map(|(p, c)| cheat(2, *p, *c, min_cost_start_finish, 100, &min_cost_finish))
        .sum()
}

fn part2(start: Position, finish: Position, walls: &HashSet<Position>) -> i64 {
    let min_cost_start = min_cost(start, walls);
    let min_cost_finish = min_cost(finish, walls);
    let min_cost_start_finish = *min_cost_start.get(&finish).unwrap();
    min_cost_start
        .iter()
        .map(|(p, c)| cheat(20, *p, *c, min_cost_start_finish, 100, &min_cost_finish))
        .sum()
}

fn main() -> Result<()> {
    let (start, finish, walls) = parse(&Path::new("input.txt"))?;
    println!("Part 1: {}", part1(start, finish, &walls));
    println!("Part 2: {}", part2(start, finish, &walls));
    Ok(())
}
