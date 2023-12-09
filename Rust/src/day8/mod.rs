use regex::Regex;
use std::collections::HashMap;

pub fn solve1(input: &String) {
    let contents: Vec<&str> = input.split('\n').filter(|l| l != &"").collect();
    let move_command: Vec<char> = contents[0].chars().collect();
    let moves: Vec<&str> = contents[1..].to_vec();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut origins: Vec<String> = Vec::new();

    let re = Regex::new(r"\w+").unwrap();

    for m in moves {
        let move_contents: Vec<&str> = m.split(" = ").collect();
        let origin = move_contents[0];
        let destinations: Vec<_> = re.find_iter(move_contents[1]).collect();

        origins.push(origin.to_string());

        map.insert(
            origin.to_string(),
            (
                destinations[0].as_str().to_owned(),
                destinations[1].as_str().to_owned(),
            ),
        );
    }

    let mut curr_pos = "AAA";
    let mut curr_com: usize = 0;
    let mut steps = 0;

    while curr_pos != "ZZZ" {
        steps += 1;

        let (l, r) = map.get(curr_pos).unwrap();
        if move_command[curr_com] == 'L' {
            curr_pos = l;
        } else if move_command[curr_com] == 'R' {
            curr_pos = r;
        }

        if curr_com == move_command.len() - 1 {
            curr_com = 0;
        } else {
            curr_com += 1;
        }
    }

    println!("Solution P1: {steps}");
}

pub fn solve2(input: &String) {
    let contents: Vec<&str> = input.split('\n').filter(|l| l != &"").collect();
    let move_command: Vec<char> = contents[0].chars().collect();
    let moves: Vec<&str> = contents[1..].to_vec();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut origins: Vec<String> = Vec::new();
    let mut starting_points: Vec<String> = Vec::new();

    let re = Regex::new(r"\w+").unwrap();

    for m in moves {
        let move_contents: Vec<&str> = m.split(" = ").collect();
        let origin = move_contents[0];

        if origin.ends_with('A') {
            starting_points.push(origin.to_owned());
        }

        let destinations: Vec<_> = re.find_iter(move_contents[1]).collect();

        origins.push(origin.to_string());

        map.insert(
            origin.to_string(),
            (
                destinations[0].as_str().to_owned(),
                destinations[1].as_str().to_owned(),
            ),
        );
    }

    let mut all_steps: Vec<u64> = Vec::new();

    for s in &starting_points {
        let mut curr_pos = s;
        let mut curr_com: usize = 0;
        let mut steps: u64 = 0;

        while !curr_pos.ends_with('Z') {
            steps += 1;

            let (l, r) = map.get(curr_pos).unwrap();
            if move_command[curr_com] == 'L' {
                curr_pos = l;
            } else if move_command[curr_com] == 'R' {
                curr_pos = r;
            }

            if curr_com == move_command.len() - 1 {
                curr_com = 0;
            } else {
                curr_com += 1;
            }
        }

        all_steps.push(steps);
    }

    println!("{:?}", lcm_of_vector(&all_steps));
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn lcm_of_vector(numbers: &[u64]) -> u64 {
    if numbers.is_empty() {
        return 0;
    }

    let mut result = numbers[0];

    for &number in &numbers[1..] {
        result = lcm(result, number);
    }

    result
}
