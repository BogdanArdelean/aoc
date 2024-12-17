use anyhow::Result;
use scan_fmt::scan_fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse(path: &Path) -> Result<(Processor, Vec<i64>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut regs = [0; 3];
    for i in 0..3 {
        let s = lines.next().unwrap()?;
        let (_, r) = scan_fmt!(&s, "Register {}: {}", char, i64)?;
        regs[i] = r;
    }
    let proc = Processor {
        ip: 0,
        A: regs[0],
        B: regs[1],
        C: regs[2],
    };

    lines.next();
    let instr_str = lines.next().unwrap().unwrap();
    let instructions = instr_str
        .split(" ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    Ok((proc, instructions))
}

#[derive(Debug, Clone, Default)]
struct Processor {
    ip: usize,
    A: i64,
    B: i64,
    C: i64,
}

impl Processor {
    fn with_a(A: i64) -> Self {
        Processor {
            ip: 0,
            A,
            B: 0,
            C: 0,
        }
    }

    fn get_operand_value(&self, i: i64, op: i64) -> i64 {
        match i {
            1 | 3 | 4 => op,
            _ if op == 4 => self.A,
            _ if op == 5 => self.B,
            _ if op == 6 => self.C,
            _ => op,
        }
    }
    fn halted(&self, instructions: &Vec<i64>) -> bool {
        self.ip >= instructions.len()
    }

    fn step(&mut self, instructions: &Vec<i64>) -> Option<i64> {
        assert!(!self.halted(instructions));

        let mut out = None;
        let i       = instructions[self.ip];
        let op      = instructions[self.ip + 1];
        let value   = self.get_operand_value(i, op);

        self.ip += 2;
        match i {
            0 => self.A  = self.A >> value,
            1 => self.B  = self.B ^ value,
            2 => self.B  = value & 7,
            3 => self.ip = if self.A != 0 { value as usize } else { self.ip },
            4 => self.B  = self.B ^ self.C,
            5 => out     = Some(value & 7),
            6 => self.B  = self.A >> value,
            7 => self.C  = self.A >> value,
            _ => panic!(),
        }

        out
    }

    fn execute(&mut self, instructions: &Vec<i64>) -> Vec<i64> {
        let mut output = vec![];
        while !self.halted(instructions) {
            if let Some(o) = self.step(instructions) {
                output.push(o)
            }
        }
        output
    }
}
fn part1(proc: &mut Processor, instructions: &Vec<i64>) {
    let r = proc.execute(instructions);
    print!("Part 1: ");
    for o in r {
        print!("{},", o);
    }
    println!();
}

fn find_quine(A: i64, idx: i64, instr: &Vec<i64>) -> Option<i64> {
    if idx as usize >= instr.len() {
        // check if processor produces the expected length
        if Processor::with_a(A).execute(instr).len() == instr.len() {
            // if so, it is guaranteed that it produced a quine
            return Some(A);
        }
        return None;
    }

    let A = A << 3;
    for bits in 0..8 {
        let res = Processor::with_a(A | bits).execute(instr);
        let target = instr[idx as usize];

        if res[res.len() - (instr.len() - idx as usize)] != target {
            continue;
        }
        if let Some(find) = find_quine(A | bits, idx - 1, instr) {
            return Some(find);
        }
    }

    None
}

fn part2(instructions: &Vec<i64>) {
    let len = instructions.len() as i64 - 1;
    if let Some(f) = find_quine(0, len, instructions) {
        println!("Part 2: {}", f);
        return;
    }
    println!("Not found!");
}

fn main() -> Result<()> {
    let (mut proc, instructions) = parse(Path::new("input.txt"))?;

    part1(&mut proc, &instructions);
    part2(&instructions);
    Ok(())
}
