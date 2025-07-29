use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Output, exit};
use itertools::Itertools;
use anyhow::{anyhow, Result};
use scan_fmt::scan_fmt;

type ComponentId = String;

trait Simulatable {
    fn get_id(&self) -> &ComponentId;
    
    fn next(&mut self, from: &ComponentId, input: bool) -> VecDeque<(ComponentId, ComponentId, bool)>;
}

#[derive(Debug, Clone)]
struct Passthrough {
    id: ComponentId,
    output_terminals: Vec<ComponentId>
}

impl Simulatable for Passthrough {
    fn next(&mut self, from: &ComponentId, input: bool) -> VecDeque<(ComponentId, ComponentId, bool)> {
        let mut result = VecDeque::<_>::new();
        
        for output in &self.output_terminals {
            result.push_back((self.get_id().clone(), output.clone(), input)); 
        }

        result
    }

    fn get_id(&self) -> &ComponentId {
        &self.id
    }
}

#[derive(Debug, Hash, Clone)]
struct FlipFlop {
    id: ComponentId,
    state: bool,

    output_terminals: Vec<ComponentId>,
}

impl Simulatable for FlipFlop {
    fn get_id(&self) -> &ComponentId {
        &self.id
    }

    fn next(&mut self, from: &ComponentId, input: bool) -> VecDeque<(ComponentId, ComponentId, bool)> {
        let mut result = VecDeque::<_>::new();
        
        if input == false {
            self.state = !self.state;
            for output in &self.output_terminals {
                result.push_back((self.get_id().clone(), output.clone(), self.state));
            }
        }

        result
    }
}

#[derive(Debug, Clone)]
struct Conjuction {
    id: ComponentId,
    input_memory: Vec<bool>,
    
    input_terminals_map: HashMap<ComponentId, usize>,
    output_terminals: Vec<String>,
}

impl Simulatable for Conjuction {
    fn get_id(&self) -> &ComponentId {
        &self.id
    }

    fn next(&mut self, from: &ComponentId, input: bool) -> VecDeque<(ComponentId, ComponentId, bool)> {
        let mut result = VecDeque::<_>::new();
        let sz = self.input_terminals_map[from];
        self.input_memory[sz] = input;
        
        // TODO: Maybe check if only changed then react to stimulus
        let output_pulse = !self.input_memory.iter().all(|t| *t == true);
        for output in &self.output_terminals {
            result.push_back((self.get_id().clone(), output.clone(), output_pulse));
        }

        result
    }
}

#[derive(Debug, Clone)]
enum Component {
    FlipFlop(FlipFlop),
    Conjuction(Conjuction),
    Passthrough(Passthrough)
}

type ComponentMap = HashMap<ComponentId, Component>;
type SimulatableMap = HashMap<ComponentId, Box<dyn Simulatable>>;

fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn count(outputs: &VecDeque<(ComponentId, ComponentId, bool)>) -> (i64, i64) {
    let mut result = (0, 0);
    
    for (_, _, o) in outputs {
        if *o {
            result.1 += 1;
        } else {
            result.0 += 1;
        }
    }

    result
}

fn simulate(components: &mut ComponentMap, step: i64, looking_for: &[ComponentId]) -> (i64, i64) {
    let mut result = (0, 0);
    let mut q = VecDeque::<(ComponentId, ComponentId, bool)>::new();
    q.push_back(("".to_string(), "button".to_string(), false));
    
    while !q.is_empty() {
        let (from, to, input) = q.pop_front().unwrap();
        let component = components.get_mut(&to);

        let mut neighbours = match component {
            Some(Component::FlipFlop(c)) => c.next(&from, input),
            Some(Component::Conjuction(c)) => c.next(&from, input),
            Some(Component::Passthrough(c)) => c.next(&from, input),
            None => {VecDeque::<_>::new()}
        };
        
        if looking_for.contains(&to) && !neighbours.is_empty() && neighbours[0].2 == true {
            println!("{}: {}", to, step);
        }

        result = add(result, count(&neighbours));
        q.append(&mut neighbours);
    }

    result
}

fn parse(path: &Path) -> Result<ComponentMap> {
    let mut component_map = ComponentMap::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<String>,_>>()?;

    // add the nodes
    for line in &lines {
        let (tp, name) = scan_fmt!(&line.split(" -> ").nth(0).unwrap(), "{/[%&b]/}{}", char, String)?;
        match tp {
            'b' => {
                component_map.insert(name.clone(), Component::Passthrough(Passthrough { id: name, output_terminals: vec![] }));
            },
            '%' => {
                component_map.insert(name.clone(), Component::FlipFlop(FlipFlop { id: name, state: false , output_terminals: vec![] }));
            },
            '&' => {
                component_map.insert(name.clone(), Component::Conjuction(Conjuction { id: name, input_memory: vec![], input_terminals_map: Default::default(), output_terminals: vec![] }));
            },
            _ => {
                panic!("At the nodes!");
            }
        }
    }

    // add the neighbours
    for line in &lines {

        let (tp, name) = scan_fmt!(&line.split(" -> ").nth(0).unwrap(), "{/[%&b]/}{}", char, String)?;
        let neighbours = line.split(" -> ").last().unwrap().split(", ");
        for n in neighbours {
            {
                let from = component_map.get_mut(&name).unwrap();
                match from {
                    Component::FlipFlop(c) => {
                        c.output_terminals.push(n.to_string());
                    },
                    Component::Conjuction(c) => {
                        c.output_terminals.push(n.to_string());
                    },
                    Component::Passthrough(c) => {
                        c.output_terminals.push(n.to_string());
                    },
                }
            }
            {
                let to = component_map.get_mut(&n.to_string());
                match to {
                    Some(Component::Conjuction(c)) => {
                        c.input_memory.push(false);
                        c.input_terminals_map.insert(name.clone(), c.input_memory.len() - 1);
                    },
                    _ => {}
                }
            }
        }
    }
    component_map.insert("button".to_string(), Component::Passthrough(Passthrough { id: "button".to_string(), output_terminals: vec!["roadcaster".to_string()] }));
    
    anyhow::Ok(component_map)
}

fn main() -> Result<()> {
    let mut cp = parse(&Path::new("input.txt"))?;
    let mut cp2 = cp.clone();

    let mut result = (0, 0);
    for _ in 0..1000 {
        result = add(result, simulate(&mut cp, 0, &vec![]));
    }
    println!("{:?}, {}", result, result.0 * result.1);
    
    let mut i = 1;
    let looking_for = vec!["sg".to_string(), "lm".to_string(), "db".to_string(), "dh".to_string()];
    loop {
        simulate(&mut cp2, i, &looking_for);
        i += 1;
    }

    anyhow::Ok(())
}
