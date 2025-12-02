use std::fs;
use std::time::Instant;
use day01::{open1, open2};

fn main() {
    let filename = "input01.txt";
    let s = fs::read_to_string(filename)
    .expect("Failed to read in input file {filename}");
    let t0 = Instant::now();
    let ans_p1 = open1(&s);
    let duration = t0.elapsed();
    println!("{ans_p1} in {:.4?}", duration);

    let t0 = Instant::now();
    let ans_p2 = open2(&s);
    let duration = t0.elapsed();
    println!("{ans_p2} in {:.4?}", duration);
}
