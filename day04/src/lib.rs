fn load_matrix(s: &String) -> Vec<Vec<char>> {
    let m: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect())
        .collect();

    m
}

fn num_accessible(m: Vec<Vec<char>>, n: i8) -> u64 {
    let mut total = 0;
    for y in 0..m.len() {
        for x in 0..m[y].len() {
            if m[y][x] == '@' {
                let mut count: i8 = 0;
                for dy in [-1, 0, 1] {
                    for dx in [-1, 0, 1] {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
                        if ny > -1 && ny < m.len() as isize &&
                            nx > -1 && nx < m[y].len() as isize {
                            if m[ny as usize][nx as usize] == '@' {
                                count += 1;
                            }
                        }
                    }
                }
                if count < n {
                    total += 1;
                }
            }
        }
    }

    total
}

pub fn part1(s: &String) -> u64 {
    let m = load_matrix(s);
    
    num_accessible(m, 4)
}

fn remove_rolls(m: &mut Vec<Vec<char>>, n: i8) -> u64 {
    // make a accessible list, record the number and remove the list
    let mut total = 0;
    let mut to_remove: Vec<(usize, usize)> = Vec::new();
    for y in 0..m.len() {
        for x in 0..m[y].len() {
            if m[y][x] == '@' {
                let mut count: i8 = 0;
                for dy in [-1, 0, 1] {
                    for dx in [-1, 0, 1] {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
                        if ny > -1 && ny < m.len() as isize &&
                            nx > -1 && nx < m[y].len() as isize {
                            if m[ny as usize][nx as usize] == '@' {
                                count += 1;
                            }
                        }
                    }
                }
                if count < n {
                    total += 1;
                    to_remove.push((y, x))
                }
            }
        }
    }
    for roll in to_remove {
        m[roll.0][roll.1] = '.';
    }

    total
}

pub fn part2(s: &String) -> u64 {
    let mut m = load_matrix(s);
    let mut total = 0;
    let mut num_removed = remove_rolls(&mut m, 4);
    while num_removed > 0 {
        total += num_removed;
        num_removed = remove_rolls(&mut m, 4);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_matrix() {
        let text = "abc\ndef\nghijk".to_string();
        let m = load_matrix(&text);
        let test_m: Vec<Vec<char>> = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i', 'j', 'k']];
        assert_eq!(m, test_m);
    }

    #[test]
    fn test_part1() {
        let s = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.".to_string();
        assert_eq!(part1(&s), 13);
    }

    #[test]
    fn test_part2() {
        let s = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.".to_string();
        assert_eq!(part2(&s), 43);
    }
}