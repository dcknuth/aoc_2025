use std::fs;
use std::time::Instant;
use day11::{part1, part2};

fn main() {
    let filename = "input11.txt";
    let s = fs::read_to_string(filename)
        .expect("Could not read the input file");

    let t0 = Instant::now();
    let ans_p1 = part1(&s);
    let duration = t0.elapsed();
    println!("Part 1 is {ans_p1} in {duration:?}");

    let t0 = Instant::now();
    let ans_p2 = part2(&s);
    let duration = t0.elapsed();
    println!("Part 2 is {ans_p2} in {duration:?}");
}
