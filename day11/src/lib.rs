use std::collections::HashMap;

pub fn part1(s: &String) -> u64 {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for l in s.lines() {
        if let Some((key, rest)) = l.split_once(':') {
            let key = key.trim().to_string();
            let values: Vec<String> = rest.split_whitespace()
                .map(|w| w.to_string()).collect();
            connections.insert(key, values);
        }
    }
    
    let mut total_paths = 0;
    let mut cur_devices: HashMap<String, u64> = HashMap::new();
    cur_devices.insert("you".to_string(), 1);
    while cur_devices.len() > 0 {
        let mut next_devices: HashMap<String, u64> = HashMap::new();
        for device in cur_devices.keys() {
            let val = cur_devices.get(device).unwrap();
            let cur_cons = connections.get(device).unwrap();
            for connection in cur_cons {
                if connection == "out" {
                    total_paths += val;
                } else {
                    let n = next_devices.entry(connection.clone()).or_insert(0);
                    *n += val;
                }
            }
        }
        cur_devices = next_devices;
    }

    total_paths
}

pub fn part2(s: &String) -> u64 {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for l in s.lines() {
        if let Some((key, rest)) = l.split_once(':') {
            let key = key.trim().to_string();
            let values: Vec<String> = rest.split_whitespace()
                .map(|w| w.to_string()).collect();
            connections.insert(key, values);
        }
    }
    
    let mut total_paths = 0;
    let mut cur_devices: HashMap<String, u64> = HashMap::new();
    cur_devices.insert("svr 0 0".to_string(), 1);
    while cur_devices.len() > 0 {
        let mut next_devices: HashMap<String, u64> = HashMap::new();
        for device in cur_devices.keys() {
            let val = cur_devices.get(device).unwrap();
            let mut cur_iter = device.split_whitespace();
            let d = cur_iter.next().unwrap();
            let seen_dac = cur_iter.next().unwrap();
            let seen_fft = cur_iter.next().unwrap();
            let cur_cons = connections.get(d).unwrap();
            for connection in cur_cons {
                let mut new_dac = seen_dac;
                let mut new_fft = seen_fft;
                if connection == "out" {
                    if seen_dac == "1" && seen_fft == "1" {
                        total_paths += val;
                    }
                } else {
                    if connection == "dac" {
                        new_dac = "1";
                    }
                    if connection == "fft" {
                        new_fft = "1";
                    }
                    let new_d = format!("{connection} {new_dac} {new_fft}");
                    let n = next_devices.entry(new_d).or_insert(0);
                    *n += val;
                }
            }
        }
        cur_devices = next_devices;
    }

    total_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out".to_string();

        let ans_p1 = part1(&s);
        assert_eq!(ans_p1, 5);
    }

    #[test]
    fn test_part2() {
        let s = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out".to_string();

        let ans_p2 = part2(&s);
        assert_eq!(ans_p2, 2);
    }
}