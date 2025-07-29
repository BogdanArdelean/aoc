use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Result;

/*
    Sorry for the hardcoded map. Already had BFS implemented elsewhere and I generated the min paths.
 */
fn all_paths(start: char, end: char) -> Vec<String> {
    let paths = HashMap::from([
        (('0', '0'), vec![String::from("A")]),
        (('0', 'A'), vec![String::from(">A")]),
        (('0', '1'), vec![String::from("^<A")]),
        (('0', '2'), vec![String::from("^A")]),
        (('0', '3'), vec![String::from(">^A"), String::from("^>A")]),
        (('0', '4'), vec![String::from("^<^A"), String::from("^^<A")]),
        (('0', '5'), vec![String::from("^^A")]),
        (('0', '6'), vec![String::from(">^^A"), String::from("^>^A"), String::from("^^>A")]),
        (('0', '7'), vec![String::from("^<^^A"), String::from("^^<^A"), String::from("^^^<A")]),
        (('0', '8'), vec![String::from("^^^A")]),
        (('0', '9'), vec![String::from(">^^^A"), String::from("^>^^A"), String::from("^^>^A"), String::from("^^^>A")]),
        (('A', '0'), vec![String::from("<A")]),
        (('A', 'A'), vec![String::from("A")]),
        (('A', '1'), vec![String::from("<^<A"), String::from("^<<A")]),
        (('A', '2'), vec![String::from("<^A"), String::from("^<A")]),
        (('A', '3'), vec![String::from("^A")]),
        (('A', '4'), vec![String::from("<^<^A"), String::from("<^^<A"), String::from("^<<^A"), String::from("^<^<A"), String::from("^^<<A")]),
        (('A', '5'), vec![String::from("<^^A"), String::from("^<^A"), String::from("^^<A")]),
        (('A', '6'), vec![String::from("^^A")]),
        (('A', '7'), vec![String::from("<^<^^A"), String::from("<^^<^A"), String::from("<^^^<A"), String::from("^<<^^A"), String::from("^<^<^A"), String::from("^<^^<A"), String::from("^^<<^A"), String::from("^^<^<A"), String::from("^^^<<A")]),
        (('A', '8'), vec![String::from("<^^^A"), String::from("^<^^A"), String::from("^^<^A"), String::from("^^^<A")]),
        (('A', '9'), vec![String::from("^^^A")]),
        (('1', '0'), vec![String::from(">vA")]),
        (('1', 'A'), vec![String::from(">>vA"), String::from(">v>A")]),
        (('1', '1'), vec![String::from("A")]),
        (('1', '2'), vec![String::from(">A")]),
        (('1', '3'), vec![String::from(">>A")]),
        (('1', '4'), vec![String::from("^A")]),
        (('1', '5'), vec![String::from(">^A"), String::from("^>A")]),
        (('1', '6'), vec![String::from(">>^A"), String::from(">^>A"), String::from("^>>A")]),
        (('1', '7'), vec![String::from("^^A")]),
        (('1', '8'), vec![String::from(">^^A"), String::from("^>^A"), String::from("^^>A")]),
        (('1', '9'), vec![String::from(">>^^A"), String::from(">^>^A"), String::from(">^^>A"), String::from("^>>^A"), String::from("^>^>A"), String::from("^^>>A")]),
        (('2', '0'), vec![String::from("vA")]),
        (('2', 'A'), vec![String::from(">vA"), String::from("v>A")]),
        (('2', '1'), vec![String::from("<A")]),
        (('2', '2'), vec![String::from("A")]),
        (('2', '3'), vec![String::from(">A")]),
        (('2', '4'), vec![String::from("<^A"), String::from("^<A")]),
        (('2', '5'), vec![String::from("^A")]),
        (('2', '6'), vec![String::from(">^A"), String::from("^>A")]),
        (('2', '7'), vec![String::from("<^^A"), String::from("^<^A"), String::from("^^<A")]),
        (('2', '8'), vec![String::from("^^A")]),
        (('2', '9'), vec![String::from(">^^A"), String::from("^>^A"), String::from("^^>A")]),
        (('3', '0'), vec![String::from("<vA"), String::from("v<A")]),
        (('3', 'A'), vec![String::from("vA")]),
        (('3', '1'), vec![String::from("<<A")]),
        (('3', '2'), vec![String::from("<A")]),
        (('3', '3'), vec![String::from("A")]),
        (('3', '4'), vec![String::from("<<^A"), String::from("<^<A"), String::from("^<<A")]),
        (('3', '5'), vec![String::from("<^A"), String::from("^<A")]),
        (('3', '6'), vec![String::from("^A")]),
        (('3', '7'), vec![String::from("<<^^A"), String::from("<^<^A"), String::from("<^^<A"), String::from("^<<^A"), String::from("^<^<A"), String::from("^^<<A")]),
        (('3', '8'), vec![String::from("<^^A"), String::from("^<^A"), String::from("^^<A")]),
        (('3', '9'), vec![String::from("^^A")]),
        (('4', '0'), vec![String::from(">vvA"), String::from("v>vA")]),
        (('4', 'A'), vec![String::from(">>vvA"), String::from(">v>vA"), String::from(">vv>A"), String::from("v>>vA"), String::from("v>v>A")]),
        (('4', '1'), vec![String::from("vA")]),
        (('4', '2'), vec![String::from(">vA"), String::from("v>A")]),
        (('4', '3'), vec![String::from(">>vA"), String::from(">v>A"), String::from("v>>A")]),
        (('4', '4'), vec![String::from("A")]),
        (('4', '5'), vec![String::from(">A")]),
        (('4', '6'), vec![String::from(">>A")]),
        (('4', '7'), vec![String::from("^A")]),
        (('4', '8'), vec![String::from(">^A"), String::from("^>A")]),
        (('4', '9'), vec![String::from(">>^A"), String::from(">^>A"), String::from("^>>A")]),
        (('5', '0'), vec![String::from("vvA")]),
        (('5', 'A'), vec![String::from(">vvA"), String::from("v>vA"), String::from("vv>A")]),
        (('5', '1'), vec![String::from("<vA"), String::from("v<A")]),
        (('5', '2'), vec![String::from("vA")]),
        (('5', '3'), vec![String::from(">vA"), String::from("v>A")]),
        (('5', '4'), vec![String::from("<A")]),
        (('5', '5'), vec![String::from("A")]),
        (('5', '6'), vec![String::from(">A")]),
        (('5', '7'), vec![String::from("<^A"), String::from("^<A")]),
        (('5', '8'), vec![String::from("^A")]),
        (('5', '9'), vec![String::from(">^A"), String::from("^>A")]),
        (('6', '0'), vec![String::from("<vvA"), String::from("v<vA"), String::from("vv<A")]),
        (('6', 'A'), vec![String::from("vvA")]),
        (('6', '1'), vec![String::from("<<vA"), String::from("<v<A"), String::from("v<<A")]),
        (('6', '2'), vec![String::from("<vA"), String::from("v<A")]),
        (('6', '3'), vec![String::from("vA")]),
        (('6', '4'), vec![String::from("<<A")]),
        (('6', '5'), vec![String::from("<A")]),
        (('6', '6'), vec![String::from("A")]),
        (('6', '7'), vec![String::from("<<^A"), String::from("<^<A"), String::from("^<<A")]),
        (('6', '8'), vec![String::from("<^A"), String::from("^<A")]),
        (('6', '9'), vec![String::from("^A")]),
        (('7', '0'), vec![String::from(">vvvA"), String::from("v>vvA"), String::from("vv>vA")]),
        (('7', 'A'), vec![String::from(">>vvvA"), String::from(">v>vvA"), String::from(">vv>vA"), String::from(">vvv>A"), String::from("v>>vvA"), String::from("v>v>vA"), String::from("v>vv>A"), String::from("vv>>vA"), String::from("vv>v>A")]),
        (('7', '1'), vec![String::from("vvA")]),
        (('7', '2'), vec![String::from(">vvA"), String::from("v>vA"), String::from("vv>A")]),
        (('7', '3'), vec![String::from(">>vvA"), String::from(">v>vA"), String::from(">vv>A"), String::from("v>>vA"), String::from("v>v>A"), String::from("vv>>A")]),
        (('7', '4'), vec![String::from("vA")]),
        (('7', '5'), vec![String::from(">vA"), String::from("v>A")]),
        (('7', '6'), vec![String::from(">>vA"), String::from(">v>A"), String::from("v>>A")]),
        (('7', '7'), vec![String::from("A")]),
        (('7', '8'), vec![String::from(">A")]),
        (('7', '9'), vec![String::from(">>A")]),
        (('8', '0'), vec![String::from("vvvA")]),
        (('8', 'A'), vec![String::from(">vvvA"), String::from("v>vvA"), String::from("vv>vA"), String::from("vvv>A")]),
        (('8', '1'), vec![String::from("<vvA"), String::from("v<vA"), String::from("vv<A")]),
        (('8', '2'), vec![String::from("vvA")]),
        (('8', '3'), vec![String::from(">vvA"), String::from("v>vA"), String::from("vv>A")]),
        (('8', '4'), vec![String::from("<vA"), String::from("v<A")]),
        (('8', '5'), vec![String::from("vA")]),
        (('8', '6'), vec![String::from(">vA"), String::from("v>A")]),
        (('8', '7'), vec![String::from("<A")]),
        (('8', '8'), vec![String::from("A")]),
        (('8', '9'), vec![String::from(">A")]),
        (('9', '0'), vec![String::from("<vvvA"), String::from("v<vvA"), String::from("vv<vA"), String::from("vvv<A")]),
        (('9', 'A'), vec![String::from("vvvA")]),
        (('9', '1'), vec![String::from("<<vvA"), String::from("<v<vA"), String::from("<vv<A"), String::from("v<<vA"), String::from("v<v<A"), String::from("vv<<A")]),
        (('9', '2'), vec![String::from("<vvA"), String::from("v<vA"), String::from("vv<A")]),
        (('9', '3'), vec![String::from("vvA")]),
        (('9', '4'), vec![String::from("<<vA"), String::from("<v<A"), String::from("v<<A")]),
        (('9', '5'), vec![String::from("<vA"), String::from("v<A")]),
        (('9', '6'), vec![String::from("vA")]),
        (('9', '7'), vec![String::from("<<A")]),
        (('9', '8'), vec![String::from("<A")]),
        (('9', '9'), vec![String::from("A")]),
        (('<', '<'), vec![String::from("A")]),
        (('<', 'v'), vec![String::from(">A")]),
        (('<', '>'), vec![String::from(">>A")]),
        (('<', '^'), vec![String::from(">^A")]),
        (('<', 'A'), vec![String::from(">>^A"), String::from(">^>A")]),
        (('v', '<'), vec![String::from("<A")]),
        (('v', 'v'), vec![String::from("A")]),
        (('v', '>'), vec![String::from(">A")]),
        (('v', '^'), vec![String::from("^A")]),
        (('v', 'A'), vec![String::from(">^A"), String::from("^>A")]),
        (('>', '<'), vec![String::from("<<A")]),
        (('>', 'v'), vec![String::from("<A")]),
        (('>', '>'), vec![String::from("A")]),
        (('>', '^'), vec![String::from("<^A"), String::from("^<A")]),
        (('>', 'A'), vec![String::from("^A")]),
        (('^', '<'), vec![String::from("v<A")]),
        (('^', 'v'), vec![String::from("vA")]),
        (('^', '>'), vec![String::from(">vA"), String::from("v>A")]),
        (('^', '^'), vec![String::from("A")]),
        (('^', 'A'), vec![String::from(">A")]),
        (('A', '<'), vec![String::from("<v<A"), String::from("v<<A")]),
        (('A', 'v'), vec![String::from("<vA"), String::from("v<A")]),
        (('A', '>'), vec![String::from("vA")]),
        (('A', '^'), vec![String::from("<A")]),
        (('A', 'A'), vec![String::from("A")]),
    ]);

    return paths.get(&(start, end)).unwrap().clone();
}

