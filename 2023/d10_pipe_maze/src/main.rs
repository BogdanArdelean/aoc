use std::collections::{HashMap, VecDeque};
use std::default;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use anyhow::{Ok, Result, anyhow};
use itertools::Itertools;

const RIGHT: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (0, -1);
const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
static DIRECTIONS: [&(i32, i32); 4] = [&UP, &DOWN, &LEFT, &RIGHT];
type PosDir = ((i32, i32), (i32, i32));

fn get_next_neighbour(maze: &Vec<Vec<char>>, (pos_x, pos_y): &(i32, i32), direction: &(i32, i32)) -> Option<((i32, i32), (i32, i32))> {
    let x = *pos_x + direction.0;
    let y = *pos_y + direction.1;
    let chr = maze.get(x as usize).and_then(|x| x.get(y as usize));
    if chr.is_none() {
        return None;
    }
    
    let chr = *chr.unwrap();
    let new_direction = match *direction {
        UP => match chr {
            '|' => *direction,
            'F' => RIGHT,
            '7' => LEFT,
            _ => return None
        },
        DOWN => match chr {
            '|' => *direction,
            'L' => RIGHT,
            'J' => LEFT,
            _ => return None
        },
        RIGHT => match chr {
            '-' => *direction,
            '7' => DOWN,
            'J' => UP,
            _ => return None,
        },
        LEFT => match chr {
            '-' => *direction,
            'F' => DOWN,
            'L' => UP,
            _ => return None
        },
        _ => return None
    };

    Some(((x, y), new_direction))
}

fn get_interior_dir(d: &(i32, i32), c: char, clockwise: bool) -> Vec<(i32, i32)> {
    match (clockwise, *d, c) {
        (true, RIGHT, 'F') => vec![DOWN],
        (true, DOWN, 'F') => vec![UP, LEFT],
        (true, DOWN, '7') => vec![LEFT],
        (true, LEFT, '7') => vec![RIGHT, UP],
        (true, UP, 'J') => vec![DOWN, RIGHT],
        (true, LEFT, 'J') => vec![UP],
        (true, UP, 'L') => vec![RIGHT],
        (true, RIGHT, 'L') => vec![LEFT, DOWN],
        (true, UP, _) => vec![RIGHT],
        (true, DOWN, _) => vec![LEFT],
        (true, LEFT, _) => vec![UP],
        (true, RIGHT, _) => vec![DOWN],

        (false, RIGHT, 'F') => vec![UP, LEFT],
        (false, DOWN, 'F') => vec![DOWN],
        (false, DOWN, '7') => vec![RIGHT, UP],
        (false, LEFT, '7') => vec![LEFT],
        (false, UP, 'J') => vec![UP], 
        (false, LEFT, 'J') => vec![DOWN, RIGHT],
        (false, UP, 'L') =>  vec![LEFT, DOWN],
        (false, RIGHT, 'L') =>  vec![RIGHT],
        (false, UP, _) => vec![LEFT],
        (false, DOWN, _) => vec![RIGHT],
        (false, LEFT, _) => vec![DOWN],
        (false, RIGHT, _) => vec![UP],
        _ => panic!("Nope!")
    }
    //     if t {
//     match *d {
//         UP => match c {
//             'J' => (DOWN, RIGHT),
//             'L' => (UP, LEFT),
//         },
//         DOWN => LEFT,
//         RIGHT => DOWN,
//         LEFT => UP,
//         _ => panic!("Nope!")
//     }
// } else {
//     match *d {
//         UP => LEFT,
//         DOWN => RIGHT,
//         RIGHT => UP,
//         LEFT => DOWN,
//         _ => panic!("Nope!")
//     }
// }
}

fn mark_interior(clean_maze: &mut Vec<Vec<char>>, new_pos: (i32, i32)) -> bool {
    clean_maze
    .get_mut(new_pos.0 as usize)
    .and_then(|v| v.get_mut(new_pos.1 as usize))
    .map(|chr| if *chr == '0' { *chr = '1'; true} else { false }).unwrap_or(false)
}

fn get_surrounding_neighbours(clean_maze: &Vec<Vec<char>>, start_pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = Vec::<(i32, i32)>::new();
    for (x, y) in DIRECTIONS {
        let (new_x, new_y) = (start_pos.0 + x, start_pos.1 + y);
        let n = clean_maze.get(new_x as usize).and_then(|row| row.get(new_y as usize));
        let ok = if let Some(chr) = n {
            if *chr == '0' {
                true
            } else {
                false
            }
        } else {
            false
        };

        if ok {
            result.push((new_x, new_y));
        }
    }

    result
}

