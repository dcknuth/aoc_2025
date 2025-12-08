use std::fs;
use std::time::Instant;
use day05::{part1, part2};

fn main() {
    let filename = "input05.txt";
    let s = fs::read_to_string(filename)
        .expect("Could not open the input file");
    let s = s.trim_end().to_string();

    let t0 = Instant::now();
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let ans_p1 = part1(&s, &mut ranges);
    let duration = t0.elapsed();
    println!("Part one is {ans_p1} and took {:.4?}", duration);

    let t0 = Instant::now();
    let ans_p2 = part2(&mut ranges);
    let duration = t0.elapsed();
    println!("Part one is {ans_p2} and took {:.4?}", duration);
}
