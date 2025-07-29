use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use anyhow::{Result, Ok};
use itertools::Itertools;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash, Clone, Copy)]
enum Direction {
    N,
    W,
    S,
    E
}
type ForwardSteps = i32;
type Index = i32;
type TownMap = Vec<Vec<i32>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    dir: Direction,
    forward: ForwardSteps,
    x: Index,
    y: Index
}

impl State {
    fn new(dir: Direction, forward: ForwardSteps, x: Index, y: Index) -> Self {
        State {
            dir,
            forward,
            x,
            y
        }
    }
}

impl From<(Direction, ForwardSteps, Index, Index)> for State {
    fn from(value: (Direction, ForwardSteps, Index, Index)) -> Self {
        State {
            dir: value.0,
            forward: value.1,
            x: value.2,
            y: value.3
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HeapState {
    cost: i32,
    state: State
}

impl HeapState {
    fn new(cost: i32, state: State) -> Self{ 
        Self { cost, state }
    }
}

impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost).and_then(|o| Some(o.reverse()))
    }
}

impl Ord for HeapState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

fn is_state_valid(map: &TownMap, state: &State) -> bool {
    !(state.x < 0 || state.x >= map.len() as i32 || state.y < 0 || state.y >= map[0].len() as i32)
}

fn updated_indices(x: Index, y: Index, dir: Direction) -> (Index, Index) {
    match dir {
        Direction::N => (x - 1, y),
        Direction::W => (x, y - 1),
        Direction::S => (x + 1, y),
        Direction::E => (x, y + 1),
    }
}

fn get_neighbours(state: &State) -> Vec<State> {
    let mut result = Vec::<State>::new();
    let dirs = match state.dir {
        Direction::N => [Direction::E, Direction::W, Direction::N],
        Direction::W => [Direction::N, Direction::S, Direction::W],
        Direction::S => [Direction::W, Direction::E, Direction::S],
        Direction::E => [Direction::N, Direction::S, Direction::E]
    };

    let pos_dir = dirs.iter().copied().map(|dir| (updated_indices(state.x, state.y, dir), dir)).collect_vec();
    for ((x_n, y_n), n_dir) in pos_dir {
        if n_dir != state.dir {
            result.push((n_dir, 1, x_n, y_n).into());
        } else if state.forward < 3 {
            result.push((n_dir, state.forward + 1, x_n, y_n).into());
        }
    }
    result
}

fn get_ultra_neighbours(state: &State) -> Vec<State> {
    assert!(state.forward <= 10);
    let mut result = Vec::<State>::new();
    let dirs = match state.dir {
        Direction::N => [Direction::E, Direction::W, Direction::N],
        Direction::W => [Direction::N, Direction::S, Direction::W],
        Direction::S => [Direction::W, Direction::E, Direction::S],
        Direction::E => [Direction::N, Direction::S, Direction::E]
    };

    let pos_dir = dirs.iter().copied().map(|dir| (updated_indices(state.x, state.y, dir), dir)).collect_vec();
    for ((x_n, y_n), n_dir) in pos_dir {
        if n_dir != state.dir && (state.forward >= 4 || state.forward == 0) {
            result.push((n_dir, 1, x_n, y_n).into());
        } else if n_dir == state.dir && state.forward < 10 {
            result.push((n_dir, state.forward + 1, x_n, y_n).into());
        }
    }
    result
}

fn shortest_path<N, D>((start_x, start_y): (i32, i32), (end_x, end_y): (i32, i32), map: &TownMap, neighbours: N, done: D) -> i32
where 
    N: Fn(&State) -> Vec<State>,
    D: Fn(&State, i32, i32) -> bool
{
    let mut heap = BinaryHeap::<HeapState>::new();
    let mut visited = HashSet::<State>::new();
    let mut min_path = std::i32::MAX;

    heap.push(HeapState::new(0, State::new(Direction::E, 0, start_x, start_y)));

    while !heap.is_empty() {
        let hstate = heap.pop().unwrap();
        let state = hstate.state;
        
        if visited.contains(&state) {
            continue;
        }

        visited.insert(state);

        if done(&state, end_x, end_y) {
            min_path = min_path.min(hstate.cost);
            break;
        }

        for new_state in neighbours(&state) {
            if is_state_valid(map, &new_state) && !visited.contains(&new_state) {
                heap.push(HeapState::new(hstate.cost + map[new_state.x as usize][new_state.y as usize], new_state));
            }
        }
    }

    min_path
}

fn parse(path: &Path) -> Result<TownMap> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut map = TownMap::new();
    for line in reader.lines() {
        let line = line?;;
        map.push(line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect());
    }

    Ok(map)
}

fn main() -> Result<()> {
    let map = parse(Path::new("input.txt"))?;
    
    let start = Instant::now();
    
    let source = (0, 0);
    let target = (map.len() as i32 - 1, map[0].len() as i32 - 1);
    let part_1 = shortest_path(source, target, &map, get_neighbours, |s, x, y| (s.x == x && s.y == y));
    let part_2 = shortest_path(source, target, &map, get_ultra_neighbours, |s, x, y| (s.x == x && s.y == y && s.forward >= 4));
    
    let duration = start.elapsed();
    
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    println!("Elapsed: {:#?}", duration);
    Ok(())
}
