pub fn solve1(input: &String) {
    let contents: Vec<&str> = input.split('\n').collect();

    let time_contents: Vec<&str> = contents[0].split(": ").into_iter().collect();
    let times: Vec<u32> = time_contents[1]
        .split(' ')
        .filter(|t| *t != "")
        .map(|t| t.trim().parse::<u32>().unwrap())
        .collect();

    let record_contents: Vec<&str> = contents[1].split(": ").into_iter().collect();
    let records: Vec<u32> = record_contents[1]
        .split(' ')
        .filter(|t| *t != "")
        .map(|t| t.trim().parse::<u32>().unwrap())
        .collect();

    println!("{:?}", times);
    println!("{:?}", records);

    let mut solutions: Vec<u32> = Vec::new();

    for (i, t) in times.into_iter().enumerate() {
        let mut count = 0;
        let record = records[i];
        for j in 0..t {
            let new_record = j * (t - j);
            if new_record > record {
                count += 1;
            }
        }
        solutions.push(count);
    }

    println!(
        "Solution P1: {:?}",
        solutions.iter().copied().reduce(|a, b| a * b).unwrap()
    );
}

pub fn solve2(input: &String) {
    let contents: Vec<&str> = input.split('\n').collect();

    let time_contents: Vec<&str> = contents[0].split(": ").into_iter().collect();
    let time: u64 = time_contents[1]
        .split(' ')
        .filter(|t| *t != "")
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let record_contents: Vec<&str> = contents[1].split(": ").into_iter().collect();
    let record: u64 = record_contents[1]
        .split(' ')
        .filter(|t| *t != "")
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    println!("{:?}", time);
    println!("{:?}", record);

    let mut count = 0;

    for j in 0..time {
        let new_record = j * (time - j);
        if new_record > record {
            count += 1;
        }
    }

    println!("Solution P2: {count}");
}
