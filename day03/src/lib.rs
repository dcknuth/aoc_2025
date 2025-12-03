fn get_jolts(batteries: &[u64], n: usize) -> u64 {
    let mut start = batteries.len();
    start -= n;
    let end = batteries.len();
    let mut indexes: Vec<usize> = (start..end).collect();
    let mut jolts = 0;

    // now shift each index as far left as possible
    let mut min_index = 0;
    for i in 0..indexes.len() {
        for j in (min_index..indexes[i]).rev() {
            if batteries[j] >= batteries[indexes[i]] {
                indexes[i] = j;
            }
        }
        min_index = indexes[i] + 1;
    }

    let mut cur_n = n as i64 - 1;
    for i in indexes {
        jolts += batteries[i] * 10u64.pow(cur_n as u32);
        cur_n -= 1;
    }

    jolts
}

pub fn part1(s: &String) -> u64 {
    let mut total = 0;
    for b in s.split('\n') {
        let batteries: Vec<u64> = b.chars().map(|c| c.to_digit(10)
            .unwrap() as u64).collect();
        total += get_jolts(&batteries, 2);
    }
    
    total
}

pub fn part2(s: &String) -> u64 {
    let mut total = 0;
    for b in s.split('\n') {
        let batteries: Vec<u64> = b.chars().map(|c| c.to_digit(10)
            .unwrap() as u64).collect();
        total += get_jolts(&batteries, 12);
    }
    
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_jolts() {
        let s = "987654321111111";
        let result = 98;
        let batteries: Vec<u64> = s.chars().map(|c| c.to_digit(10)
            .unwrap() as u64).collect();
        let jolts = get_jolts(&batteries, 2);
        assert_eq!(jolts, result);
    }

    #[test]
    fn test_part1() {
        let s = "987654321111111
811111111111119
234234234234278
818181911112111".to_string();
        let result = 357;
        
        assert_eq!(part1(&s), result);
    }

    #[test]
    fn test_part2() {
        let s = "987654321111111
811111111111119
234234234234278
818181911112111".to_string();
        let result = 3121910778619;
        
        assert_eq!(part2(&s), result);
    }
}