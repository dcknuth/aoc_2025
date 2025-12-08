use std::fs;
use std::time::Instant;
use day04::{part1, part2};

fn main() {
    let filename = "input04.txt";
    let s = fs::read_to_string(filename)
        .expect("Could not read input file");

    let t0 = Instant::now();
    let ans_p1 = part1(&s);
    let duration = t0.elapsed();
    println!("Part 1 answer is {ans_p1} in {:.4?}", duration);

    let t0 = Instant::now();
    let ans_p2 = part2(&s);
    let duration = t0.elapsed();
    println!("Part 2 answer is {ans_p2} in {:.4?}", duration);
}
