use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone)]
struct Claw {
    a: (i64, i64),
    b: (i64, i64),
    target: (i64, i64),
}

fn parse(path: &Path) -> Result<Vec<Claw>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    let mut res = vec![];
    for line in input.split("\n\n") {
        let (a0, a1, b0, b1, t0, t1) = scan_fmt::scan_fmt!(
            line,
            "Button A: X+{}, Y+{}\nButton B: X+{}, Y+{}\nPrize: X={}, Y={}",
            i64,
            i64,
            i64,
            i64,
            i64,
            i64
        )?;

        res.push(Claw {
            a: (a0, a1),
            b: (b0, b1),
            target: (t0, t1),
        });
    }
    Ok(res)
}

fn calculate_cost(c: &Claw) -> i64 {
    let mn = (c.target.0 / c.b.0).min(c.target.1 / c.b.1);

    for i in (1..=mn).rev() {
        let x_rem = c.target.0 - (c.b.0 * i);
        let y_rem = c.target.1 - (c.b.1 * i);
        let a = x_rem / c.a.0;
        if x_rem % c.a.0 == 0 && y_rem % c.a.1 == 0 && a == y_rem / c.a.1 {
            return i + a * 3;
        }
    }
    0
}

fn calculate_cost_2(c: &Claw) -> i64 {
    let tx = c.target.0;
    let ty = c.target.1;
    let bx = c.b.0;
    let by = c.b.1;
    let ax = c.a.0;
    let ay = c.a.1;
    assert_ne!(ax * by - ay * bx, 0);

    let B = (ty * ax - ay * tx) / (ax * by - ay * bx);
    let A = (tx - B * bx) / ax;

    if A * ax + B * bx == tx && A * ay + B * by == ty {
        return B + 3 * A;
    }

    return 0;
}
fn part1(claws: &Vec<Claw>) -> i64 {
    let mut sum = 0;
    for c in claws {
        sum += calculate_cost(c);
    }
    sum
}

fn part2(claws: &Vec<Claw>) -> i64 {
    let mut sum = 0;
    for c in claws {
        let claw_big = Claw {
            a: c.a,
            b: c.b,
            target: (c.target.0 + 10000000000000, c.target.1 + 10000000000000),
        };
        sum += calculate_cost_2(&claw_big);
    }
    sum
}

fn main() -> Result<()> {
    let claws = parse(Path::new("input.txt"))?;
    println!("Part 1 {}", part1(&claws));

    println!("Part 2 {}", part2(&claws));
    Ok(())
}
