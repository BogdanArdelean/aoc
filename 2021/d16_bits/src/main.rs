use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;
use bitvec::macros::internal::funty::Integral;
use bitvec::prelude::*;

struct BitVecReader<'a, T, O>
where
    T: BitStore,
    O: BitOrder
{
    inner: &'a BitVec<T, O>,
    pos: usize
}

impl<'a, T, O> BitVecReader<'a, T, O>
where
    T: BitStore,
    O: BitOrder {

    fn new(inner: &'a BitVec<T, O>) -> BitVecReader<'a, T, O> {
        Self {
            inner,
            pos: 0,
        }
    }
}

impl<'a, T> BitVecReader<'a, T, Msb0>
where
    T: BitStore {

    fn load<I>(&mut self, size: usize) -> I
    where
        I: Integral {
        self.pos += size;
        self.inner[self.pos - size..self.pos].load_be()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal
}

impl Operation {
    fn from(type_id: i64) -> Operation {
        match type_id {
            0 => Operation::Sum,
            1 => Operation::Product,
            2 => Operation::Minimum,
            3 => Operation::Maximum,
            5 => Operation::Greater,
            6 => Operation::Less,
            7 => Operation::Equal,
            _ => panic!("At operation")
        }
    }

    fn apply(&self, packets: &[Packet]) -> i64 {
        match self {
            Operation::Sum => {
                packets.iter().map(|x| x.calculate()).sum()
            }
            Operation::Product => {
                packets.iter().map(|x| x.calculate()).product()
            }
            Operation::Minimum => {
                packets.iter().map(|x| x.calculate()).min().unwrap()
            }
            Operation::Maximum => {
                packets.iter().map(|x| x.calculate()).max().unwrap()
            }
            Operation::Greater => {
                assert_eq!(packets.len(), 2);
                let a = packets[0].calculate();
                let b = packets[1].calculate();
                (a > b) as i64
            }
            Operation::Less => {
                assert_eq!(packets.len(), 2);
                let a = packets[0].calculate();
                let b = packets[1].calculate();
                (a < b) as i64
            }
            Operation::Equal => {
                assert_eq!(packets.len(), 2);
                let a = packets[0].calculate();
                let b = packets[1].calculate();
                (a == b) as i64
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Length {
    InBits(i64),
    InPackets(i64)
}
#[derive(Debug, Clone, Eq, PartialEq)]
enum PacketType {
    Literal(i64),
    Operator(Operation, Length, Vec<Packet>)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet {
    version: i64,
    packet_type: PacketType
}

impl Packet {
    fn parse_literal(wrapper: &mut BitVecReader<u8, Msb0>) -> i64 {
        let mut literal: u64 = 0;
        loop {
            let chunk: u64 = wrapper.load(5);
            literal = (literal << 4) | (chunk & 0xF);

            if (chunk & (1 << 4)) == 0 {
                break;
            }
        }

        literal as i64
    }
    fn parse(wrapper: &mut BitVecReader<u8, Msb0>) -> Packet {
        let version: u32 = wrapper.load(3);
        let type_id: u32 = wrapper.load(3);

        let packet_type = match type_id {
            4 => PacketType::Literal(Self::parse_literal(wrapper)),
            t => {
                let type_id_length: u32 = wrapper.load(1);
                let length = if type_id_length == 0 {
                    let bts: u32 = wrapper.load(15);
                    Length::InBits(bts as i64)
                } else {
                    let packets: u32 = wrapper.load(11);
                    Length::InPackets(packets as i64)
                };

                let mut packets = vec![];
                match &length {
                    Length::InBits(bts) => {
                        let pos = wrapper.pos;
                        while wrapper.pos - pos < *bts as usize {
                            packets.push(Self::parse(wrapper));
                        }
                    }
                    Length::InPackets(nr) => {
                        for _ in 0..*nr {
                            packets.push(Self::parse(wrapper));
                        }
                    }
                }
                PacketType::Operator(Operation::from(t as i64), length, packets)
            }
        };

        Packet {
            version: version as i64,
            packet_type
        }
    }

    fn add_versions(&self) -> i64 {
        let sum = match &self.packet_type {
            PacketType::Literal(_) => { self.version }
            PacketType::Operator(_, _, p) => { self.version + p.iter().map(Self::add_versions).sum::<i64>()}
        };

        sum
    }

    fn calculate(&self) -> i64 {
        match &self.packet_type {
            PacketType::Literal(l) => {
                *l
            }
            PacketType::Operator(op, _, p) => {
                op.apply(p)
            }
        }
    }
}

fn parse(path: &Path) -> Result<BitVec<u8, Msb0>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;

    let bytes: Vec<u8> = line
        .as_bytes()
        .chunks(2) // Each byte is represented by 2 hex characters
        .map(|chunk| {
            let hex_str = std::str::from_utf8(chunk).expect("Invalid UTF-8 in hex string");
            u8::from_str_radix(hex_str, 16).expect("Invalid hex character")
        })
        .collect();

    // Load the bytes into a BitVec with MSB-first ordering
    Ok(BitVec::from_vec(bytes))
}

fn main() -> Result<()> {
    let bv = parse(&Path::new("input.txt"))?;
    let pack = Packet::parse(&mut BitVecReader::new(&bv));
    println!("Part 1 {}", pack.add_versions());
    println!("Part 2 {}", pack.calculate());
    Ok(())
}
