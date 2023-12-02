use std::fs;
pub fn solve1(input: &String) {
    let sum: u32 = input
        .split("\n")
        .into_iter()
        .map(|l| {
            let mut start: String = "".to_string();
            let mut end: String = "".to_string();

            for c in l.chars() {
                if c.is_numeric() {
                    if start == "" {
                        start = c.to_string();
                    }

                    end = c.to_string();
                }
            }

            return format!("{}{}", start, end).parse::<u32>().unwrap_or(0);
        })
        .into_iter()
        .sum();

    println!("{:?}", sum);
}

pub fn solve2(input: &String) {
    let filtered_input: Vec<u32> = input
        .split("\n")
        .clone()
        .map(|i| {
            let replaced: Vec<char> = i
                .replace("zero", "z0o")
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .chars()
                .filter(|chr| chr.is_digit(10))
                .collect();

            let first = &replaced[0];
            let last = &replaced[replaced.len() - 1];

            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .collect();

    println!("{:?}", filtered_input.into_iter().sum::<u32>());
}
