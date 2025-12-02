pub fn open1(s: &String) -> u32 {
    let mut times_at_0: u32 = 0;
    let mut dial_pointer: i32 = 50;
    for cur_rotation in s.trim_end().split("\n") {
        let Some(direction) = cur_rotation.chars().nth(0) else {
                panic!("No first char of L or R?")
        };
        let num_str: String = cur_rotation.chars().skip(1).collect();
        let n: i32 = num_str.parse().unwrap();
        if direction == 'R' {
            dial_pointer += n;
        } else {
            dial_pointer -= n;
        }
        dial_pointer %= 100;
        if dial_pointer == 0 {
            times_at_0 += 1;
        }
    }
    times_at_0
}

pub fn open2(s: &String) -> u32 {
    let mut times_by_0: u32 = 0;
    let mut dial_pointer: i32 = 50;
    for cur_rotation in s.trim_end().split("\n") {
        let Some(direction) = cur_rotation.chars().nth(0) else {
                panic!("No first char of L or R?")
        };
        let num_str: String = cur_rotation.chars().skip(1).collect();
        let n: i32 = num_str.parse().unwrap();
        if direction == 'R' {
            times_by_0 += ((dial_pointer + n) / 100) as u32;
            dial_pointer = (dial_pointer + n) % 100;
        } else {
            if dial_pointer > 0 {
                dial_pointer -= n;
                if dial_pointer < 1 {
                    times_by_0 += ((dial_pointer / 100).abs() as u32) + 1;
                    dial_pointer = ((dial_pointer % 100) + 100) % 100;
                }
            } else {
                dial_pointer -= n;
                if dial_pointer < 1 {
                    times_by_0 += (dial_pointer / 100).abs() as u32;
                    dial_pointer = ((dial_pointer % 100) + 100) % 100;
                }
            }
        }
    }
    times_by_0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open1() {
        let s = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82".to_string();
        assert_eq!(open1(&s), 3);
    }

    #[test]
    fn test_open2() {
        let s = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82".to_string();
        assert_eq!(open2(&s), 6);
    }
}