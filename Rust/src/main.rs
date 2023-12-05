use std::fs;
mod day1;
mod day3;
mod day4;

fn main() {
    println!("Solving AOC 2023\n");
    let input = fs::read_to_string("day4/data2.txt").expect("File not found");
    day4::solve2(&input);
}
