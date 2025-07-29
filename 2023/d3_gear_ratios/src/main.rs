use std::collections::HashSet;
use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug, Clone, Copy)]
enum EngineMapEntry {
    None,
    PartNumber(i32, i32),
    PartType(char)
}

struct EngineMap {
    map: Vec<Vec<EngineMapEntry>>,
    part_types: Vec<(char, isize, isize)>,
}

impl EngineMap {
    fn parse(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut map = Vec::<Vec<EngineMapEntry>>::new();
        let mut part_types = Vec::<(char, isize, isize)>::new();
        
        let mut parts_seq_nr = 0;
        for (x, line) in reader.lines().enumerate() {
            let line = line?;
            let mut engine_map_line = Vec::<EngineMapEntry>::new();
            
            let mut digit_length = 0;
            let mut number: i32 = 0;
            for (y, char) in line.chars().enumerate() {
                if let Some(d) = char.to_digit(10) {
                    digit_length += 1;
                    number = number * 10 + d as i32;
                    continue;
                }

                match char {
                    '.' => {
                        if digit_length > 0 {
                            parts_seq_nr += 1;
                        }
                        while digit_length > 0 {
                            engine_map_line.push(EngineMapEntry::PartNumber(number, parts_seq_nr));
                            digit_length -= 1;
                        }
                        engine_map_line.push(EngineMapEntry::None);
                        number = 0;
                    }
                    default => {
                        if digit_length > 0 {
                            parts_seq_nr += 1;
                        }
                        while digit_length > 0 {
                            engine_map_line.push(EngineMapEntry::PartNumber(number, parts_seq_nr));
                            digit_length -= 1;
                        }
                        engine_map_line.push(EngineMapEntry::PartType(default));
                        number = 0;
                        part_types.push((default, x as isize, y as isize));
                    }
                }
            }
            if digit_length > 0 {
                parts_seq_nr += 1;
            }
            while digit_length > 0 {
                engine_map_line.push(EngineMapEntry::PartNumber(number, parts_seq_nr));
                digit_length -= 1;
            }

            map.push(engine_map_line);
        }
        
        Ok(EngineMap {
            map,
            part_types
        })
    }

    fn get(&self, x: isize, y: isize) -> Option<EngineMapEntry> {
        println!("N: {}, {}", x, y);
        if x < 0 || y < 0 {
            return None;
        }

        if let Some(line) = self.map.get(x as usize) {
            line.get(y as usize).copied()
        } else {
            None
        }
    }

    fn sum_parts(&self) -> i32 {
        let dxdy = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        let mut sum = 0;
        let mut hset = HashSet::<(i32, char)>::new();

        for part_type in &self.part_types {
            println!("pt: {}, {}, {}", part_type.0, part_type.1, part_type.2);

            for pos in &dxdy {
                sum += if let Some(EngineMapEntry::PartNumber(pn, seq_nr)) = self.get(part_type.1 + pos.0, part_type.2 + pos.1) {
                    if hset.contains(&(seq_nr, part_type.0)) {
                        println!("eh");
                        0
                    } else {
                        println!("{}, {}", pn, seq_nr);
                        hset.insert((seq_nr, part_type.0));
                        pn
                    }
                } else {
                    0
                }
            }
        }
        sum
    }

    fn sum_gears(&self) -> i32 {
        let dxdy = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        let mut sum = 0;
        let mut hset = HashSet::<(i32, char)>::new();

        for part_type in &self.part_types {
            if part_type.0 != '*' {
                continue;
            }

            let mut aux_prod = 1;
            let mut neighbours = 0;
            for pos in &dxdy {
                aux_prod *= if let Some(EngineMapEntry::PartNumber(pn, seq_nr)) = self.get(part_type.1 + pos.0, part_type.2 + pos.1) {
                    if hset.contains(&(seq_nr, part_type.0)) {
                        1
                    } else {
                        hset.insert((seq_nr, part_type.0));
                        neighbours += 1;
                        pn
                    }
                } else {
                    1
                }
            }

            if neighbours == 2 {
                sum += aux_prod;
            }
        }
        sum
    }
}

fn main() -> Result<(), std::io::Error> {
    let em = EngineMap::parse(Path::new("input.txt"))?;
    
    println!("Sum parts: {} Sum gears: {}", em.sum_parts(), em.sum_gears());

    Ok(())
}
