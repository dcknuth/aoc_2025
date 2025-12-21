pub fn part1(s: &String) -> i64 {
    let points: Vec<Vec<i32>> = s.lines()
        .map(|l| l.split(',')
            .map(|n| n.parse::<i32>().unwrap()).collect())
            .collect();
    
    let mut max_area: i64 = 0;
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            let cur_area: i64 = ((points[i][0] - points[j][0]).abs() + 1) as i64 *
                ((points[i][1] - points[j][1]).abs() + 1) as i64;
            if cur_area > max_area {
                max_area = cur_area;
            }
        }
    }

    max_area
}

use itertools::Itertools;

pub fn part2(s: &String) -> i64 {
    let points: Vec<Vec<i32>> = s.lines()
        .map(|l| l.split(',')
            .map(|n| n.parse::<i32>().unwrap()).collect())
            .collect();
    let mut lines: Vec<Vec<&Vec<i32>>> = Vec::new();
    for (a, b) in points.iter().cycle().tuple_windows()
        .take(points.len()) {
            lines.push(vec![a, b]);
        }
    
    let mut max_area: i64 = 0;
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            let cur_area: i64 = ((points[i][0] - points[j][0]).abs() + 1) as i64 *
                    ((points[i][1] - points[j][1]).abs() + 1) as i64;
            if cur_area > max_area {
                // for each rectangle, see if a line passes inside
                let mut intersect = false;
                for line in &lines {
                    if i32::max(line[0][0], line[1][0]) <=
                        i32::min(points[i][0], points[j][0]) {
                            continue;
                    }
                    if i32::min(line[0][0], line[1][0]) >=
                        i32::max(points[i][0], points[j][0]) {
                            continue;
                    }
                    if i32::max(line[0][1], line[1][1]) <=
                        i32::min(points[i][1], points[j][1]) {
                            continue;
                    }
                    if i32::min(line[0][1], line[1][1]) >=
                        i32::max(points[i][1], points[j][1]) {
                            continue;
                    }
                    intersect = true;
                    break;
                }
                if !intersect {
                        max_area = cur_area;
                    }
            }
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3".to_string();
        let ans_p1 = part1(&s);
        assert_eq!(ans_p1, 50);
    }

    #[test]
    fn test_part2() {
        let s = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3".to_string();
        let ans_p2 = part2(&s);
        assert_eq!(ans_p2, 24);
    }
}