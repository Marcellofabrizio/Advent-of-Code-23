use std::cmp::{max, min};

pub fn solve1(input: &String) {
    let contents = input.split("\n\n").collect::<Vec<&str>>();
    let map_contents: Vec<&str> = contents[1..].to_vec();
    let mut seeds = contents
        .into_iter()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for map_content in &map_contents {
        let lines: Vec<&str> = map_content
            .split(":")
            .nth(1)
            .unwrap()
            .split("\n")
            .filter(|x| *x != "")
            .collect();

        let ranges: Vec<(std::ops::Range<i64>, std::ops::Range<i64>)> = lines
            .into_iter()
            .map(|l| {
                let content: Vec<&str> = l.split(" ").collect();
                let destination = content[0].parse::<i64>().unwrap();
                let source = content[1].parse::<i64>().unwrap();
                let len = content[2].parse::<i64>().unwrap();
                (
                    std::ops::Range {
                        start: destination,
                        end: destination + len,
                    },
                    std::ops::Range {
                        start: source,
                        end: source + len,
                    },
                )
            })
            .collect();

        seeds = seeds
            .clone()
            .into_iter()
            .map(|s| translate(s, &ranges))
            .collect();
    }

    println!("{:?}", seeds.into_iter().min().unwrap());
}

fn translate(seed: i64, ranges: &Vec<(std::ops::Range<i64>, std::ops::Range<i64>)>) -> i64 {
    for (a, b) in ranges {
        println!("{:?}, {}, {:?}", a, seed, b);
        if b.contains(&seed) {
            return a.start + (seed - b.start);
        }
    }

    return seed;
}

pub fn solve2(input: &String) {
    let contents = input.split("\n\n").collect::<Vec<&str>>();
    let map_contents: Vec<&str> = contents[1..].to_vec();
    let seeds_content = contents
        .into_iter()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut seeds: Vec<std::ops::Range<i64>> = Vec::new();

    for (i, s) in seeds_content.iter().enumerate().step_by(2) {
        seeds.push(std::ops::Range {
            start: *s,
            end: s + seeds_content[i + 1],
        });
    }

    for map_content in &map_contents {
        let lines: Vec<&str> = map_content
            .split(":")
            .nth(1)
            .unwrap()
            .split("\n")
            .filter(|x| *x != "")
            .collect();

        let ranges: Vec<(std::ops::Range<i64>, std::ops::Range<i64>)> = lines
            .into_iter()
            .map(|l| {
                let content: Vec<&str> = l.split(" ").collect();
                let destination = content[0].parse::<i64>().unwrap();
                let source = content[1].parse::<i64>().unwrap();
                let len = content[2].parse::<i64>().unwrap();
                (
                    std::ops::Range {
                        start: destination,
                        end: destination + len,
                    },
                    std::ops::Range {
                        start: source,
                        end: source + len,
                    },
                )
            })
            .collect();

        let mut new_seeds: Vec<std::ops::Range<i64>> = Vec::new();

        println!("{:?}", ranges);

        for s in seeds.iter() {
            for (a, b) in ranges.iter() {
                if s.end <= b.start || b.end <= s.start {
                    new_seeds.push(s.clone());
                    continue;
                }

                let inner_range = std::ops::Range {
                    start: max(s.start, b.start),
                    end: min(s.end, b.end),
                };

                if s.start > inner_range.start {
                    new_seeds.push(std::ops::Range {
                        start: s.start,
                        end: inner_range.start,
                    });
                }

                if inner_range.end > s.end {
                    new_seeds.push(std::ops::Range {
                        start: inner_range.end,
                        end: s.end,
                    });
                }

                new_seeds.push(std::ops::Range {
                    start: inner_range.start + (a.start - b.start),
                    end: inner_range.end + (a.start - b.start),
                });
                break;
            }
        }
        seeds = new_seeds;
    }

    println!(
        "{:?}",
        seeds
            .into_iter()
            .map(|s| s.start)
            .collect::<Vec<i64>>()
            .into_iter()
            .min()
    );
}
