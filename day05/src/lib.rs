fn merge_range(r1: (i64, i64), r2: (i64, i64)) -> Option<(i64, i64)> {
    if r1.1 < r2.0 || r1.0 > r2.1 {
        None
    } else {
        let start: i64;
        let end: i64;
        if r1.0 < r2.0 {
            start = r1.0;
        } else {
            start = r2.0;
        }
        if r1.1 > r2.1 {
            end = r1.1;
        } else {
            end = r2.1;
        }
        Some((start, end))
    }
}

fn merge_all(ranges: &mut Vec<(i64, i64)>) {
    let mut done = false;
    while !done {
        let mut found = false;
        for i in 0..(ranges.len()-1) {
            if found {
                break;
            }
            for j in i+1..ranges.len() {
                if let Some(new_range) = merge_range(ranges[i], ranges[j]) {
                    found = true;
                    ranges[i] = new_range;
                    // Maybe try swap_remove later or a HashMap
                    ranges.remove(j);
                    break;
                } else {
                    continue;
                }
            }
        }
        if !found {
            done = true;
        }
    }
}

pub fn part1(s: &String, ranges: &mut Vec<(i64, i64)>) -> u64 {
    let mut input_break = 0;
    let lines: Vec<&str> = s.split('\n').collect();
    for l in &lines {
        if *l == "" {
            input_break += 1;
            break;
        }
        let (s1, s2) = l.split_once('-').unwrap();
        let start = s1.parse().unwrap();
        let end = s2.parse().unwrap();
        let range = (start, end);
        ranges.push(range);
        input_break += 1;
    }

    merge_all(ranges);

    let mut count_fresh = 0;
    for l in &lines[input_break..] {
        let id: i64 = l.parse().unwrap();
        for (lower, upper) in &mut *ranges {
            if id >= *lower && id <= *upper {
                count_fresh += 1;
                break
            }
        }
    }

    count_fresh
}

pub fn part2(ranges: &mut Vec<(i64, i64)>) -> u64 {
    let mut total: u64 = 0;

    for (lower, upper) in &mut *ranges {
        total += (*upper - *lower) as u64 + 1;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_range() {
        let r1 = (10, 14);
        let r2 = (16, 20);
        let r3 = (12, 18);
        
        assert_eq!(merge_range(r1, r2), None);
        assert_eq!(merge_range(r1, r3), Some((10, 18)));
        assert_eq!(merge_range(r2, r3), Some((12, 20)));
    }

    #[test]
    fn test_merge_all() {
        let mut ranges = vec![(10, 14), (16, 20), (12, 18)];
        let final_range = vec![(10, 20)];

        merge_all(&mut ranges);
        assert_eq!(ranges, final_range);
    }

    #[test]
    fn test_parts() {
        let s = "3-5
10-14
16-20
12-18

1
5
8
11
17
32".to_string();
        let mut ranges: Vec<(i64, i64)> = Vec::new();

        let ans_p1 = part1(&s, &mut ranges);
        assert_eq!(ans_p1, 3);
        let ans_p2 = part2(&mut ranges);
        assert_eq!(ans_p2, 14);
    }
}