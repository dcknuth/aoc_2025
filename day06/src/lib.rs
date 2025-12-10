pub fn part1(s: &String) -> i64 {
    let mut problems: Vec<Vec<i64>> = Vec::new();
    let lines: Vec<&str> = s.split('\n').collect();
    for l in &lines[..lines.len()-1] {
        let row: Vec<i64> = l.split_whitespace()
            .map(|n| n.parse().unwrap()).collect();
        problems.push(row);
    }
    let ops: Vec<&str> = lines[lines.len()-1].split_ascii_whitespace()
        .collect();

    let mut total = 0;
    for i in 0..problems[0].len() {
        if ops[i] == "+" {
            let mut sub_total = 0;
            for j in 0..problems.len() {
                sub_total += problems[j][i];
            }
            total += sub_total;
        } else {
            let mut sub_total = 1;
            for j in 0..problems.len() {
                sub_total *= problems[j][i];
            }
            total += sub_total;
        }
    }

    total
}

pub fn part2(s: &String) -> i64 {
    let mut problems: Vec<Vec<i64>> = Vec::new();
    let lines: Vec<&str> = s.split('\n').collect();
    // this time we make a character matrix to read numbers by column
    let char_matrix: Vec<Vec<char>> = s.lines()
        .map(|line| line.chars().collect()).collect();
    
    let mut cur_prob: Vec<i64> = Vec::new();
    for i in 0..char_matrix[0].len() {
        let mut col: Vec<char> = Vec::new();
        for j in 0..char_matrix.len()-1 {
            col.push(char_matrix[j][i]);
        }
        let cur_str: String = col.iter().collect();
        if let Ok(x) = cur_str.trim().parse() {
            cur_prob.push(x);
        } else {
            problems.push(cur_prob.clone());
            cur_prob.clear();
        }
    }
    // catch the last problem
    problems.push(cur_prob.clone());
    
    let ops: Vec<&str> = lines[lines.len()-1].split_ascii_whitespace()
        .collect();

    // now similar to part one
    let mut total = 0;
    for i in 0..problems.len() {
        if ops[i] == "+" {
            let mut sub_total = 0;
            for n in &problems[i] {
                sub_total += n;
            }
            total += sub_total;
        } else {
            let mut sub_total = 1;
            for n in &problems[i] {
                sub_total *= n;
            }
            total += sub_total;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ".to_string();
        let ans_p1 = part1(&s);
        assert_eq!(ans_p1, 4277556);
    }

    #[test]
    fn test_part2() {
        let s = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ".to_string();
        let ans_p2 = part2(&s);
        assert_eq!(ans_p2, 3263827);
    }

}