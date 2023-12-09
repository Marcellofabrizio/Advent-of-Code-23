pub fn solve1(input: &String) {
    let contents = input
        .split('\n')
        .map(|l| {
            l.split(" ")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let last_nums = contents
        .iter()
        .map(|c| extrapolate(c.to_vec()))
        .collect::<Vec<i64>>();

    println!("{:?}", last_nums.iter().sum::<i64>());
}

pub fn extrapolate(vals: Vec<(i64)>) -> i64 {
    if vals.iter().all(|v| v == &0) {
        return 0;
    } else {
        let last = vals.last().unwrap();
        let next_vals = vals.windows(2).map(|x| x[1] - x[0]).collect::<Vec<i64>>();
        return *last + extrapolate(next_vals);
    }
}

pub fn solve2(input: &String) {
    let contents = input
        .split('\n')
        .map(|l| {
            l.split(" ")
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let last_nums = contents
        .iter()
        .map(|c| extrapolate2(c.to_vec()))
        .collect::<Vec<(i64, i64)>>();

    let mut result: (i64, i64) = (0, 0);
    for n in last_nums {
        result.0 += n.0;
        result.1 += n.1;
    }

    println!("First Extrapolations: {:?}", result.0);
    println!("Last Extrapolations: {:?}", result.1);
}

pub fn extrapolate2(vals: Vec<i64>) -> (i64, i64) {
    if vals.iter().all(|v| v == &0) {
        return (0, 0);
    } else {
        let next_vals = vals.windows(2).map(|x| x[1] - x[0]).collect();
        let (first, last) = extrapolate2(next_vals);

        return (vals[0] - first, vals.last().unwrap() + last);
    }
}
