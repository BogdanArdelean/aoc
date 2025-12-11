use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
struct Machine {
    // part 1
    target_size: usize,
    target: u32,
    moves: Vec<u32>,

    // part 2
    target_joltage: [i16; 10],
    joltage_moves: Vec<[i16; 10]>,
    prefix: Vec<[i16; 10]>,
}

impl Machine {
    fn new(spec: &str) -> Self {
        let (target_state_str, rest) = {
            let v: Vec<&str> = spec.split("]").collect();
            (v[0], v[1])
        };
        let mut target = 0;
        let mut target_size = 0;
        for c in target_state_str[1..].chars() {
            target_size += 1;
            target <<= 1;
            if c == '#' {
                target |= 1;
            }
        }

        let (moves_str, rest) = {
            let v: Vec<&str> = rest.split("{").collect();
            (v[0], v[1])
        };

        let moves: Vec<u32> = moves_str
            .split(") ")
            .map(|x| x.trim())
            .filter(|s| s.len() > 0)
            .map(|s| {
                if s.len() == 0 {
                    return 0;
                }
                if s.len() == 2 {
                    return (1 << (target_size - 1 - (s.as_bytes()[1] - '0' as u8)));
                }
                let mut m = 0;
                for nr_str in s[1..].split(",") {
                    let nr = nr_str.parse::<u8>().unwrap();
                    m = m | (1 << (target_size - 1 - nr));
                }
                m
            })
            .collect();

        let target_joltage_aux: Vec<i16> = rest
            .split("}")
            .filter(|s| s.len() > 0)
            .flat_map(|s| s.split(","))
            .map(|x| x.parse().unwrap())
            .collect();

        let mut target_joltage = [0i16; 10];
        for (idx, joltage) in target_joltage_aux.iter().enumerate() {
            target_joltage[target_size as usize - 1 - idx] = *joltage;
        }

        let mut joltage_moves: Vec<_> = moves_str
            .split(") ")
            .map(|x| x.trim())
            .filter(|s| s.len() > 0)
            .map(|s| {
                let mut joltage_move = [0i16; 10];
                if s.len() == 2 {
                    let idx = (target_size - 1 - (s.as_bytes()[1] - '0' as u8)) as usize;
                    joltage_move[idx] = 1;
                } else {
                    for nr_str in s[1..].split(",") {
                        let nr = nr_str.parse::<u8>().unwrap();
                        let idx = (target_size - 1 - nr) as usize;
                        joltage_move[idx] = 1;
                    }
                };
                joltage_move
            })
            .collect();

        // so it converges 'faster' to a solution
        // in order to give an upper bound
        joltage_moves.sort_by_key(|v| {
            Reverse(v.iter().sum::<i16>())
        });

        let mut prefix = vec![];
        let mut accumulator = [0i16; 10];
        for e in joltage_moves.iter().rev() {
            accumulator = or_v(accumulator, *e);
            prefix.push(accumulator);
        }
        prefix.reverse();
        prefix = prefix.into_iter().map(|x| not_v(x)).collect();
        Self {
            target_size: target_size as usize,
            target,
            moves,

            target_joltage,
            joltage_moves,
            prefix,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct State {
    cost: u64,
    state: u32,
}

fn parse(path: &Path) -> Vec<Machine> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|x| Machine::new(&x.unwrap()))
        .collect()
}

fn min_path<F>(m: &Machine, cost_model: F) -> u64
where F: Fn(&Machine, &State, usize) -> State
{
    let mut viz = vec![false; 1 << m.target_size];
    let mut min_state = vec![u64::MAX; 1 << m.target_size];
    let mut binary_heap = BinaryHeap::<_>::new();

    binary_heap.push(Reverse(State {cost: 0, state: 0}));
    while let Some(Reverse(state)) = binary_heap.pop() {

        // we've seen this
        if viz[state.state as usize] {
            continue;
        }
        // we've reached the target state
        if state.state == m.target {
            return state.cost
        }

        viz[state.state as usize] = true;
        for i in 0..m.moves.len() {
            let new_state = cost_model(m, &state, i);

            if viz[new_state.state as usize] {
                continue;
            }
            // don't fill up the heap if it's not necessary
            if min_state[new_state.state as usize] < new_state.cost {
                continue;
            }

            min_state[new_state.state as usize] = new_state.cost;
            binary_heap.push(Reverse(new_state));
        }
    }
    panic!("There should be a path to a state");
}
fn part1(machines: &Vec<Machine>) -> u64 {
    fn cost_per_move(m: &Machine, s: &State, i: usize) -> State {
        State {
            cost: s.cost + 1,
            state: s.state ^ m.moves[i],
        }
    }
    machines
        .iter()
        .map(|m| min_path(m, cost_per_move))
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct JoltageState {
    cost: i32,
    state: [i16; 10]
}

fn or_v(a: [i16; 10], b: [i16; 10]) -> [i16; 10] {
    let mut r = [0i16; 10];
    for i in 0..a.len() {
        r[i] = a[i] | b[i];
    }
    r
}

fn add_v(a: [i16; 10], b: [i16; 10]) -> [i16; 10] {
    let mut r = [0i16; 10];
    for i in 0..a.len() {
        r[i] = a[i] + b[i];
    }
    r
}

fn sub_v(a: [i16; 10], b: [i16; 10]) -> [i16; 10] {
    let mut r = [0i16; 10];
    for i in 0..a.len() {
        r[i] = a[i] - b[i];
    }
    r
}

fn max_v(a: [i16; 10]) -> i16 {
    let mut r = 0i16;
    for i in 0..a.len() {
        r = r.max(a[i]);
    }
    r
}

fn min_v_masked(a: [i16; 10], mask: [i16; 10]) -> i16 {
    let mut r = i16::MAX - 1;
    for i in 0..a.len() {
        let first = a[i] * mask[i];
        let second = (i16::MAX-1) * (!mask[i] & 1);
        r = r.min(first + second);
    }
    r
}

fn max_v_masked(a: [i16; 10], mask: [i16; 10]) -> i16 {
    let mut r = -1;
    for i in 0..a.len() {
        let first = a[i] * mask[i];
        let second = -1 * (!mask[i] & 1);
        r = r.max(first + second);
    }
    r
}

fn mul_s_v(a: [i16; 10], b: i16) -> [i16; 10] {
    let mut r = [0i16; 10];
    for i in 0..a.len() {
        r[i] = a[i] * b;
    }
    r
}

fn not_v(a: [i16; 10]) -> [i16; 10] {
    let mut r = [0i16; 10];
    for i in 0..a.len() {
        r[i] = !a[i] & 1;
    }
    r
}

fn search(m: &Machine, state: JoltageState, move_idx: usize, upper_bound: &mut i32) -> bool {
    fn lower_bound(state: &JoltageState, m: &Machine) -> i32 {
        state.cost + max_v(sub_v(m.target_joltage, state.state)) as i32
    }

    if m.target_joltage == state.state {
        *upper_bound = (*upper_bound).min(state.cost);
        return false;
    }

    if move_idx >= m.moves.len() {
        return false;
    }

    if lower_bound(&state, m) > *upper_bound {
        return true;
    }

    let joltage_move = m.joltage_moves[move_idx];
    let ceiling_with_move = min_v_masked(sub_v(m.target_joltage, state.state), joltage_move);

    for nr in (0..=ceiling_with_move).rev() {
        let increment = mul_s_v(joltage_move, nr);
        let new_state = JoltageState {
            cost: state.cost + nr as i32,
            state: add_v(state.state, increment)
        };
        let rem = sub_v(m.target_joltage, new_state.state);
        let x = if move_idx + 1 < m.prefix.len() {
            m.prefix[move_idx + 1]
        } else {
            [1i16; 10]
        };
        let mn = max_v_masked(rem, x);
        // we can't possibly reach the quota
        if mn != -1 && mn != 0 {
            return true;
        }
        let should_continue = search(m, new_state, move_idx + 1, upper_bound);
        if !should_continue {
            return true;
        }
    }
    true
}

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn part2(machines: &Vec<Machine>, num_threads: usize) -> u64 {
    let machines = Arc::new(machines.clone());
    let next = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let machines = Arc::clone(&machines);
        let next = Arc::clone(&next);

        let handle = thread::spawn(move || {
            let mut sum = 0u64;

            loop {
                let idx = next.fetch_add(1, Ordering::Relaxed);

                if idx >= machines.len() {
                    break;
                }

                let machine = &machines[idx];

                let mut upper_bound = 300;
                let state = JoltageState { cost: 0, state: [0i16; 10] };

                search(machine, state, 0, &mut upper_bound);

                println!("{}: {}", idx, upper_bound);

                sum += upper_bound as u64;
            }

            sum
        });

        handles.push(handle);
    }

    // collect partial sums
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

fn main() {
    let machines = parse(Path::new("input.txt"));
    let p1 = part1(&machines);
    println!("part 1: {}", p1);

    let p2 = part2(&machines, 32);
    println!("part 2: {}", p2);
}
