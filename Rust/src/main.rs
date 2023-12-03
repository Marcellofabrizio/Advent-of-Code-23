use std::fs;
mod day1;
mod day3;

fn main() {
    println!("Solving AOC 2023\n");
    let input = fs::read_to_string("day3/data2.txt").expect("File not found");
    day3::solve1(&input);
}
