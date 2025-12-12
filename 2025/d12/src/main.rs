use std::fs;

fn parse(input: &str) -> Vec<(i32, Vec<i32>)> {
    // ignore the fluff
    let mut res = Vec::new();

    let lines = input.split('\n');
    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let nums: Vec<&str> = line.split('x').collect();

        let area = nums[0].parse::<i32>().unwrap()
            * nums[1][..2].parse::<i32>().unwrap();

        let mut vals = Vec::new();
        for reqs in nums[1][4..].split(' ') {
            vals.push(reqs.parse::<i32>().unwrap());
        }

        res.push((area, vals));
    }

    res
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let reqs = parse(&input);

    let mut ok = 0;

    for (a, req) in reqs {
        let mut s = 0;
        for r in req {
            s += r * 9;
        }

        if a >= s {
            ok += 1;
        }
    }

    println!("part 1: {}", ok);
}
