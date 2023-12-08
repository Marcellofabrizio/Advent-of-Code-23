use std::cmp::min;

use counter::Counter;

#[derive(Debug)]
struct Hand {
    cards_rank: Vec<usize>,
    priority: usize,
    bid: u32,
}

pub fn solve1(input: &String) {
    let card_values = "AKQJT98765432";

    let mut hands: Vec<Hand> = input
        .split("\n")
        .into_iter()
        .map(|l| {
            let contents = l.split(" ").collect::<Vec<&str>>();

            let cards_rank = contents[0]
                .chars()
                .into_iter()
                .map(|c| card_values.find(c).unwrap() + 1)
                .collect::<Vec<usize>>();

            let card_count = contents[0]
                .chars()
                .collect::<Counter<char>>()
                .most_common_ordered()
                .into_iter()
                .map(|(_, b)| b)
                .collect::<Vec<_>>();

            return Hand {
                cards_rank: cards_rank,
                priority: get_type_priority(&card_count),
                bid: contents[1].parse::<u32>().unwrap(),
            };
        })
        .collect();

    hands.sort_by(|a, b| {
        if a.priority == b.priority {
            for i in 0..5 {
                if a.cards_rank[i] != b.cards_rank[i] {
                    return b.cards_rank[i].cmp(&a.cards_rank[i]);
                }
            }
        }

        return b.priority.cmp(&a.priority);
    });

    let result = hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1) as u32)
        .reduce(|a, b| a + b)
        .unwrap();

    println!("{}", result);
}

pub fn solve2(input: &String) {
    let card_values = "AKQT98765432J";

    let mut hands: Vec<Hand> = input
        .split("\n")
        .into_iter()
        .map(|l| {
            let contents = l.split(" ").collect::<Vec<&str>>();
            let cards = contents[0].to_owned();

            let cards_rank = cards
                .chars()
                .into_iter()
                .map(|c| card_values.find(c).unwrap() + 1)
                .collect::<Vec<usize>>();

            let mut min_card_type = 7;

            for c in card_values.chars() {
                let card_count = cards
                    .replace("J", c.to_string().as_str())
                    .chars()
                    .collect::<Counter<char>>()
                    .most_common_ordered()
                    .into_iter()
                    .map(|(_, b)| b)
                    .collect::<Vec<_>>();

                min_card_type = min(min_card_type, get_type_priority(&card_count));
            }

            return Hand {
                cards_rank: cards_rank,
                priority: min_card_type,
                bid: contents[1].parse::<u32>().unwrap(),
            };
        })
        .collect();

    hands.sort_by(|a, b| {
        if a.priority == b.priority {
            for i in 0..5 {
                if a.cards_rank[i] != b.cards_rank[i] {
                    return b.cards_rank[i].cmp(&a.cards_rank[i]);
                }
            }
        }

        return b.priority.cmp(&a.priority);
    });

    let result = hands
        .into_iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i + 1) as u32)
        .reduce(|a, b| a + b)
        .unwrap();

    println!("{}", result);
}

pub fn get_type_priority(cards: &Vec<usize>) -> usize {
    match cards[..] {
        [5] => return 1,
        [4, 1] => return 2,
        [3, 2] => return 3,
        [3, 1, 1] => return 4,
        [2, 2, 1] => return 5,
        [2, 1, 1, 1] => return 6,
        _ => 7,
    }
}
