use std::collections::{HashSet, HashMap};

pub fn part1(s: &String) -> i64 {
    let m: Vec<Vec<char>> = s.split('\n').map(|l| l.chars().collect()).
        collect();
    // instead of changing values in the map I'm just going to track positions
    let mut t_pos = HashSet::new();
    for (i, c) in m[0].iter().enumerate() {
        if *c == 'S' {
            t_pos.insert(i);
        }
    }
    let mut total = 0;
    let mut row = 1;
    while row < m.len() {
        let mut new_t_pos = HashSet::new();
        for i in t_pos {
            if m[row][i] == '.' {
                new_t_pos.insert(i);
            } else if m[row][i] == '^' {
                new_t_pos.insert(i-1);
                new_t_pos.insert(i+1);
                total += 1;
            }
        }
        row += 1;
        t_pos = new_t_pos;
    }

    total
}


pub fn part2(s: &String) -> i64 {
    let m: Vec<Vec<char>> = s.split('\n').map(|l| l.chars().collect()).
        collect();
    // instead of changing values in the map I'm just going to track positions
    let mut t_pos: HashMap<usize, i64> = HashMap::new();
    for (i, c) in m[0].iter().enumerate() {
        if *c == 'S' {
            *t_pos.entry(i).or_insert(0) += 1;
        }
    }
    let mut row = 1;
    while row < m.len() {
        let mut new_t_pos: HashMap<usize, i64> = HashMap::new();
        for i in t_pos.keys() {
            if m[row][*i] == '.' {
                *new_t_pos.entry(*i).or_insert(0) += t_pos[i];
            } else if m[row][*i] == '^' {
                *new_t_pos.entry(*i-1).or_insert(0) += t_pos[i];
                *new_t_pos.entry(*i+1).or_insert(0) += t_pos[i];
            }
        }
        row += 1;
        t_pos = new_t_pos;
    }

    t_pos.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............".to_string();
        let ans_p1 = part1(&s);
        assert_eq!(ans_p1, 21);
    }

    #[test]
    fn test_part2() {
        let s = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............".to_string();
        let ans_p1 = part2(&s);
        assert_eq!(ans_p1, 40);
    }
}