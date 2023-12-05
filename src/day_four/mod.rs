use std::collections::HashMap;

pub fn process_day() {
    let file = std::fs::read_to_string("inputs/day-4-small.txt").unwrap();

    let res: Vec<_> = file
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(head, rest)| {
            (
                head.split_once(' ').unwrap().1,
                rest.split_once('|').unwrap(),
            )
        })
        .map(|(id, (winning, mine))| {
            let wining = winning
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap());
            let mine = mine.split_whitespace().map(|x| x.parse::<usize>().unwrap());

            let contained = mine.filter(|m| wining.clone().any(|w| w == *m)).count();

            // println!("{contained}");
            contained
        })
        .collect();

    let mut total_cards: HashMap<usize, usize> = HashMap::new();

    res.iter().enumerate().for_each(|(idx, x)| {
        let end = idx + x;
        let times = *total_cards.get(&idx).unwrap_or(&1);
        // println!("{idx} {x} {times}");

        for _ in 0..times {
            for n in idx..=end {
                println!("{n} {idx} {x}");

                total_cards.entry(n).and_modify(|entry| {
                    *entry += 1;
                });
            }
        }
        // println!("{:?}", total_cards);
    });

    let cards_count: usize = total_cards.values().sum();

    println!("res {:?} {}", res, cards_count);
}
