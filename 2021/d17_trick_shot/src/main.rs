use std::collections::{BTreeSet, HashSet};

fn get_t_from_eq(vel: i32, target: i32) -> Option<i32> {
    let a = -0.5;
    let b = vel as f64 + 0.5;
    let c = -target as f64;
    let delta = b*b - 4.0*a*c;

    if delta < 0.0 {
        return None
    }

    let x1 = (-b + delta.sqrt()) / (2.0*a);
    let x2 = (-b - delta.sqrt()) / (2.0*a);

    return Some(x1.max(x2).floor() as i32)
}

fn get_y_pos_from_eq(vel: i32, t: i32) -> i32 {
    if t == 0 {
        return 0;
    }

    let vel = vel as f64;
    let t = t as f64;
    let y = (vel * t - t*(t-1.0)*0.5) as i32;
    return y;
}

fn get_x_pos_from_eq(vel: i32, t: i32) -> i32 {
    let max_f = (vel*(vel+1)) / 2;
    if t > vel {
        max_f
    } else {
        let vel = vel as f64;
        let t = t as f64;
        let y = (vel * t - t*(t-1.0)*0.5) as i32;
        y
    }
}

fn solve((x_min, x_max): (i32, i32), (y_min, y_max): (i32, i32)) -> i32 {
    for y_vel in (1..=200).rev() {
        if let Some(t) = get_t_from_eq(y_vel, y_max) {
            let mut t = t;
            while get_y_pos_from_eq(y_vel, t) >= y_min {
                if get_y_pos_from_eq(y_vel, t) <= y_max {
                    for x_vel in (1..=200) {
                        let x_pos = get_x_pos_from_eq(x_vel, t);
                        if x_pos <= x_max && x_pos >= x_min {
                            return get_x_pos_from_eq(y_vel, t);
                        }
                    }
                }
                t += 1;
            }
        }
    }

    0
}

fn solve2((x_min, x_max): (i32, i32), (y_min, y_max): (i32, i32)) -> i32 {
    let mut hset = BTreeSet::<(i32, i32)>::new();

    for y_vel in -10000..=10000 {
        if let Some(t) = get_t_from_eq(y_vel, y_max) {
            let mut t = t;
            while get_y_pos_from_eq(y_vel, t) >= y_min {
                if get_y_pos_from_eq(y_vel, t) <= y_max {
                    for x_vel in 1..=10000 {
                        let x_pos = get_x_pos_from_eq(x_vel, t);
                        if x_pos <= x_max && x_pos >= x_min {
                            hset.insert((x_vel, y_vel));
                        }
                    }
                }
                t += 1;
            }
        }
    }

    hset.len() as i32
}

fn main() {
    let t = get_t_from_eq(2, -7).unwrap_or(0);
    let mx = solve((235, 280), (-73, -46));
    let all = solve2((253, 280), (-73, -46));
    let all2 = solve2((20, 30), (-10, -5));
    println!("t {}, mx {}, all {}", t, mx, all);
    println!("{}", get_y_pos_from_eq(4, 5));
    println!("{}", all2);
}
