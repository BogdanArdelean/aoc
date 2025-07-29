use std::default;
use std::fs::File;
use std::io::{BufRead, self, BufReader};
use std::path::Path;
use scan_fmt::scan_fmt;

fn get_minimum_bucket(input: &str) -> (i32, i32, i32) {
    let rounds = input.split(": ").last().expect("No rounds available in game");
    let mut minimal_cube = (0, 0, 0);

    for round in rounds.split("; ") {
        let mut cubes = (0, 0, 0);
        
        for cube_count in round.split(", ") {
            let count: i32 = cube_count.split(" ").nth(0).expect("Could not find cube nr").parse().expect("Could not parse number");
            let cube = cube_count.split(" ").last().expect("Could not find cube type");

            match cube {
                "red" => cubes.0 += count,
                "green"  => cubes.1 += count,
                "blue" => cubes.2 += count,
                default => {
                    panic!("What is {}", cube);
                }
            };
        }

        minimal_cube = (minimal_cube.0.max(cubes.0), minimal_cube.1.max(cubes.1), minimal_cube.2.max(cubes.2));
    }
    println!("Minimal cube: {:?}", minimal_cube);
    minimal_cube
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let reference_cube = (12, 13, 14);

    let result = reader
    .lines()
    .collect::<Result<Vec<String>, _>>()?
    .iter()
    .enumerate()
    .map(|(game, line)| (game, get_minimum_bucket(line)))
    .map(|(game, minimal_bucket)| {
        let power = minimal_bucket.0 * minimal_bucket.1 * minimal_bucket.2;
        if (minimal_bucket.0 <= reference_cube.0 && minimal_bucket.1 <= reference_cube.1 && minimal_bucket.2 <= reference_cube.2) {
            (game + 1, power)
        } else {
            (0, power)
        }
    })
    .reduce(|acc, elm| {
        (acc.0 + elm.0, acc.1 + elm.1)
    }).expect("Error at reducing");
    
    println!("Part 1: {}, Part 2: {}", result.0, result.1);
    Ok(())
}
