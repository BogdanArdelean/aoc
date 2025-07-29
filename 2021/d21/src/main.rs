fn wrap_dice(dice: i32) -> i32 {
    if dice % 100 == 0 {
        100
    } else {
        dice % 100
    }
}

fn step(p: i32, score: i32, dice: i32) -> (i32, i32, i32) {
    let offset = wrap_dice(dice + 1) + wrap_dice(dice + 2) + wrap_dice(dice + 3);
    let p = wrap_pos(p + offset);
    let score = score + p;
    let dice = wrap_dice(dice + 3);

    (p, score, dice)
}
fn simulate(mut p1: i32, mut p2: i32) -> (i32, i32){
    let mut dice = 0;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut steps = 0;
    let loser = loop {
        (p1, p1_score, dice) = step(p1, p1_score, dice);
        steps += 3;
        if p1_score >= 1000 {
            break p2_score;
        }

        (p2, p2_score, dice) = step(p2, p2_score, dice);
        steps += 3;
        if p2_score >= 1000 {
            break p1_score;
        }
    };
    (loser, steps)
}
fn wrap_pos(pos: i32) -> i32 {
    if pos % 10 == 0 {
        10
    } else {
        pos % 10
    }
}
fn simulate_player(state: &[[i128; 22];11]) -> ([[i128; 22];11], i128, i128) {
    let clamp = |s: i32| s.min(21);
    let mut state_aux = [[0i128; 22];11];
    for i in 0..11 {
        for j in 0..21 {
            for d1 in 1..=3 {
                for d2 in 1..=3 {
                    for d3 in 1..=3 {
                        let dd = d1 + d2 + d3;
                        let ii = wrap_pos(i + dd) as usize;
                        let jj = clamp(j + wrap_pos(i + dd)) as usize;
                        state_aux[ii][jj] += state[i as usize][j as usize];
                    }
                }
            }
        }
    }

    let mut winning = 0;
    for i in 1..11 {
        winning += state_aux[i][21];
        state_aux[i][21] = 0;
    }

    let mut not_winning = 0;
    for i in 1..11 {
        for j in 1..21 {
            not_winning += state_aux[i][j];
        }
    }

    (state_aux, winning, not_winning)
}

fn simulate_dp(p1: i32, p2: i32) -> i128 {
    let mut p1_dp = [[0i128; 22];11]; p1_dp[p1 as usize][0] = 1;
    let mut p2_dp = [[0i128; 22];11]; p2_dp[p2 as usize][0] = 1;
    let mut p1_total: i128 = 0;
    let mut p2_total: i128 = 0;
    let mut p1_last_non_winning = 1;
    let mut p2_last_non_winning = 1;

    for _ in 0..21 {
        let (p1_new, p1_winning, p1_current_not_winning) = simulate_player(&p1_dp);
        let (p2_new, p2_winning, p2_current_not_winning) = simulate_player(&p2_dp);

        p1_last_non_winning = p1_current_not_winning;

        p1_total += p1_winning * p2_last_non_winning;
        p2_total += p2_winning * p1_last_non_winning;

        p2_last_non_winning = p2_current_not_winning;

        p1_dp = p1_new;
        p2_dp = p2_new;
    }

    p1_total.max(p2_total)
}

fn main() {
    let (loser, steps) = simulate(1, 3);
    println!("Part 1 {}", loser * steps);

    let universes = simulate_dp(4, 8);
    println!("Part 2 {}", universes);
}
