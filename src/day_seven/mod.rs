use std::{cmp::Ordering, collections::HashSet};

pub fn process_day() {
    let file = std::fs::read_to_string("inputs/day-7-large.txt").unwrap();
    let mut hands_played = file
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| {
            let kind = get_hand_kind(hand);
            (hand, bid, kind)
        })
        .collect::<Vec<_>>();

    hands_played.sort_by(|a, b| match a.2.cmp(&b.2) {
        Ordering::Equal => cmp_hands_ranks(a.0, b.0),
        x => x,
    });

    let winnings: usize = hands_played
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            let bid = hand.1.parse::<usize>().unwrap();

            bid * (idx + 1)
        })
        .sum();

    println!("res {:?} {}", hands_played, winnings)
}

static RANKS: &str = "23456789TJQKA";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn get_hand_kind(hand: &str) -> HandKind {
    let set: HashSet<char> = HashSet::from_iter(hand.chars());

    match set.len() {
        1 => HandKind::FiveOfKind,
        2 => {
            let first = set.iter().nth(0).unwrap();
            match hand.chars().filter(|ch| ch == first).count() {
                2 | 3 => HandKind::FullHouse,
                1 | 4 => HandKind::FourOfKind,
                _ => unreachable!("wrong hand"),
            }
        }
        3 => {
            let has_three_of_kind = set.iter().any(|set_ch| {
                let count = hand.chars().filter(|ch| ch == set_ch).count();
                count == 3
            });

            if has_three_of_kind {
                HandKind::ThreeOfKind
            } else {
                HandKind::TwoPair
            }
        }
        4 => HandKind::OnePair,
        _ => HandKind::HighCard,
    }
}

fn cmp_hands_ranks(hand_1: &str, hand_2: &str) -> Ordering {
    hand_1
        .chars()
        .zip(hand_2.chars())
        .find_map(|(a, b)| {
            if a != b {
                Some(RANKS.find(a).unwrap().cmp(&RANKS.find(b).unwrap()))
            } else {
                None
            }
        })
        .unwrap()
}
