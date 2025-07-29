use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{AddAssign, self};
use std::path::Path;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use scan_fmt::scan_fmt;

type PondMap = Vec<Vec<i128>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    N,
    W,
    S,
    E
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    dir: Direction,
    len: i128,
    color: u32
}

impl Instruction {
    fn plus_length(&self, dx: i128) -> Self {
        Self {
            dir: self.dir,
            len: self.len + dx,
            color: 0
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i128,
    y: i128
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

fn area(points: &[Point]) -> i128 {
    let mut area = 0;
    
    for (a, b) in points.iter().tuple_windows() {
        area += (a.x * b.y) - (b.x * a.y);
    }
    
    (area / 2).abs()
}

fn determine_plan_size_and_starting_point(instructions: &[Instruction]) -> (i128, i128, Point) {
    let mut min_width = std::i128::MAX;
    let mut max_width = std::i128::MIN;
    let mut min_height = std::i128::MAX;
    let mut max_height = std::i128::MIN;

    let mut curr_width = 0;
    let mut curr_height = 0;
    for instr in instructions {
        match instr.dir {
            Direction::N => { curr_height -= instr.len; min_height = min_height.min(curr_height); },
            Direction::W => { curr_width -= instr.len; min_width = min_width.min(curr_width); },
            Direction::S => { curr_height += instr.len; max_height = max_height.max(curr_height); },
            Direction::E => { curr_width += instr.len; max_width = max_width.max(curr_width); },
        }
    }
    let height = max_height - min_height + 1;
    let width = max_width - min_width + 1;
    let start = Point { x: min_height.abs(), y: min_width.abs() };
    
    println!("{}, {}, {:?}", height, width, start);
    
    (height, width, start)
}

fn generate_perimeter(height: i128, width: i128, mut start: Point, instructions: &[Instruction]) -> (PondMap, i128) {
    let mut pond = PondMap::new();
    let mut perimeter_size = 0;

    pond.resize_with(height as usize, || {
        let mut col = Vec::<i128>::new();
        col.resize(width as usize, 0);
        col
    });

    pond[start.x as usize][start.y as usize] = 1;
    
    for instr in instructions {
        for _ in 0..instr.len {
            match instr.dir {
                Direction::N => start.x -= 1,
                Direction::W => start.y -= 1,
                Direction::S => start.x += 1,
                Direction::E => start.y += 1,
            }
            perimeter_size += 1;
            pond[start.x as usize][start.y as usize] = 1;
        }
    }

    for row in &pond {
        for col in row {
            print!("{}", col);
        }
        println!();
    }

    (pond, perimeter_size)
}

fn calculate_area(pond: &PondMap) -> i128 {
    let mut area = 0;
   
    let mut out = true;
    for i in 1..pond.len()-1 {
        for j in 0..pond[i].len() {
            let val = pond[i][j];
            if val == 0 && !out {
                area += 1;
            }
            if val == 1 && pond[i-1][j] == 1 {
                out = !out;
            }
        }
    }

    area
}

fn parse(path: &Path) -> Result<Vec<Instruction>> {
    let mut result = Vec::<_>::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let (chr, ln, _) = scan_fmt!(&line, "{} {} {}", char, i128, String)?;
        let dir = match chr {
            'U' => Direction::N,
            'D' => Direction::S,
            'L' => Direction::W,
            'R' => Direction::E,
            _ => panic!("At the disco!")
        };
        result.push(Instruction { dir: dir, len: ln, color: 0 });
    }
    anyhow::Ok(result)
}

fn get_dir_point(ins: &Instruction) -> Point {
    match ins.dir {
        Direction::N => Point { x: -ins.len, y: 0 },
        Direction::W => Point { x: 0, y: -ins.len},
        Direction::S => Point { x: ins.len, y: 0 },
        Direction::E => Point { x: 0, y: ins.len },
    }
}

fn get_points(instructions: &[Instruction]) -> Vec<Point> {
    let mut points = Vec::<_>::new();
    
    let mut start_point = Point { x: 0, y: 0 };
    let mut last_exterior = true;
    points.push(start_point);
    for (idx, ins) in instructions.iter().enumerate() {
        let last: Option<Direction> = instructions.get(idx + 1).and_then(|instr| Some(instr.dir));
        
        let exterior = match (ins.dir, last){
            (Direction::E, Some(Direction::S)) => true,
            (Direction::S, Some(Direction::W)) => true,
            (Direction::W, Some(Direction::N)) => true,
            (Direction::N, Some(Direction::E)) => true,
            _ => false
        };

        let mut addition = 0;
        if exterior {
            addition += 1;
        }
        if !last_exterior {
            addition -= 1;
        }

        start_point += get_dir_point(&ins.plus_length(addition));
        points.push(start_point);

        last_exterior = exterior;
    }
    points.push(points[0]);
    points
}

// 44644464596918

fn parse_part_2(path: &Path) -> Result<Vec<Instruction>> {
    let mut result = Vec::<_>::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let (_, _, hex) = scan_fmt!(&line, "{} {} (#{})", char, i128, String)?;
        
        let s = "0x".to_string() + &hex[0..5].to_string();
        let ln = scan_fmt!(&s, "{x}", [hex i128])?;
        
        let s = "0x".to_string() + &hex[5..6].to_string();
        let chr = scan_fmt!(&s, "{x}", [hex i128])?;

        let dir = match chr {
            0 => Direction::E,
            1 => Direction::S,
            2 => Direction::W,
            3 => Direction::N,
            _ => panic!("At the disco!")
        };

        result.push(Instruction { dir: dir, len: ln, color: 0 });
    }
    anyhow::Ok(result)
}

fn main() -> Result<()> {
    let instructions = parse_part_2(Path::new("input.txt"))?;
    let points = get_points(&instructions);
    let part_1 = area(&points);

    println!("Part 2: {}", part_1);
    anyhow::Ok(())
}
