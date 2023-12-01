use std::fs;

fn main() {
    day2_2();
}

fn day2_1() {
    let digits: Vec<(String, _)> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .flat_map(|(i, &digit)| [(digit.into(), i + 1), (format!("{}", i + 1), i + 1)])
    .collect();

    let contents = fs::read_to_string("../data2.txt").expect("File not found");
    let data: Vec<u32> = contents
        .split("\n")
        .into_iter()
        .map(|d| {
            let mut row_calibration: String = "".to_string();
            let mut char_buf: String = "".to_string();
            for (_, c) in d.chars().enumerate() {
                if c.is_numeric() {
                    row_calibration.push(c);
                    break;
                } else {
                    char_buf.push(c);
                    if let Some(n) = map_string_to_digit(&char_buf) {
                        println!("{}", n);
                        row_calibration.push(n);
                    }
                }
            }

            for (_, c) in d.chars().rev().enumerate() {
                if c.is_numeric() {
                    row_calibration.push(c);
                    break;
                }
            }

            return row_calibration.parse::<u32>().unwrap_or(0);
        })
        .collect();

    let sum = data.into_iter().reduce(|a, b| a + b).unwrap();
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

    let sum: usize = fs::read_to_string("data2.txt")
        .expect("File not found")
        .split("\n")
        .map(|l| {
            let test1: Vec<(String, _)> = digits.iter().map(|(s, i)| l.find(s.as_str())).collect();
            let test2: Vec<Option<usize>> =
                digits.iter().map(|(s, i)| l.rfind(s.as_str())).collect();
            println!("{:?}", l);
            println!("{:?}", test1);
            println!("{:?}", test2);

            let (_, first) = digits
                .iter()
                .min_by_key(|(s, _)| l.find(s.as_str()).unwrap_or(usize::MAX))
                .unwrap();
            let (_, second) = digits
                .iter()
                .max_by_key(|(s, _)| l.rfind(s.as_str()).map(|x| x as i32))
                .unwrap();
            return first * 10 + second;
        })
        .sum();

    println!("{:?}", sum);
}
