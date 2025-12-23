// use z3::{Config, Context, Solver, ast::Int};
use itertools::Itertools;

pub fn part1(s: &String) -> u64 {
    let mut lights: Vec<Vec<bool>> = Vec::new();
    let mut buttons: Vec<Vec<Vec<usize>>> = Vec::new();
    let mut jolts: Vec<Vec<u64>> = Vec::new();
    for l in s.lines() {
        // lights part
        let parts: Vec<&str> = l.split_whitespace().collect();
        let light: Vec<bool> = parts[0].chars().skip(1)
            .take(parts[0].chars().count() - 2)
            .map(|c| if c=='.' {false} else {true}).collect();
        lights.push(light);
        // buttons part
        let cur_buttons: Vec<Vec<usize>> = parts[1..parts.len()-1].iter()
            .map(|b| {
                b[1..b.len()-1].split(',').map(|n| n.parse::<usize>().unwrap())
                .collect()
            }).collect();
        buttons.push(cur_buttons);
        // jolts part
        let cur_jolts: Vec<u64> = 
            parts[parts.len()-1][1..parts[parts.len()-1].len()-1]
            .split(',').map(|n| n.parse().unwrap()).collect();
        jolts.push(cur_jolts);
    }
    
    let mut min_total = 0;
    for m in 0..buttons.len() {
        let mut n = 1;
        let mut found = false;
        while !found {
            for presses in buttons[m].iter().combinations(n) {
                let mut test_state = vec![false; lights[m].len()];
                for press in presses {
                    for &i in press {
                        test_state[i] = !test_state[i];
                    }
                }
                if test_state == lights[m] {
                    found = true;
                    min_total += n;
                    break;
                }
            }
            n += 1;
        }
    }

    min_total as u64
}


use z3::{Optimize, SatResult, ast::Int, ast::Bool};
fn constraint(vars: &[Int], coeffs: &[i64],
                    rhs: u64,) -> Bool {
    assert_eq!(vars.len(), coeffs.len());
    // Build sum(coeff[i] * vars[i])
    let terms: Vec<Int> = vars.iter()
        .zip(coeffs)
        .map(|(v, c)| Int::mul(&[v, &Int::from_i64(*c)]))
        .collect();
    // Z3's add takes &[&Int], so convert
    let term_refs: Vec<&Int> = terms.iter().collect();
    let sum = Int::add(&term_refs);
    sum.eq(&Int::from_u64(rhs))
}

pub fn part2(s: &String) -> u64 {
    let mut lights: Vec<Vec<bool>> = Vec::new();
    let mut buttons: Vec<Vec<Vec<usize>>> = Vec::new();
    let mut jolts: Vec<Vec<u64>> = Vec::new();
    for l in s.lines() {
        // lights part
        let parts: Vec<&str> = l.split_whitespace().collect();
        let light: Vec<bool> = parts[0].chars().skip(1)
            .take(parts[0].chars().count() - 2)
            .map(|c| if c=='.' {false} else {true}).collect();
        lights.push(light);
        // buttons part
        let cur_buttons: Vec<Vec<usize>> = parts[1..parts.len()-1].iter()
            .map(|b| {
                b[1..b.len()-1].split(',').map(|n| n.parse::<usize>().unwrap())
                .collect()
            }).collect();
        buttons.push(cur_buttons);
        // jolts part
        let cur_jolts: Vec<u64> = 
            parts[parts.len()-1][1..parts[parts.len()-1].len()-1]
            .split(',').map(|n| n.parse().unwrap()).collect();
        jolts.push(cur_jolts);
    }
    
    let mut min_total = 0;
    for m in 0..jolts.len() {
        let opt = Optimize::new();
        let mut b_indexes: Vec<Vec<usize>> =
            vec![vec![0; jolts[m].len()]; buttons[m].len()];
        let mut vars: Vec<Int> = Vec::new();
        for b in 0..buttons[m].len() {
            for &j in &buttons[m][b] {
                b_indexes[b][j] += 1;
            }
            let var = Int::new_const(format!("b{b}"));
            opt.assert(&var.ge(0));
            vars.push(var);
        }
        // println!("b_indexes is:\n{b_indexes:?}");
        let total_presses = Int::add(&vars);
        opt.minimize(&total_presses);
        for i in 0..jolts[m].len() {
            let mut coeffs = vec![0; buttons[m].len()];
            for b in 0..b_indexes.len() {
                if b_indexes[b][i] == 1 {
                    coeffs[b] = 1;
                }
            }
            let cur_eq = constraint(&vars, &coeffs, jolts[m][i]);
            opt.assert(&cur_eq);
        }
        // println!("{opt:?}");
        if let SatResult::Sat = opt.check(&[]) {
            // println!("There is some SatResult");
            let model = opt.get_model().unwrap();
            // println!("Model is:\n{model:?}");
            for b in 0..buttons[m].len() {
                let b_presses = model.eval(&vars[b], true)
                    .unwrap().as_u64().unwrap();
                // println!("{b_presses}");
                min_total += b_presses;
            }
        }
    }

    min_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string();
        let ans_p1 = part1(&s);
        assert_eq!(ans_p1, 7);
    }

    #[test]
    fn test_part2() {
        let s = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}".to_string();
        let ans_p2 = part2(&s);
        assert_eq!(ans_p2, 33);
    }
}