fn min_path_length(code: String, robots: i64, cache: &mut HashMap<(String, i64), usize>) -> usize {
    if robots == 0 {
        return code.len();
    }
    if cache.contains_key(&(code.clone(), robots)) {
        return cache.get(&(code, robots)).unwrap().clone();
    }

    let mut length = 0;
    let mut start = 'A';
    for end in code.chars() {
        let mut min_length = usize::MAX;
        for path in all_paths(start, end) {
            let path_length = min_path_length(path, robots - 1, cache);
            min_length = min_length.min(path_length);
        }
        length += min_length;
        start = end;
    }

    cache.insert((code, robots), length);
    length
}

fn code_to_number(input: &str) -> usize {
    let numeric_part: String = input.chars().filter(|c| c.is_digit(10)).collect();
    numeric_part.parse::<usize>().unwrap()
}

fn enter_code(codes: &Vec<String>, robots: i64) -> usize {
    let mut cache = HashMap::new();
    codes
        .iter()
        .map(|c| min_path_length(c.clone(), robots + 1, &mut cache))
        .zip(codes)
        .map(|(l, c)| {
            l * code_to_number(c)
        })
        .sum()
}

fn parse(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(|l| l.unwrap()).collect())
}

fn main() -> Result<()> {
    let codes = parse(&Path::new("input.txt"))?;
    let p1 = enter_code(&codes, 2);
    println!("Part 1: {}", p1);

    let p2 = enter_code(&codes, 25);
    println!("Part 2: {}", p2);
    Ok(())
}
