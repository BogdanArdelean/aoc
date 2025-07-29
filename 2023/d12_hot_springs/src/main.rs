use std::default;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::{anyhow, Result, Ok};

fn check_at(spring_map: &str, idx: i128) -> bool {
    if idx < 0 {
        return true;
    }

    if idx >= spring_map.len() as i128 {
        return true;
    }

    spring_map.chars().nth(idx as usize).unwrap() != '#'
}

fn get_dp(dp: &[Vec<i128>], i: i128, j: i128) -> i128 {
    if i < 0 {
        return 1;
    }
    if i == 0 && j < 0 {
        return 1;
    }
    *dp.get(i as usize).and_then(|row| row.get(j as usize)).unwrap_or(&0)
}

fn calculate_possibilities(spring_map: &str, groups: &[i128]) -> i128 {
    
    let mut dp = Vec::<Vec<i128>>::new();
    dp.resize_with(groups.len() + 1, || {
        let mut v = Vec::<i128>::new();
        v.resize(spring_map.len(), 0);
        v
    });

    // init first row
    {
        let mut not_found_group = true;
        for i in 0..dp[0].len() {
            if spring_map.chars().nth(i).unwrap() == '#' {
                not_found_group = false;
            }
            dp[0][i] = not_found_group as i128;
        }
    }

    // consecutive left side ?,#
    let mut consecutive_left = Vec::<i128>::new();
    consecutive_left.resize(spring_map.len(), 0);
    let mut sum = 0;    {
        for (i, chr) in spring_map.char_indices() {
            if chr == '.' {
                sum = 0;
            } else {
                sum += 1;
            }
            consecutive_left[i] = sum;
        }
    }
    
    for i in 1..dp.len() {
        let group_size = groups[i - 1];
        for j in 0..dp[i].len() {
            let group_in_between = check_at(spring_map, j as i128 + 1) && check_at(spring_map, j as i128 - group_size);
            let has_enough_consecutive = consecutive_left[j] >= group_size;
            let chr = spring_map.chars().nth(j).unwrap();
            match chr {
                '#' => {
                    dp[i][j] = ((group_in_between && has_enough_consecutive) as i128) * get_dp(dp.as_slice(), i as i128 - 1, j as i128 - group_size - 1);
                },
                '?' => {
                    dp[i][j] = get_dp(dp.as_slice(), i as i128, j as i128 - 1) + 
                      ((group_in_between && has_enough_consecutive) as i128) * get_dp(dp.as_slice(), i as i128 - 1, j as i128 - group_size - 1);
                },
                '.' => {
                    dp[i][j] = get_dp(dp.as_slice(), i as i128, j as i128 - 1);
                },
                _ => {
                    panic!("Panic at the disco!");
                }
            }
        }
    }
    
    *dp.last().unwrap().last().unwrap()
}

fn parse(path: &Path) -> Result<Vec<(String, Vec<i128>)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut res = Vec::<_>::new();
    for line in reader.lines() {
        let line = line?;
        let mut groups = Vec::<_>::new();
        
        let str = line.split(" ").nth(0).ok_or(anyhow!("Expected pattern"))?.to_string();
        for grp in line.split(" ").last().ok_or(anyhow!("Expected groups"))?.split(",") {
            groups.push(grp.parse()?);
        }

        res.push((str, groups));
    }

    Ok(res)
}

fn main() -> Result<()> {
    let mut v = parse(Path::new("input.txt"))?;
    let mut sum = 0;
    for (pat, grp) in &v {
        let res = calculate_possibilities(pat, grp);
        // println!("Pattern {}, Groups {:?} -> {}", pat, grp, res);
        sum += res;

    }
    println!("Part 1: {}", sum);

    for (pat, grp) in &mut v {
        let original_pat = pat.clone();
        for _ in 0..4 {
            *pat = (pat.clone() + "?" + &original_pat.clone()).to_string(); 
        }
        *grp = grp.repeat(5);
    }

    let mut sum = 0;
    for (pat, grp) in &v {
        let res = calculate_possibilities(pat, grp);
        // println!("Pattern {}, Groups {:?} -> {}", pat, grp, res);
        sum += res;

    }
    println!("Part 2: {}", sum);
    Ok(())
}

/*
    10000000000
    ?###..#?#?.
    00011111111
    00000011222
    
    valid(group_nr, at_idx) = 1, at_idx - group_size[group_nr] != '#' 
                                 and consecutive_defect[at_idx] > group_size[group_nr]
                                 and valid(group_nr - 1, at_idx - group_size[group_nr]),

    count(group_nr, ends_at_idx) = count(group_nr - 1, ends_at_idx - group_size[group_nr]) * valid()   
*/
