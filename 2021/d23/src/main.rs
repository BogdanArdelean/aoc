use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone)]
struct State {
    depth: usize,
    s: [Vec<char>; 11],
    cost: i64,
}

impl Eq for State {}

impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl State {
    fn new(depth: usize, r1: Vec<char>, r2: Vec<char>, r3: Vec<char>, r4: Vec<char>) -> Self {
        Self {
            depth,
            s: [vec!['.'], vec!['.'], r1, vec!['.'], r2, vec!['.'], r3, vec!['.'], r4, vec!['.'], vec!['.']],
            cost: 0
        }
    }
    fn go(&self, from: usize, to: usize) -> Option<State> {
        if self.s[from].is_empty() {
            return None;
        }

        let from_last = *self.s[from].last().unwrap();
        if State::get_room(from_last) == from && self.is_all_same(from_last, from) {
            return None;
        }
        if State::is_hallway(from) && State::is_hallway(to) {
            return None;
        }
        if State::is_hallway(from) && from_last == '.' {
            return None;
        }
        if State::is_hallway(to) && *self.s[to].last().unwrap() != '.' {
            return None;
        }

        if State::is_hallway(from) &&
            State::get_room(from_last) == to &&
            self.is_hallway_clear(from, to) &&
            self.is_all_same(from_last, to) {

            let mut new_state = self.clone();
            new_state.s[from][0] = '.';
            new_state.s[to].push(from_last);
            new_state.cost += (self.get_steps(from, to) * State::get_energy(from_last));

            return Some(new_state);
        } else if !State::is_hallway(from) &&
            State::is_hallway(to) &&
            self.is_hallway_clear(from, to) {

            let mut new_state = self.clone();
            new_state.s[from].pop();
            new_state.s[to][0] = from_last;
            new_state.cost += (self.get_steps(from, to) * State::get_energy(from_last));

            return Some(new_state);
        } else if !State::is_hallway(from) &&
            !State::is_hallway(to) &&
            self.is_hallway_clear(from, to) &&
            State::get_room(from_last) == to &&
            self.is_all_same(from_last, to)
            {

            let mut new_state = self.clone();
            new_state.s[from].pop();
            new_state.s[to].push(from_last);
            new_state.cost += (self.get_steps(from, to) * State::get_energy(from_last));

            return Some(new_state);
        }

        None
    }

    fn is_hallway(h: usize) -> bool {
        match h {
            0 => true,
            1 => true,
            3 => true,
            5 => true,
            7 => true,
            9 => true,
            10 => true,
            _ => false
        }
    }

    fn get_room(a: char) -> usize {
        match a {
            'A' => 2,
            'B' => 4,
            'C' => 6,
            'D' => 8,
            _ => usize::MAX
        }
    }

    fn get_energy(a: char) -> i64 {
        match a {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("Say what?!")
        }
    }

    fn is_all_same(&self, a: char, to: usize) -> bool {
        self.s[to]
            .iter()
            .all(|c| a == *c)
    }

    fn is_hallway_clear(&self, from: usize, to: usize) -> bool {
        for i in (from.min(to)..=from.max(to)) {
            if State::is_hallway(i) && self.s[i][0] != '.' && i != from {
                return false;
            }
        }

        true
    }

    fn get_steps(&self, from: usize, to: usize) -> i64 {
        let depth = if State::is_hallway(from) {
            (self.depth - self.s[to].len()) as i64
        } else if State::is_hallway(to) {
            (self.depth - self.s[from].len()) as i64 + 1
        } else {
            (self.depth - self.s[from].len()) as i64 +
                (self.depth - self.s[to].len()) as i64 + 1
        };

        (to as i64 - from as i64).abs() + depth
    }

    fn is_orgranized(&self) -> bool {
        self.is_all_same('A', 2) && self.s[2].len() == self.depth &&
            self.is_all_same('B', 4) && self.s[4].len() == self.depth &&
            self.is_all_same('C', 6) && self.s[6].len() == self.depth &&
            self.is_all_same('D', 8) && self.s[8].len() == self.depth
    }
}

fn find_min_bfs(state: State) -> i64 {
    let mut q = BinaryHeap::<State>::new();
    let mut hs = HashSet::<([Vec<char>; 11], i64)>::new();

    hs.insert((state.s.clone(), state.cost));
    q.push(state);
    while !q.is_empty() {
        let state = q.pop().unwrap();
        if state.is_orgranized() {
            return state.cost;
        }

        for i in 0..11 {
            for j in 0..11 {
                if let Some(aux) = state.go(i, j) {
                    if !hs.contains(&(aux.s.clone(), aux.cost)) {
                        hs.insert((aux.s.clone(), aux.cost));
                        q.push(aux);
                    }
                }
            }
        }
    }
    0
}

fn main() {
    let state = State::new(
        4,
        vec!['C', 'D', 'D', 'B'],
        vec!['D', 'B', 'C', 'D'],
        vec!['B', 'A', 'B', 'C'],
        vec!['A', 'C', 'A', 'A']
    );
    let min_energy = find_min_bfs(state.clone());

    println!("Part 1 {}", min_energy);
}