fn flood(clean_maze: &mut Vec<Vec<char>>, start_pos: (i32, i32)) -> i32 {
    let mut count = 1;
    for neigh in get_surrounding_neighbours(clean_maze, start_pos) {
        if clean_maze[neigh.0 as usize][neigh.1 as usize] == '0' {
            clean_maze[neigh.0 as usize][neigh.1 as usize] = '1';
            count += flood(clean_maze, neigh);
        }
    }

    count
}

fn compute_interior_tiles(maze: &Vec<Vec<char>>, steps: &Vec<PosDir>) -> i32 {
    // clean the maze
    let mut clean_maze = maze.clone();
    for ((pos_x, pos_y), _) in steps {
        clean_maze[*pos_x as usize][*pos_y as usize] = '*';
    }
    
    for row in &mut clean_maze {
        for col in row {
            if *col != '*' {
                *col = '0';
            }
        }
    }

    // mark interior walls
    let (idx, ((start_x, start_y), start_dir)) = steps.iter().enumerate().min_by_key(|(_, ((_, y), _))| *y).unwrap();
    let t_type = match *start_dir {
        UP => true,
        DOWN => false,
        RIGHT => maze[*start_x as usize][*start_y as usize] == 'F',
        _ => panic!("Nope at start dir")
    };

    let mut seeds = Vec::<(i32, i32)>::new();
    for i in 0..steps.len() {
        let idx = (idx + i) % steps.len();
        let (pos, dir) = steps[idx];
        for interior_dir in get_interior_dir(&dir, maze[pos.0 as usize][pos.1 as usize], t_type) {
            let new_pos = (pos.0 + interior_dir.0, pos.1 + interior_dir.1);
            if mark_interior(&mut clean_maze, new_pos) {
                seeds.push(new_pos);
                // result += 1;
            }
        }
    }

    for (x, row) in clean_maze.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == '*' {
                print!("{}", maze[x][y]);
            } else {
                print!("{}", col);
            }
        }
        println!();
    }
    
    println!();

    let mut result = 0;
    for seed in seeds {
        result += flood(&mut clean_maze, seed);
    }

    for (x, row) in clean_maze.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == '*' {
                print!("{}", maze[x][y]);
            } else {
                print!("{}", col);
            }
        }
        println!();
    }
    
    result 
}


fn find_steps_until_intersection(maze: &Vec<Vec<char>>, start: (i32, i32)) -> (Vec<PosDir>, i32) {
    let mut q = VecDeque::<((i32, i32),(i32, i32))>::new();
    let mut found = false;
    let mut steps = 1;
    let mut steps_taken = Vec::<PosDir>::new();

    for d in DIRECTIONS {
        if let Some(r) = get_next_neighbour(maze, &start, d) {
            steps_taken.push((start, *d));
            q.push_back(r);
            break;
        }
    }
    
    println!("Q: {:?}", q);
    while !found && !q.is_empty() {
        let mut new_q = VecDeque::<((i32, i32),(i32, i32))>::new();
        steps_taken.append(&mut q.iter().cloned().collect_vec());
        for (pos, dir) in q {
            if let Some((new_pos, new_dir)) = get_next_neighbour(maze, &pos, &dir) {
                new_q.push_back((new_pos, new_dir));
            } else {
                found = true;
            }
        }
        q = new_q;
        steps += 1;
    }
    steps_taken.append(&mut q.iter().cloned().collect_vec());

    (steps_taken, steps)
}

fn parse(path: &Path) -> Result<(Vec<Vec<char>>, (i32, i32))> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut maze = Vec::<Vec<char>>::new();
    let mut start = (0, 0);
    for (x, line) in reader.lines().enumerate() {
        let line = line?;
        for (y, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x as i32, y as i32);
            }
        }
        maze.push(line.chars().collect());
    }
    Ok((maze, start))
}

fn main() -> Result<()> {
    let (maze, start) = parse(Path::new("input.txt"))?;
    let (steps, steps_taken) = find_steps_until_intersection(&maze, start);
    println!("Part 1: {}", steps_taken / 2);
    let part2 = compute_interior_tiles(&maze, &steps);
    println!("Part 2: {}", part2);    
    Ok(())
}
