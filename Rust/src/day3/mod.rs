use regex::Regex;
use std::{collections::hash_map::Entry, collections::HashMap};

#[derive(Debug)]
struct Number {
    value: i32,
    position: (usize, usize),
    length: usize,
}

#[derive(Debug)]
struct Symbol {
    value: char,
    position: (usize, usize),
}

pub fn solve1(input: &String) {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let size_n = lines.len();
    let size_m = lines[0].len();

    let numbers = get_nums_re(&lines);

    let symbols = get_symbols(&lines);

    let mut sum = 0;
    for n in numbers.iter() {
        let neighbors = get_neighbors(size_n, size_m, n.position, n.length);
        for neighbor in neighbors.iter() {
            match symbols.get(neighbor) {
                Some(s) => sum += n.value,
                None => (),
            }
        }
    }

    println!("{sum}");
}

pub fn solve2(input: &String) {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let size_n = lines.len();
    let size_m = lines[0].len();

    let numbers = get_nums_re(&lines);
    let symbols = get_symbols(&lines);

    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
    for n in numbers.iter() {
        let neighbors = get_neighbors(size_n, size_m, n.position, n.length);
        for neighbor in neighbors.iter() {
            match symbols.get(neighbor) {
                Some(s) => {
                    if s.value == '*' {
                        if let Entry::Vacant(e) = gears.entry(s.position) {
                            gears.insert(s.position, vec![n.value]);
                        } else {
                            gears.get_mut(&s.position).unwrap().push(n.value);
                        }
                    }
                }
                None => (),
            }
        }
    }

    let result: i32 = gears
        .iter()
        .filter(|g| g.1.len() > 1)
        .map(|g| g.1.iter().product::<i32>())
        .sum();

    println!("{result}");
}

fn get_nums_re(input: &Vec<Vec<char>>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    let re = Regex::new(r"\d+").unwrap();
    for (row, line) in input.iter().enumerate() {
        for m in re.find_iter(&line.iter().collect::<String>()) {
            println!("{:?}", m);
            numbers.push(Number {
                value: m.as_str().parse::<i32>().unwrap(),
                position: (row, m.start()),
                length: m.as_str().len(),
            });
        }
    }

    return numbers;
}

fn get_symbols(input: &Vec<Vec<char>>) -> HashMap<(usize, usize), Symbol> {
    let mut symbols = HashMap::new();

    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if !c.is_ascii_digit() && *c != '.' {
                let pos: (usize, usize) = (row, col);
                symbols.insert(
                    pos,
                    Symbol {
                        value: *c,
                        position: pos,
                    },
                );
            }
        }
    }

    return symbols;
}

fn get_neighbors(
    grid_rows_len: usize,
    grid_cols_len: usize,
    pos: (usize, usize),
    length: usize,
) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(i32, i32)> = Vec::new();

    for row in pos.0 as i32 - 1..pos.0 as i32 + 2 {
        for col in pos.1 as i32 - 1..pos.1 as i32 + length as i32 + 1 {
            neighbors.push((row, col));
        }
    }

    return neighbors
        .iter()
        .filter(|n| {
            !(n.0 == pos.0 as i32 && n.1 >= pos.1 as i32 && n.1 < pos.1 as i32 + length as i32)
        })
        .filter(|n| n.0 >= 0 && n.0 < grid_rows_len as i32)
        .filter(|n| n.1 >= 0 && n.1 < grid_cols_len as i32)
        .map(|n| (n.0 as usize, n.1 as usize))
        .collect();
}
