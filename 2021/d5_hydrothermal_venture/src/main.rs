use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }
}

#[derive(Debug, Clone)]
struct LineSegment {
    p1: Point,
    p2: Point
}

impl LineSegment {
    fn new(p1: Point, p2: Point) -> Self {
        Self {
            p1,
            p2
        }
    }

    fn is_horizontal(&self) -> bool {
        return self.p1.x == self.p2.x
    }

    fn is_vertical(&self) -> bool {
        return self.p1.y == self.p2.y
    }

    fn get_y_min_max(&self) -> (i32, i32) {
        if self.p1.y > self.p2.y {
            (self.p2.y, self.p1.y)
        } else {
            (self.p1.y, self.p2.y)
        }
    }

    fn get_x_min_max(&self) -> (i32, i32) {
        if self.p1.x > self.p2.x {
            (self.p2.x, self.p1.x)
        } else {
            (self.p1.x, self.p2.x)
        }
    }
}

fn parse(path: &Path) -> Result<Vec<LineSegment>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut segments = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let (x1, y1, x2, y2) = scan_fmt::scan_fmt!(&line, "{},{} -> {},{}", i32, i32, i32 ,i32)?;
        segments.push(
            LineSegment::new(Point::new(x1, y1), Point::new(x2, y2))
        );
    }

    Ok(segments)
}

fn count_dangerous(segments: &Vec<LineSegment>) -> i32 {
    let mut dangerous_cells = 0;
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];

    for segment in segments {
        if segment.is_horizontal() {
            let x = segment.p1.x as usize;
            let (y_min, y_max) = segment.get_y_min_max();
            for y in y_min..=y_max {
                let y = y as usize;
                grid[x][y] += 1;
            }
        } else if segment.is_vertical() {
            let y = segment.p1.y as usize;
            let (x_min, x_max) = segment.get_x_min_max();
            for x in x_min..=x_max {
                let x = x as usize;
                grid[x][y] += 1;
            }
        } else {
            let dx = if segment.p2.x > segment.p1.x {
                1
            } else {
                -1
            };
            let dy = if segment.p2.y > segment.p1.y {
                1
            } else {
                -1
            };
            let mut x = segment.p1.x;
            let mut y = segment.p1.y;
            while x != segment.p2.x && y != segment.p2.y {
                grid[x as usize][y as usize] += 1;
                x += dx;
                y += dy;
            }
            grid[segment.p2.x as usize][segment.p2.y as usize] += 1;
        }
    }

    for row in grid {
        for col in row {
            if col > 1 {
                dangerous_cells += 1;
            }
        }
    }

    dangerous_cells
}

fn main() -> Result<()> {
    let segments = parse(&Path::new("input.txt"))?;
    let part_1 = count_dangerous(&segments);
    println!("Part 2: {}", part_1);
    Ok(())
}
