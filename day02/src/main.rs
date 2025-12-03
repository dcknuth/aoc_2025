use std::fs;
use std::time::Instant;
use day02::{part1, part2};

fn main() {
    let filename = "input02.txt";
    let s = fs::read_to_string(filename)
        .expect("could not find file");
    let s = s.trim_end();
    let t0 = Instant::now();
    let ans_p1 = part1(s);
    let duration = t0.elapsed();
    println!("{ans_p1} in {:.4?}", duration);

    let t0 = Instant::now();
    let ans_p2 = part2(s);
    let duration = t0.elapsed();
    println!("{ans_p2} in {:.4?}", duration);
}
