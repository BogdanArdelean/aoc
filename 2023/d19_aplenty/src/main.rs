use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use anyhow::{anyhow, Result, bail};
use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use scan_fmt::scan_fmt;

type MinMaxMap = HashMap<Category, (i128, i128)>;
type VisitedMap = HashMap<Category, [bool; 4001]>;

type WorkflowId = String;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum Category {
    #[default] X,
    M,
    A,
    S
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!("At the category!")
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum Condition {
    Less(Category, i32),
    Greater(Category, i32),
    #[default] None
}

impl Condition {
    fn from(cat: Category, sign: char, value: i32) -> Condition {
        if sign == '<' {
            Condition::Less(cat, value)
        } else if sign == '>' {
            Condition::Greater(cat, value)
        } else {
            panic!("Invalid sign!");
        }
    }

    fn get_comparison_value(&self) -> i32 {
        match &self {
            Condition::Less(_, x) => *x,
            Condition::Greater(_, x) => *x,
            Condition::None => std::i32::MAX,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Rule {
    condition: Condition,
    target_workflow: WorkflowId
}

impl Rule {
    fn matches(&self, category: Category, rate: i32) -> Option<WorkflowId> {
        match self.condition {
            Condition::Less(c, r) if c == category && rate < r => Some(self.target_workflow.clone()),
            Condition::Greater(c, r) if c == category && rate > r => Some(self.target_workflow.clone()),
            Condition::None => Some(self.target_workflow.clone()),
            _ => None
        }
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.condition.get_comparison_value() == other.condition.get_comparison_value()
    }
}

impl Eq for Rule {
}

impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.condition.get_comparison_value().partial_cmp(&other.condition.get_comparison_value())
    }
}

impl Ord for Rule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.condition.get_comparison_value().cmp(&other.condition.get_comparison_value())
    }
}

#[derive(Debug, Clone, Default)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32
}

impl Part {
    fn matches(&self, rule: &Rule) -> Option<WorkflowId> {
        rule.matches(Category::X, self.x)
        .or(rule.matches(Category::M, self.m))
        .or(rule.matches(Category::A, self.a))
        .or(rule.matches(Category::S, self.s))
    }

    fn sum(&self) -> i64 {
        return (self.x + self.m + self.a + self.s).into();
    }
}

#[derive(Debug, Clone, Default)]
struct Workflow {
    id: WorkflowId,
    rules: Vec<Rule>
}

impl Workflow {
    fn new(id: WorkflowId, mut rules: Vec<Rule>) -> Workflow {
        // rules.sort();
        
        Workflow {
            id,
            rules
        }
    }
    fn next(&self, part: &Part) -> WorkflowId {
        for rule in &self.rules {
            if let Some(workflow_id) = part.matches(&rule) {
                return workflow_id
            }
        }
        panic!("Should match at least one rule!");
    }

    fn first_match(&self, part: &Part) -> Option<WorkflowId> {
        for rule in self.rules.iter().take(self.rules.len() - 1) {
            if let Some(workflow_id) = part.matches(&rule) {
                return Some(workflow_id)
            }
        }

        None
    }
}

type WorkflowMap = LinkedHashMap<WorkflowId, Workflow>;

fn parse(path: &Path) -> Result<(WorkflowMap, Vec<Part>)> {
    let mut workflow_map = WorkflowMap::new();
    let mut parts = Vec::<Part>::new();

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let workflows_str = buf.split("\n\n").nth(0).unwrap();
    let part_str = buf.split("\n\n").last().unwrap();

    for workflow_str in workflows_str.split("\n") {
        let wid = scan_fmt!(&workflow_str, "{}{", String)?;
        let rules_s = workflow_str.split("{").nth(1).unwrap().replace("}", "");
        
        let mut rules_vec = Vec::<Rule>::new();
        for r in rules_s.split(",") {
            let scan = scan_fmt!(&r, "{/[xmas]/}{/[<>]/}{d}:{}", char, char, i32, String);
            let rule = if let Ok((cat, sign, val, twid)) = scan {
                let category = cat.into();
                let condition = Condition::from(category, sign, val);
                Rule { condition, target_workflow: twid }
            } else {
                Rule { condition: Condition::None, target_workflow: r.to_string() }
            };
            rules_vec.push(rule);
        }

        workflow_map.insert(wid.clone(), Workflow::new(wid, rules_vec));
    }

    for part in part_str.split("\n") {
        if part.is_empty() {
            continue;
        }

        let (x, m, a, s) = scan_fmt!(&part, "{{x={},m={},a={},s={}}}", i32, i32, i32, i32)?;
        parts.push(Part { x, m, a, s });
    }

    anyhow::Ok((workflow_map, parts))
}

