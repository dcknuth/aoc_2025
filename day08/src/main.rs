use std::fs;
use std::time::Instant;
use day08::{part1_and2};

fn main() {
    let filename = "input08.txt";
    let s = fs::read_to_string(filename)
        .expect("Could not open input file");

    let t0 = Instant::now();
    let (ans_p1, ans_p2) = part1_and2(&s, 1000);
    let duration = t0.elapsed();
    println!("Part 1 is {ans_p1}, part 2 is {ans_p2} in {:.4?}", duration);
}
