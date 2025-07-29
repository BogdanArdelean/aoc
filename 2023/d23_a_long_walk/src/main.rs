use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use anyhow::{anyhow, Result, Ok};

type HikeMap = Vec<Vec<char>>;

fn add((a, b): (i32, i32), (c, d): (i32, i32)) -> (i32, i32) {
    (a + c, b + d)
}

fn subtract((a, b): (i32, i32), (c, d): (i32, i32)) -> (i32, i32) {
    (a - c, b - d)
}

fn is_within_bounds((x, y): (i32, i32), hm: &HikeMap) -> bool {
    !((x < 0) || (x >= hm.len() as i32) || (y < 0) || (y >= hm[0].len() as i32))
}

fn is_walkable((x, y): (i32, i32), hm: &HikeMap) -> bool {
    hm[x as usize][y as usize] != '#' 
}

fn can_hike_slope((x, y): (i32, i32), dir: (i32, i32), hm: &HikeMap) -> bool {
    if hm[x as usize][y as usize] == 'S' {
        return true;
    }

    if hm[x as usize][y as usize] == '.' {
        return true;
    }

    if hm[x as usize][y as usize] == '>' && dir.1 == 1 {
        return true;
    }

    if hm[x as usize][y as usize] == '<' && dir.1 == -1 {
        return true;
    }
    
    if hm[x as usize][y as usize] == '^' && dir.0 == -1 {
        return true;
    }

    if hm[x as usize][y as usize] == 'v' && dir.0 == 1 {
        return true;
    }

    false 
}

fn get_neighbours((x, y): (i32, i32), hm: &HikeMap) -> Vec<(i32, i32)> {
    let dirs = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    let chr = hm[x as usize][y as usize];
    let result: Vec<((i32, i32), (i32, i32))> = match chr {
        '>' => {
            vec![(add((x, y), (0, 1)), (0, 1))]
        },
        '<' => {
            vec![(add((x, y), (0, -1)), (0, -1))]
        },
        'v' => {
            vec![(add((x, y), (1, 0)), (1, 0))]
        },
        '^' => {
            vec![(add((x, y), (-1, 0)), (-1, 0))]
        }
        _ => {
            dirs.iter().map(|dir| (add((x, y), *dir), *dir)).collect()
        }
    };
    
    result.into_iter().filter(|(elm, dir)| is_within_bounds(*elm, hm) && is_walkable(*elm, hm)).map(|(elm, _)| elm).collect()
}


