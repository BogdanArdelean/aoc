use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node(String);
type Graph = HashMap<Node, Vec<Node>>;

fn parse(path: &Path) -> Graph {
    let mut graph = Graph::new();
    let reader = BufReader::new(File::open(path).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let (from, rest) = {
            let v: Vec<&str> = line.split(":").collect();
            (v[0], v[1])
        };

        let from_node = Node(from.into());
        for to in rest.trim().split_whitespace() {
            let to_node = Node(to.into());
            graph.entry(from_node.clone()).or_default().push(to_node.clone());
            graph.entry(to_node.clone()).or_default();
        }
    }

    graph
}

fn topo_sort(g: &Graph, from: Node) -> Vec<Node> {
    fn dfs(c: &Node, g: &Graph, viz: &mut HashSet<Node>, ts: &mut Vec<Node>) {
        if viz.contains(&c) {
            return;
        }
        viz.insert(c.clone());
        for neighbour in &g[&c] {
            dfs(neighbour, g, viz, ts);
        }
        ts.push(c.clone());
    }
    let mut viz = HashSet::<Node>::new();
    let mut r = vec![];
    dfs(&from, g, &mut viz, &mut r);
    r.reverse();
    r
}

fn count_paths(g: &Graph, ts: &Vec<Node>, to: Node) -> i64 {
    let mut path_count = HashMap::<Node, i64>::new();
    for n in ts {
        for neigh in &g[n] {
            *path_count.entry(neigh.clone()).or_default() += *path_count.entry(n.clone()).or_insert(1);
        }
    }
    *path_count.get(&to).unwrap_or(&0)
}

fn part2(g: &Graph) -> i64 {
    /* it's a DAG. so it's one or the other */
    let svr_dac = count_paths(&g, &topo_sort(&g, Node("svr".into())), Node("dac".into()));
    let fft_dac = count_paths(&g, &topo_sort(&g, Node("fft".into())), Node("dac".into()));
    let dac_fft = count_paths(&g, &topo_sort(&g, Node("dac".into())), Node("fft".into()));
    let svr_fft = count_paths(&g, &topo_sort(&g, Node("svr".into())), Node("fft".into()));
    let dac_out = count_paths(&g, &topo_sort(&g, Node("dac".into())), Node("out".into()));
    let fft_out = count_paths(&g, &topo_sort(&g, Node("fft".into())), Node("out".into()));

    let one = svr_dac * dac_fft * fft_out;
    let two = svr_fft * fft_dac * dac_out;

    one + two
}

fn main() {
    let g = parse(Path::new("input.txt"));
    let p1 = count_paths(&g, &topo_sort(&g, Node("you".into())), Node("out".into()));
    println!("part 1: {}", p1);

    let p2 = part2(&g);
    println!("part 2: {}", p2);
}