fn part1(wmap: &WorkflowMap, parts: &Vec<Part>) -> i64 {
    let mut sum = 0;

    for part in parts {
        // Find start
        let mut start = "in".to_string();
        while start != "R" && start != "A" {
            let wf = &wmap[&start];
            start = wf.next(part);
            println!("{}, {:?}", start, part);
        }
        if start == "A" {
            sum += part.sum();
        }
    }

    sum
}

fn add_constraint(mp: &MinMaxMap, rule: &Rule, rev: bool) -> MinMaxMap {
    let mut new_map = mp.clone();
    if !rev {
        match rule.condition {
            Condition::Less(cat, rating) => {
                let val = new_map.get_mut(&cat).unwrap();
                val.1 = val.1.min(rating as i128);
                new_map
            },
            Condition::Greater(cat, rating) => {
                let val = new_map.get_mut(&cat).unwrap();
                val.0 = val.0.max(rating as i128);
                new_map
            },
            Condition::None => new_map,
        }
    } else {
        match rule.condition {
            Condition::Less(cat, rating) => {
                let val = new_map.get_mut(&cat).unwrap();
                val.0 = val.0.max(rating as i128 - 1);
                new_map
            },
            Condition::Greater(cat, rating) => {
                let val = new_map.get_mut(&cat).unwrap();
                val.1 = val.1.min(rating as i128 + 1);
                new_map
            },
            Condition::None => new_map,
        }
    }
}

fn part_2(node: &WorkflowId, graph: &WorkflowMap, mut mp: MinMaxMap, out: &mut Vec<MinMaxMap>) {
    if node == "R" {
        return;
    }

    if node == "A" {
        out.push(mp);
        return;
    }

    for rule in &graph[node].rules {
        part_2(&rule.target_workflow, graph, add_constraint(&mp, rule, false), out);
        mp = add_constraint(&mp, rule, true);
    }
}

fn intersection(a: &MinMaxMap, b: &MinMaxMap) -> MinMaxMap {
    let mut c = MinMaxMap::new();
    for (cat, (al, ar)) in a {
        let (bl, br) = b.get(cat).unwrap();
        c.insert(*cat, (*al.max(bl), *ar.min(br)));
    }
    c
}

fn possibilities(a: &MinMaxMap) -> i128 {
    let mut prod = 1;
    
    for (l, r) in a.values() {
        let s = if r - l - 1 <= 0 {
            1
        } else {
            r - l - 1
        };

        prod *= s;
    } 

    prod
}

fn calculate(out: &Vec<MinMaxMap>) -> i128 {
    let mut p = 0;
    for i in 0..out.len() {
        let mut sum = possibilities(&out[i]);
        // let mut it = out[i].clone();

        // for j in i+1..out.len() {
        //     it = intersection(&it, &out[j]);
        // }
        // sum -= (out.len() as i128 - 1) * possibilities(&it);
        p += sum;
    }   

    p
}

fn main() -> Result<()> {
    let (wmap, parts) = parse(Path::new("input.txt"))?;
    let part1 = part1(&wmap, &parts);
    
    let mut mp = MinMaxMap::new();
    mp.insert(Category::X, (0, 4001));
    mp.insert(Category::M, (0, 4001));
    mp.insert(Category::A, (0, 4001));
    mp.insert(Category::S, (0, 4001));
    
    let mut out = Vec::<_>::new();
    part_2(&"in".to_string(), &wmap, mp.clone(), &mut out);
    println!("{:?}", out);
    let part2 = calculate(&out);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    anyhow::Ok(())
}
