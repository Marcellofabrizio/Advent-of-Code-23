use std::fs;

fn main() {
    day2_1();
}

fn day2_1() {
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

fn map_string_to_digit(s: &String) -> Option<char> {
    match s.as_str() {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn day2_2() {
    let digits: Vec<(String, _)> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .flat_map(|(i, &d)| [(d.into(), i + 1), (format!("{}", i + 1), i + 1)])
    .collect();
}
