use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{anyhow, Ok, Result};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
type Contraption = Vec<Vec<char>>;
type Step = (i32, i32, Direction);
type VisitedSet = HashSet<Step>;

fn next_coordinates(x: i32, y: i32, dir: Direction) -> Step {
    match dir {
        Direction::Up => (x - 1, y, dir),
        Direction::Down => (x + 1, y, dir),
        Direction::Left => (x, y - 1, dir),
        Direction::Right => (x, y + 1, dir),
    }
}

fn get_next_directions(cell: char, (x, y, dir): Step) -> Vec<Step> {
    let mut result = Vec::<_>::new();
    match cell {
        '.' => {
            result.push(next_coordinates(x, y, dir));
        },
        '|' => {
            if dir == Direction::Left || dir == Direction::Right {
                result.push(next_coordinates(x, y, Direction::Up));
                result.push(next_coordinates(x, y, Direction::Down));
            } else {
                result.push(next_coordinates(x, y, dir));
            }
        },
        '-' => {
            if dir == Direction::Up || dir == Direction::Down {
                result.push(next_coordinates(x, y, Direction::Left));
                result.push(next_coordinates(x, y, Direction::Right));
            } else {
                result.push(next_coordinates(x, y, dir));
            }
        },
        '/' => {
            let nx = match dir {
                Direction::Up => next_coordinates(x, y, Direction::Right),
                Direction::Down => next_coordinates(x, y, Direction::Left),
                Direction::Left => next_coordinates(x, y, Direction::Down),
                Direction::Right => next_coordinates(x, y, Direction::Up),
            };
            result.push(nx);
        },
        '\\' => {
            let nx = match dir {
                Direction::Up => next_coordinates(x, y, Direction::Left),
                Direction::Down => next_coordinates(x, y, Direction::Right),
                Direction::Left => next_coordinates(x, y, Direction::Up),
                Direction::Right => next_coordinates(x, y, Direction::Down),
            };
            result.push(nx);
        }
        _ => {}
    }
    result
}

fn get_energized_tiles(start: Step, contraption: &Contraption) -> usize {
    let mut visisted = VisitedSet::new();
    let mut q = VecDeque::<Step>::new();
    q.push_back(start);

    while !q.is_empty() {
        let (x, y, dir) = q.pop_front().unwrap();
        visisted.insert((x, y, dir));
        let cell = contraption[x as usize][y as usize];
        let next_points = get_next_directions(cell, (x, y, dir));
        for (x_n, y_n, dir_n) in next_points {
            if x_n < 0 || x_n >= contraption.len() as i32 || y_n < 0 || y_n >= contraption[0].len() as i32 {
                continue;
            }
            if visisted.contains(&(x_n, y_n, dir_n)) {
                continue;
            }

            q.push_back((x_n, y_n, dir_n));
        }
    }

    visisted.iter().map(|(x, y, _)| (*x, *y)).collect::<HashSet<(i32, i32)>>().len()
}

fn parse(path: &Path) -> Result<Contraption> {
    let mut contraption = Contraption::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        contraption.push(line.chars().collect());
    }

    Ok(contraption)
}

fn get_maximum_energized_tiles(contraption: &Contraption) -> usize {
    let mut max = 0;
    for i in (0..contraption.len()) {
        max = max.max(get_energized_tiles((i as i32, 0, Direction::Right), contraption));
        max = max.max(get_energized_tiles((i as i32, contraption[0].len() as i32 - 1 , Direction::Left), contraption));
    }

    for j in (0..contraption[0].len()) {
        max = max.max(get_energized_tiles((0, j as i32, Direction::Down), contraption));
        max = max.max(get_energized_tiles((contraption.len() as i32 - 1, j as i32 , Direction::Up), contraption));
    }

    max
}

fn main() -> Result<()> {
    let contraption = parse(Path::new("input.txt"))?;

    let part_1 = get_energized_tiles((0, 0, Direction::Right), &contraption);
    let part_2 = get_maximum_energized_tiles(&contraption);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    Ok(())
}
