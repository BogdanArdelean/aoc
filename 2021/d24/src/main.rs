use anyhow::{Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::iter::Iterator;

#[derive(Debug, Clone)]
enum Register {
    X,
    Y,
    W,
    Z
}

impl Register {
    fn parse(s: &str) -> Result<Register> {
        match s {
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            "w" => Ok(Register::W),
            _ => anyhow::bail!("Register parse error"),
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Variable(Register),
    Constant(i64)
}

impl Operand {
    fn parse(s: &str) -> Result<Operand> {
        if let Ok(r) = Register::parse(s) {
            Ok(Operand::Variable(r))
        } else {
            Ok(Operand::Constant(s.parse()?))
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand)
}

impl Instruction {
    fn parse(s: &str) -> Result<Instruction> {
        let (instr, dest, op) = scan_fmt::scan_fmt_some!(s, "{} {} {}", String, String, String);
        let instr = instr.unwrap();
        let dest = dest.unwrap();
        Ok(
            match instr.as_str() {
                "inp" => Instruction::Inp(Register::parse(&dest)?),
                "add" => Instruction::Add(Register::parse(&dest)?, Operand::parse(&op.unwrap())?),
                "mul" => Instruction::Mul(Register::parse(&dest)?, Operand::parse(&op.unwrap())?),
                "div" => Instruction::Div(Register::parse(&dest)?, Operand::parse(&op.unwrap())?),
                "mod" => Instruction::Mod(Register::parse(&dest)?, Operand::parse(&op.unwrap())?),
                "eql" => Instruction::Eql(Register::parse(&dest)?, Operand::parse(&op.unwrap())?),
                _ => anyhow::bail!("Can't recognize instruction"),
            }
        )
    }
}

#[derive(Debug, Clone, Default)]
struct Alu {
    x: i64,
    y: i64,
    w: i64,
    z: i64
}

impl Alu {
    fn execute_all<'a, I>(
        &mut self, 
        instructions: &Vec<Instruction>, 
        input_iterator: &mut I) 
    where 
        I: Iterator<Item=&'a i64> {
            for i in instructions {
                self.execute(i, input_iterator);
            }
    }

    fn execute<'a, I>(
        &mut self, 
        instruction: &Instruction, 
        input_iterator: &mut I) 
    where 
        I: Iterator<Item=&'a i64> {
            let value = self.get_value(instruction);
            let destination = self.get_destination(instruction);

            match instruction {
                Instruction::Inp(_) => *destination = *input_iterator.next().unwrap(),
                Instruction::Add(_, _) => *destination += value,
                Instruction::Mul(_, _) => *destination *= value,
                Instruction::Div(_, _) => *destination /= value,
                Instruction::Mod(_, _) => *destination %= value,
                Instruction::Eql(_, _) => *destination = (*destination == value) as i64,
            }
    }

    fn get_destination(&mut self, instruction: &Instruction) -> &mut i64 {
        let register = match instruction {
            Instruction::Inp(reg) => reg,
            Instruction::Add(reg, _) => reg,
            Instruction::Mul(reg, _) => reg,
            Instruction::Div(reg, _) => reg,
            Instruction::Mod(reg, _) => reg,
            Instruction::Eql(reg, _) => reg,
        };

        match register {
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::W => &mut self.w,
            Register::Z => &mut self.z,
        }
    }

    fn get_value(&self, instruction: &Instruction) -> i64 {
        let operand = match instruction {
            Instruction::Inp(_) => return 0,
            Instruction::Add(_, op) => op,
            Instruction::Mul(_, op) => op,
            Instruction::Div(_, op) => op,
            Instruction::Mod(_, op) => op,
            Instruction::Eql(_, op) => op
        };

        match operand {
            Operand::Constant(c) => *c,
            Operand::Variable(r) => {
                match r {
                    Register::X => self.x,
                    Register::Y => self.y,
                    Register::W => self.w,
                    Register::Z => self.z
                }
            }
        }
    }
}

fn should_decrease(instr: &Vec<Instruction>) -> bool {
    let instruction = &instr[5];
    match instruction {
        Instruction::Add(_, Operand::Constant(c)) => *c < 0,
        _ => panic!("instruction not valid")
    }
}

fn part2(alu: Alu, instrs: &Vec<Vec<Instruction>>, idx: usize) -> bool {
    if idx >= instrs.len() {
        return alu.z == 0;
    }

    let instructions = &instrs[idx];

    let decrease = should_decrease(instructions);
    let z = alu.z;

    for i in 1..=9 {
        let input = vec![i];
        let mut aux_alu = alu.clone();
        let mut iter = input.iter();
        aux_alu.execute_all(instructions, &mut iter);

        if decrease {
            let z = z / 26;
            let aux_alu_z = aux_alu.z / 26;

            if ((z == 0 && aux_alu_z == 0) || (z > aux_alu_z)) && part2(aux_alu.clone(), instrs, idx + 1) {
                println!("{}", i);
                return true;
            }
        } else {
            if part2(aux_alu.clone(), instrs, idx + 1) {
                println!("{}", i);
                return true;
            }
        }
    }

    return false;
}   

fn parse(path: &Path) -> Result<Vec<Instruction>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut v = vec![];
    for line in reader.lines() {
        let line = line?;
        v.push(Instruction::parse(&line)?);
    }

    Ok(v)
}

fn test(instructions: &Vec<Instruction>) {
    //               |     | |         | | | |  
    let input = vec![1,1,9,1,1,9,9,9,9,7,5,3,6,1];
    let mut input_iter = input.iter();
    let mut alu = Alu::default();
    
    for instruction in instructions {
        alu.execute(instruction, &mut input_iter);
        println!("{:?}", alu);
    }
}

fn main() -> Result<()> {
    let instructions = parse(&Path::new("input.txt"))?;
    let chunked: Vec<Vec<Instruction>> = instructions.chunks(18).map(|c| c.to_vec()).collect();
    part2(Alu::default(), &chunked, 0);
    Ok(())
}
