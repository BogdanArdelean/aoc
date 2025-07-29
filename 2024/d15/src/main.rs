use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::path::Path;

fn parse(path: &Path) -> Result<(Warehouse, Vec<Direction>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut grid = vec![];
    let mut lines = reader.lines();
    loop {
        let line = lines.next().unwrap()?;
        if line.is_empty() {
            break;
        }
        grid.push(line.chars().collect());
    }
    let w = Warehouse { grid };

    let mut d = vec![];
    for line in lines {
        let line = line?;
        d.extend(line.chars().map(|c| match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            d => panic!("Invalid direction {}", d),
        }));
    }

    Ok((w, d))
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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

trait WarehouseOperations {
    const TARGET: char;
    fn can_move(&self, p: Position, d: Direction) -> bool;

    fn move_object(&mut self, p: Position, d: Direction);

    fn grid(&self) -> &Vec<Vec<char>>;

    fn get_robot(&self) -> Position {
        let grid = self.grid();
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] == '@' {
                    return Position { x, y };
                }
            }
        }
        panic!("Can't find robot!");
    }

    fn count_gps(&self) -> usize {
        let grid = self.grid();
        let mut sum = 0;
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                if grid[x][y] == Self::TARGET {
                    sum += 100 * x + y;
                }
            }
        }
        sum
    }

    fn contains(&self, p: Position) -> bool {
        let grid = self.grid();
        p.x < grid.len() && p.y < grid[0].len()
    }
}

#[derive(Debug, Clone)]
struct Warehouse {
    grid: Vec<Vec<char>>,
}

impl Warehouse {
    fn to_double_warehouse(&self) -> DoubleWarehouse {
        let grid = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|&c| match c {
                        '#' => ['#', '#'].iter().copied(),
                        'O' => ['[', ']'].iter().copied(),
                        '@' => ['@', '.'].iter().copied(),
                        '.' => ['.', '.'].iter().copied(),
                        c => panic!("Invalid char {}", c),
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        DoubleWarehouse { grid }
    }
}

impl WarehouseOperations for Warehouse {
    const TARGET: char = 'O';

    fn can_move(&self, p: Position, d: Direction) -> bool {
        let np = p + d;
        assert!(self.contains(p));
        assert!(self.contains(np));

        match self.grid[np.x][np.y] {
            '#' => false,
            '.' => true,
            'O' => self.can_move(np, d),
            c => panic!("Can't move tile {}.", c),
        }
    }
    fn move_object(&mut self, p: Position, d: Direction) {
        let np = p + d;

        match self.grid[np.x][np.y] {
            '#' => return,
            '.' => self.grid[np.x][np.y] = self.grid[p.x][p.y],
            'O' => {
                self.move_object(np, d);
                self.grid[np.x][np.y] = self.grid[p.x][p.y];
            }
            c => panic!("Can't move tile {}.", c),
        }
        self.grid[p.x][p.y] = '.';
    }

    fn grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }
}

#[derive(Debug, Clone)]
struct DoubleWarehouse {
    grid: Vec<Vec<char>>,
}

impl WarehouseOperations for DoubleWarehouse {
    const TARGET: char = '[';

    fn can_move(&self, p: Position, d: Direction) -> bool {
        let np = p + d;
        assert!(self.contains(p));
        assert!(self.contains(np));

        match self.grid[np.x][np.y] {
            '#' => false,
            '.' => return true,
            '[' => {
                (d == Direction::Right || self.can_move(np, d))
                    && self.can_move(np + Direction::Right, d)
            }
            ']' => {
                (d == Direction::Left || self.can_move(np, d))
                    && self.can_move(np + Direction::Left, d)
            }
            c => panic!("Can't move tile {}", c),
        }
    }

    fn move_object(&mut self, p: Position, d: Direction) {
        let np = p + d;
        let c = self.grid[np.x][np.y];
        match (c, d) {
            ('#', _) => {
                return;
            }
            ('.', _) => self.grid[np.x][np.y] = self.grid[p.x][p.y],
            ('[', Direction::Right) | (']', Direction::Left) => {
                let adj = np + d;
                self.move_object(adj, d);
                self.grid[adj.x][adj.y] = self.grid[np.x][np.y];
                self.grid[np.x][np.y] = self.grid[p.x][p.y];
            }
            ('[', _) | (']', _) => {
                let adj = np
                    + if c == '[' {
                        Direction::Right
                    } else {
                        Direction::Left
                    };
                self.move_object(np, d);
                self.move_object(adj, d);

                self.grid[np.x][np.y] = self.grid[p.x][p.y];
                self.grid[adj.x][adj.y] = '.';
            }
            _ => panic!("Can't move tile {}", c),
        }

        self.grid[p.x][p.y] = '.';
    }

    fn grid(&self) -> &Vec<Vec<char>> {
        &self.grid
    }
}

fn simulate(mut warehouse: impl WarehouseOperations, d: Vec<Direction>) -> usize {
    let mut robot = warehouse.get_robot();
    for dir in d {
        if warehouse.can_move(robot, dir) {
            warehouse.move_object(robot, dir);
            robot = robot + dir;
        }
    }

    warehouse.count_gps()
}

fn main() -> Result<()> {
    let (w, d) = parse(Path::new("input.txt"))?;
    println!("Part 1 {}", simulate(w.clone(), d.clone()));

    println!("Part 2 {}", simulate(w.to_double_warehouse(), d.clone()));
    Ok(())
}
