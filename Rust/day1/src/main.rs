use std::fs;

fn main() {
    let input = fs::read_to_string("../data.txt").expect("File not found");
    // let replacedInput: Vec<u32> =
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

    // let sol: u64 = filtered_input.iter().sum();
    println!("{:?}", filtered_input.into_iter().sum::<u32>());
}

fn day1_1() {
    let contents = fs::read_to_string("../data.txt").expect("File not found");
    let sum: u32 = contents
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
                    println!("{start}, {end}");
                }
            }

            return format!("{}{}", start, end).parse::<u32>().unwrap_or(0);
        })
        .into_iter()
        .sum();

    println!("{:?}", sum);
}

fn day1_2() {
    let input = fs::read_to_string("../data2.txt").expect("File not found");

    // let replacedInput: Vec<u32> =
    let replaced_input = input.split("\n").into_iter().map(|i| {
        println!("Oi");

        i.replace("zero", "zeroOzero")
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("thee", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
    });
}
