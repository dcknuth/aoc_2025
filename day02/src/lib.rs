use fancy_regex::Regex;

fn halves_match(s: &str) -> bool {
    let pattern = format!(r"^(.+)\1$");
    let re = Regex::new(&pattern).unwrap();
    let matches = re.is_match(s)
        .expect("There was an error in the regex");

    matches
}

fn match_repeats(s: &str) -> bool {
    let pattern = format!(r"^(.+)\1+$");
    let re = Regex::new(&pattern).unwrap();
    let matches = re.is_match(s)
        .expect("There was an error in the regex");

    matches
}

pub fn part1(s: &str) -> u64 {
    let mut total = 0;
    let ranges: Vec<&str> = s.split(',').collect();
    for range in ranges {
        let start_stop: Vec<u64> = range.split('-')
            .map(|x| x.parse().unwrap()).collect();
        let start = start_stop[0];
        let end = start_stop[1] + 1;
        for i in start..end {
            let cur_s: &str = &(i.to_string());
            if halves_match(cur_s) {
                total += i;
            }
        }
    }

    total
}

pub fn part2(s: &str) -> u64 {
    let mut total = 0;
    let ranges: Vec<&str> = s.split(',').collect();
    for range in ranges {
        let start_stop: Vec<u64> = range.split('-')
            .map(|x| x.parse().unwrap()).collect();
        let start = start_stop[0];
        let end = start_stop[1] + 1;
        for i in start..end {
            let cur_s: &str = &(i.to_string());
            if match_repeats(cur_s) {
                total += i;
            }
        }
    }

    total
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_halves_match() {
        let tests = ["abab", "aaaa", "abcd", "abcabc"];
        let results = [true, true, false, true];
        for t in 0..tests.len() {
            assert_eq!(halves_match(tests[t]), results[t]);
        }
    }

    #[test]
    fn test_match_repeats() {
        let tests = ["123123123", "12341234", "123456", "1111111"];
        let results = [true, true, false, true];
        for t in 0..tests.len() {
            assert_eq!(match_repeats(tests[t]), results[t]);
        }
    }

    #[test]
    fn test_part1() {
        let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
            824824821-824824827,2121212118-2121212124";
        let result: u64 = 1227775554;
        let ans_p1 = part1(s);
        assert_eq!(ans_p1, result);
    }

    #[test]
    fn test_part2() {
        let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
            824824821-824824827,2121212118-2121212124";
        let result: u64 = 4174379265;
        let ans_p1 = part2(s);
        assert_eq!(ans_p1, result);
    }
}