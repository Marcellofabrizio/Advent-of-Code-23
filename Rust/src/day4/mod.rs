use std::collections::HashSet;

pub fn solve1(input: &String) {
    let lines = input.lines();
    let mut ans = 0;

    for l in lines {
        let sections: Vec<&str> = l.split(':').collect();
        let line_nums: Vec<&str> = sections[1].split('|').collect();

        let nums: HashSet<u32> = line_nums[0]
            .split(' ')
            .filter(|n| *n != "")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let wins: Vec<u32> = line_nums[1]
            .split(' ')
            .filter(|n| *n != "")
            .map(|n| n.parse::<u32>().unwrap())
            .filter(|n| nums.contains(n))
            .collect();

        let base: u32 = 2;
        if wins.len() > 0 {
            let l_ans = base.pow(wins.len() as u32 - 1);
            ans += l_ans;
        }
    }

    println!("Answer: {ans}");
}

pub fn solve2(input: &String) {
    let lines = input.lines();
    let mut winning_tickets = vec![0; lines.clone().count()];

    for (i, l) in lines.enumerate() {
        let sections: Vec<&str> = l.split(':').collect();
        let line_nums: Vec<&str> = sections[1].split('|').collect();

        let nums: HashSet<u32> = line_nums[0]
            .split(' ')
            .filter(|n| *n != "")
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let wins: Vec<u32> = line_nums[1]
            .split(' ')
            .filter(|n| *n != "")
            .map(|n| n.parse::<u32>().unwrap())
            .filter(|n| nums.contains(n))
            .collect();

        winning_tickets[i] += 1;

        for j in i + 1..i + wins.len() + 1 {
            winning_tickets[j] += winning_tickets[i];
        }
    }

    println!("Answer: {:?}", winning_tickets.iter().sum::<i32>());
}
