use std::{collections::HashMap, f32::consts::E};

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

    let numbers = get_nums(&lines);
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

fn get_nums(input: &Vec<Vec<char>>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    for (row, line) in input.iter().enumerate() {
        let mut num_buffer: String = String::new();
        let mut checked_cols: usize = 0;
        for (col, c) in line.iter().enumerate() {
            let mut num_length: usize = 1;
            if c.is_ascii_digit() && col >= checked_cols {
                let pos: (usize, usize) = (row, col);
                num_buffer.push(*c);
                while col + num_length < line.len() && line[col + num_length].is_ascii_digit() {
                    num_buffer.push(line[col + num_length].to_owned());
                    num_length += 1;
                }

                checked_cols = col + num_length;

                numbers.push(Number {
                    value: num_buffer.parse::<i32>().unwrap(),
                    position: pos,
                    length: num_length,
                });

                num_buffer.clear();
            }
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
