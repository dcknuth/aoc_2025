pub fn part1(s: &String) -> i64 {
    let mut presents: Vec<(u32, u32)> = Vec::new();
    let s_lines: Vec<_> = s.lines().collect();
    for i in 0..6 {
        let j = i * 5 + 1;
        let mut blocks = 0;
        let mut spaces = 0;
        for k in 0..3 {
            for c in s_lines[j+k].chars() {
                if c == '#' {
                    blocks += 1;
                } else {
                    spaces += 1;
                }
            }
        }
        presents.push((blocks, spaces));
    }
    
    let mut total = 0;
    for line in &s_lines[30..] {
        let parts: Vec<_> = line.split(':')
            .map(|s| s.trim()).collect();
        let tree: Vec<u32> = parts[0].split('x')
            .map(|n| n.parse().unwrap()).collect();
        let p_nums: Vec<u32> = parts[1].split_whitespace()
            .map(|n| n.parse().unwrap()).collect();
        let tree_area = tree[0] * tree[1];
        let mut present_area = 0;
        for (i, p) in p_nums.iter().enumerate() {
            present_area += presents[i].0 * p;
        }
        if (present_area as f32 / tree_area as f32) < 0.85 {
            total += 1;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2".to_string();

        let ans_p1 = part1(&s);
        assert_eq!(ans_p1, 2);
    }
}