fn print(hm: &HikeMap) {
    for row in hm {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn find_maximal_path((start_x, start_y): (i32, i32), (end_x, end_y): (i32, i32), hm: &mut HikeMap, current_path: i64) -> i64 {
    // println!("{} {}: {}", start_x, start_y, current_path);

    if start_x == end_x && start_y == end_y {
        // print(hm);
        return current_path;
    }

    let mut max_path = 0;
    let neighbours = get_neighbours((start_x, start_y), hm);
    for (nx, ny) in neighbours {
        if can_hike_slope((nx, ny), subtract((start_x, start_y), (nx, ny)), hm) {
            let last = hm[nx as usize][ny as usize];

            hm[nx as usize][ny as usize] = 'X';
            max_path = max_path.max(find_maximal_path((nx, ny), (end_x, end_y), hm, current_path + 1));
            hm[nx as usize][ny as usize] = last;
        }
    }

    max_path
}

fn find_maximal_path_2((start_x, start_y): (i32, i32), (end_x, end_y): (i32, i32), hm: &mut HikeMap, current_path: i64, prune: &mut HashMap<((i32, i32), (i32, i32)), i64>) -> i64 {
    // println!("{} {}: {}", start_x, start_y, current_path);
    if start_x == end_x && start_y == end_y {
        // print(hm);
        return current_path;
    }

    let mut max_path = 0;
    let neighbours = get_neighbours((start_x, start_y), hm);
    for (nx, ny) in neighbours {
        let last = hm[nx as usize][ny as usize];
        
        if let Some(x) = prune.get(&((start_x, start_y), (nx, ny))) {
            if *x > current_path {
                continue;
            }
        }

        hm[nx as usize][ny as usize] = 'X';
        
        let path = find_maximal_path_2((nx, ny), (end_x, end_y), hm, current_path + 1, prune);
        max_path = max_path.max(path);
        
        hm[nx as usize][ny as usize] = last;

        prune.insert(((start_x, start_y), (nx, ny)), current_path + 1);
    }
    
    max_path
}

type Node = (i32, i32);

#[derive(Debug, Clone, Copy)]
struct Edge {
    start: Node,
    end: Node,
    cost: i32
}

fn extract_graph((start_x, start_y): (i32, i32), hm: &mut HikeMap, (ce_x, ce_y): (i32, i32), res: &mut Vec<Edge>, cost: i32) {
    if hm[start_x as usize][start_y as usize] == 'S' {
        res.push(Edge { start: (ce_x, ce_y), end: (start_x, start_y), cost });
        return;
    }
    
    if hm[start_x as usize][start_y as usize] == '.' {
        hm[start_x as usize][start_y as usize] = '#';
    }

    let mut new_edge = false;
    let neighbours = get_neighbours((start_x, start_y), hm);
    
    if neighbours.is_empty() {
        res.push(Edge { start: (ce_x, ce_y), end: (start_x, start_y), cost });
        return;
    }

    if neighbours.len() > 2 {
        hm[start_x as usize][start_y as usize] = 'S';
        res.push(Edge { start: (ce_x, ce_y), end: (start_x, start_y), cost });
        new_edge = true;
    }

    for (nx, ny) in neighbours {
        if can_hike_slope((nx, ny), subtract((nx, ny), (start_x, start_y)), hm) {
            if new_edge {
                extract_graph((nx, ny), hm, (start_x, start_y), res, 1);
            } else {
                extract_graph((nx, ny), hm, (ce_x, ce_y), res, cost + 1);
            }
        }
    }
}

fn parse(path: &Path) -> Result<HikeMap> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut hm = HikeMap::new();
    for line in reader.lines() {
        let line = line?;
        hm.push(line.chars().collect());
    }

    Ok(hm)
}

fn make_adj_list(edges: &Vec<Edge>) -> HashMap<Node, Vec<(Node, i32)>> {
    let mut adj_list = HashMap::new();
    for edge in edges {
        {
            let start = adj_list.entry(edge.start).or_insert(Vec::new());
            start.push((edge.end, edge.cost));
        }
        {
            let end = adj_list.entry(edge.end).or_insert(Vec::new());
            end.push((edge.start, edge.cost));
        }
    }
    adj_list
}

fn maximal_path(start: Node, end: Node, adj: &HashMap<Node, Vec<(Node, i32)>>, viz: &mut HashSet<Node>, current_path: i64) -> i64 {
    if start == end {
        return current_path;
    }

    let mut max_path = 0;
    for (next, cost) in &adj[&start] {
        if viz.contains(next) {
            continue;
        }

        viz.insert(*next);
        max_path = max_path.max(maximal_path(*next, end, adj, viz, current_path + *cost as i64));
        viz.remove(next);
    }

    max_path
}

fn main() -> Result<()> {
    let mut hm = parse(&Path::new("input.txt"))?;
    // let part_1 = find_maximal_path((0, 1), (hm.len() as i32 - 1, hm[0].len() as i32 - 2), &mut hm, 0);
    let mut res = Vec::<_>::new();
    extract_graph((0, 1), &mut hm, (0, 1), &mut res, 0);
    
    let adj_list = make_adj_list(&res);
    let part_2 = maximal_path((0, 1), (hm.len() as i32 - 1, hm[0].len() as i32 - 2), &adj_list, &mut HashSet::new(), 0);
    // println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    Ok(())
}

