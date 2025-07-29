use anyhow::Result;
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

impl PartialEq for Robot {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

impl Eq for Robot {}

impl Hash for Robot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.p.hash(state);
    }
}

impl Robot {
    fn move_steps(&self, steps: i64, bounds: (i64, i64)) -> Robot {
        let p_x = (self.p.0 + self.v.0 * steps).rem_euclid(bounds.0);
        let p_y = (self.p.1 + self.v.1 * steps).rem_euclid(bounds.1);

        Robot {
            p: (p_x, p_y),
            v: self.v,
        }
    }

    fn get_quadrant(&self, (max_x, max_y): (i64, i64)) -> i64 {
        ((self.p.0 > max_x / 2) as i64) << 1 | (self.p.1 > max_y / 2) as i64
    }

    fn x_y(x: i64, y: i64) -> Self {
        Self {
            p: (x, y),
            v: (0, 0),
        }
    }
}

fn parse(path: &Path) -> Result<Vec<Robot>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut res = vec![];
    for line in reader.lines() {
        let line = line?;
        let (p_x, p_y, v_x, v_y) = scan_fmt!(&line, "p={},{} v={},{}", i64, i64, i64, i64)?;
        res.push(Robot {
            p: (p_x, p_y),
            v: (v_x, v_y),
        });
    }

    Ok(res)
}

fn safety_score(robots: &[Robot], (max_x, max_y): (i64, i64)) -> i64 {
    robots
        .iter()
        .filter(|r| r.p.0 != max_x / 2 && r.p.1 != max_y / 2)
        .map(|r| r.get_quadrant((max_x, max_y)) as usize)
        .fold([0i64; 4], |mut acc, elm| {
            acc[elm] += 1;
            acc
        })
        .iter()
        .product()
}

fn part1(robots: &[Robot], steps: i64, (max_x, max_y): (i64, i64)) -> i64 {
    safety_score(
        &robots
            .iter()
            .map(|r| r.move_steps(steps, (max_x, max_y)))
            .collect::<Vec<_>>(),
        (max_x, max_y),
    )
}

fn print_robots(robots: &HashSet<Robot>, (max_x, max_y): (i64, i64)) {
    for y in 0..max_y {
        for x in 0..max_x {
            if robots.contains(&Robot::x_y(x, y)) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn part2(mut robots: Vec<Robot>, (max_x, max_y): (i64, i64)) -> i64 {
    let mut min_score = i64::MAX;
    let mut step = 0;
    let mut config = vec![];

    for steps in 0..=(max_x * max_y) {
        let safety_score = safety_score(&robots, (max_x, max_y));
        if min_score > safety_score {
            min_score = safety_score;
            step = steps;
            config = robots.clone();
        }

        robots = robots
            .iter()
            .map(|r| r.move_steps(1, (max_x, max_y)))
            .collect();
    }

    print_robots(&config.into_iter().collect(), (max_x, max_y));
    step
}

fn main() -> Result<()> {
    let robots = parse(Path::new("input.txt"))?;
    println!("Part 1 {}", part1(&robots, 100, (101, 103)));

    println!("Part 2 {}", part2(robots, (101, 103)));
    Ok(())
}
