use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    name: String,
}

impl Node {
    fn create(name: String) -> Self {
        Self {
            name
        }
    }

    fn is_start(&self) -> bool {
        self.name == "start"
    }

    fn is_end(&self) -> bool {
        self.name == "end"
    }

    fn is_small(&self) -> bool {
        self.name.chars().all(|c| c.is_lowercase())
    }
}

#[derive(Debug, Clone, Default)]
struct Graph {
    neighbours: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn add_edge(&mut self, a: Node, b: Node) {
        self.neighbours.entry(a.clone()).or_default().push(b.clone());
        self.neighbours.entry(b.clone()).or_default().push(a.clone());
    }

    fn get_start(&self) -> Node {
        self.neighbours.keys().find(|k|k.is_start()).unwrap().clone()
    }

    fn get_end(&self) -> Node {
        self.neighbours.keys().find(|k|k.is_end()).unwrap().clone()
    }

    fn get_neighbours<'a>(&'a self, n: &'a Node) -> impl Iterator<Item = &'a Node> {
        self.neighbours.get(n).into_iter().flat_map(|v| v.iter())
    }
}

fn parse(path: &Path) -> Result<Graph> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut graph = Graph::default();
    for line in reader.lines() {
        let line = line?;
        let mut splt = line.split("-");
        let n1 = Node::create(splt.next().unwrap().to_string());
        let n2 = Node::create(splt.next().unwrap().to_string());

        graph.add_edge(n1, n2);
    }

    Ok(graph)
}

fn dfs(node: &Node, graph: &Graph, seen: &mut HashSet<Node>) -> i32 {
    if node.is_end() {
        return 1;
    }

    let mut count = 0;

    if node.is_small() {
        seen.insert(node.clone());
    }

    for neighbour in graph.get_neighbours(node) {
        if seen.contains(neighbour) {
            continue;
        }

        count += dfs(neighbour, graph, seen);
    }

    if node.is_small() {
        seen.remove(&node);
    }

    count
}

fn dfs2(node: &Node, graph: &Graph, seen: &mut HashSet<Node>, twice: Option<Node>) -> i32 {
    if node.is_end() {
        return 1;
    }

    let mut count = 0;

    if node.is_small() {
        seen.insert(node.clone());
    }

    for neighbour in graph.get_neighbours(node) {
        if seen.contains(neighbour) && !neighbour.is_start() && twice.is_none() {
            count += dfs2(neighbour, graph, seen, Some(neighbour.clone()));
        }

        if seen.contains(neighbour) {
            continue;
        }

        count += dfs2(neighbour, graph, seen, twice.clone());
    }

    if node.is_small() {
        seen.remove(&node);
    }

    if let Some(x) = twice {
        if x.name == node.name {
            seen.insert(node.clone());
        }
    }

    count
}

fn part_1(graph: &Graph) -> i32 {
    let start = graph.get_start();
    let mut seen = HashSet::new();

    dfs(&start, graph, &mut seen)
}

fn part_2(graph: &Graph) -> i32 {
    let start = graph.get_start();
    let mut seen = HashSet::new();

    dfs2(&start, graph, &mut seen, None)
}

fn main() -> Result<()> {
    let graph = parse(&Path::new("input.txt"))?;
    let p1 = part_1(&graph);
    println!("Part 1 {}", p1);

    let p2 = part_2(&graph);
    println!("Part 2 {}", p2);

    Ok(())
}
