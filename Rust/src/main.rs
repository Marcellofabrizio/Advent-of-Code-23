use std::fs;
pub mod day1;

fn main() {
    println!("Hello, world!");
    let input = fs::read_to_string("day1/data.txt").expect("File not found");
    day1::solve1(&input);
    day1::solve2(&input);
}
